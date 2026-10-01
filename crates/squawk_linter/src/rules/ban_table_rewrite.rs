use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_table_rewrite(ctx: &mut Linter, parse: &Parse<SourceFile>) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::AlterTable(table) = stmt {
            for action in table.actions() {
                match action {
                    ast::AlterTableAction::SetLogged(node) => {
                        ctx.report(Violation::for_node(Rule::BanTableRewrite, "Rewriting a table requires an `ACCESS EXCLUSIVE` lock that blocks reads and writes.".into(), node.syntax()));
                    }
                    ast::AlterTableAction::SetUnlogged(node) => {
                        ctx.report(Violation::for_node(Rule::BanTableRewrite, "Rewriting a table requires an `ACCESS EXCLUSIVE` lock that blocks reads and writes.".into(), node.syntax()));
                    }
                    ast::AlterTableAction::SetTablespace(node) => {
                        ctx.report(Violation::for_node(Rule::BanTableRewrite, "Rewriting a table requires an `ACCESS EXCLUSIVE` lock that blocks reads and writes.".into(), node.syntax()));
                    }
                    ast::AlterTableAction::SetWithoutOids(node) => {
                        ctx.report(Violation::for_node(Rule::BanTableRewrite, "Rewriting a table requires an `ACCESS EXCLUSIVE` lock that blocks reads and writes.".into(), node.syntax()));
                    }
                    ast::AlterTableAction::AlterColumn(column) => {
                        if let Some(ast::AlterColumnOption::SetStorage(node)) = column.option() {
                            ctx.report(Violation::for_node(Rule::BanTableRewrite, "Rewriting a table requires an `ACCESS EXCLUSIVE` lock that blocks reads and writes.".into(), node.syntax()));
                        }
                    }
                    _ => (),
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
        let sql = "ALTER TABLE t SET LOGGED; ALTER TABLE t SET UNLOGGED; ALTER TABLE t SET TABLESPACE ts; ALTER TABLE t SET WITHOUT OIDS; ALTER TABLE t ALTER COLUMN c SET STORAGE PLAIN;";
        let errors = lint_errors(sql, Rule::BanTableRewrite);
        assert_eq!(errors.matches("warning[ban-table-rewrite]").count(), 5);
        assert_snapshot!(errors);
    }
    #[test]
    fn ok() {
        lint_ok(
            "ALTER TABLE t ALTER COLUMN c SET DEFAULT 1;",
            Rule::BanTableRewrite,
        );
    }
}
