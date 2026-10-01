use crate::{Linter, Rule, Violation};
use squawk_syntax::{
    Parse, SourceFile,
    ast::{self, AstNode},
};

pub(crate) fn ban_refresh_materialized_view_non_concurrent(
    ctx: &mut Linter,
    parse: &Parse<SourceFile>,
) {
    for stmt in parse.tree().stmts() {
        if let ast::Stmt::Refresh(node) = stmt {
            if node.concurrently_token().is_none() {
                ctx.report(Violation::for_node(
                    Rule::BanRefreshMaterializedViewNonConcurrent,
                    "Refreshing a materialized view without CONCURRENTLY blocks readers.".into(),
                    node.syntax(),
                ));
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
        let sql = "REFRESH MATERIALIZED VIEW mv;";
        assert_snapshot!(lint_errors(
            sql,
            Rule::BanRefreshMaterializedViewNonConcurrent
        ));
    }
    #[test]
    fn ok() {
        lint_ok(
            "REFRESH MATERIALIZED VIEW CONCURRENTLY mv;",
            Rule::BanRefreshMaterializedViewNonConcurrent,
        );
    }
}
