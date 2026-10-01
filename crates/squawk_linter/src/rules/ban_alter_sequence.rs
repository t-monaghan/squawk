use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_alter_sequence(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterSequence(seq) = stmt {
            for action in seq.actions() {
                if let ast::AlterSequenceAction::SequenceOption(option) = action {
                    if matches!(
                        option,
                        ast::SequenceOption::OptionRestart(_)
                            | ast::SequenceOption::OptionMinValue(_)
                            | ast::SequenceOption::OptionNoMinValue(_)
                            | ast::SequenceOption::OptionMaxValue(_)
                            | ast::SequenceOption::OptionNoMaxValue(_)
                            | ast::SequenceOption::OptionAsType(_)
                            | ast::SequenceOption::OptionIncrement(_)
                    ) {
                        ctx.report(Violation::for_node(Rule::BanAlterSequence, "Altering a sequence may silently change values used by existing clients.".into(), option.syntax()));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        Rule,
        test_utils::{lint_errors, lint_ok},
    };
    use insta::assert_snapshot;
    #[test]
    fn err() {
        let sql = "ALTER SEQUENCE s RESTART WITH 10; ALTER SEQUENCE s MINVALUE 2; ALTER SEQUENCE s MAXVALUE 99; ALTER SEQUENCE s AS bigint; ALTER SEQUENCE s INCREMENT BY 2;";
        let errors = lint_errors(sql, Rule::BanAlterSequence);
        assert_eq!(errors.matches("warning[ban-alter-sequence]").count(), 5);
        assert_snapshot!(errors);
    }
    #[test]
    fn ok() {
        lint_ok("ALTER SEQUENCE s CACHE 2;", Rule::BanAlterSequence);
    }
}
