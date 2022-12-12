#[cfg(test)]
mod tests {

    use std::assert_matches::assert_matches;

    use crate::ast;
    use crate::parser;
    use crate::parser::*;
    use crate::scanner::*;
    use crate::scope::*;
    use crate::tree::*;

    // NOTE(pht) move some of those to utils ?
    fn scope(symbols: Vec<&str>) -> Scope {
        let scope = Scope::new();
        for symbol in symbols {
            scope.add(symbol);
        }
        scope
    }

    fn parse_statement<'a>(scope: &'a Scope, content: &str) -> ParseResult {
        let mut scanner = Scanner::new(content);
        parser::scan_next(&mut scanner)?;
        parser::parse_statement(&mut scanner, scope)
    }

    #[test]
    fn fails_on_if_expression_without_condition() {
        let mut scope = Scope::new();
        let error = parse_statement(&mut scope, "IF").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
    }

    #[test]
    fn fails_on_invalid_if_expression() {
        let mut scope = Scope::new();
        let error = parse_statement(&mut scope, "IF 0 =").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
    }

    #[test]
    fn fails_on_if_expression_without_then() {
        let mut scope = Scope::new();
        let error = parse_statement(&mut scope, "IF 0 = 1").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
    }

    #[test]
    fn fails_on_if_expression_without_then_statements() {
        let mut scope = Scope::new();
        let error = parse_statement(&mut scope, "IF 0 = 1 THEN").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
    }

    #[test]
    fn fails_on_if_expression_without_then_statements_ending() {
        let mut scope = scope(vec!["x"]);
        let error = parse_statement(&mut scope, "IF 0 = 1 THEN x:= 1; x:= 2").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
    }

    #[test]
    fn can_parse_complete_if_expression() {
        let mut scope = scope(vec!["x"]);
        let root_tree = parse_statement(&mut scope, "IF 0 = 1 THEN x:= 1; x:= 2 END").unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::IfStatement);

        let path = root.child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Expression(ExpressionOp::Eql));

        let path = root.child().child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Constant(0));

        let path = root.child().sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Constant(1));

        let path = root.sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Then);

        let path = root.sibling().child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::StatementSequence);

        let path = root.sibling().child().sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::StatementSequence);
    }

    #[test]
    fn fails_on_if_expression_with_incomplete_else() {
        let mut scope = scope(vec!["x"]);
        let error = parse_statement(&mut scope, "IF 0 = 1 THEN x:= 1; x:= 2 ELSE").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
    }

    #[test]
    fn fails_on_if_expression_with_incomplete_else_statements() {
        let mut scope = scope(vec!["x"]);
        let error = parse_statement(&mut scope, "IF 0 = 1 THEN x:= 1; x:= 2 ELSE x:=3").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
    }

    #[test]
    fn can_parse_complete_if_then_else_expression() {
        let mut scope = scope(vec!["x"]);
        let root_tree = parse_statement(&mut scope, "IF 0 = 1 THEN x:= 1 ELSE x:=2 END").unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::IfStatement);

        let path = root.child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Expression(ExpressionOp::Eql));

        let path = root.child().child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Constant(0));

        let path = root.child().sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Constant(1));

        let path = root.sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Then);

        let path = root.sibling().child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::StatementSequence);

        let path = root.sibling().sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Else);

        let path = root.sibling().sibling().child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::StatementSequence);

        let path = root.sibling().sibling().child().child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Assignement);
    }

    #[test]
    fn can_parse_nested_if_then_else_expression() {
        let mut scope = scope(vec!["x"]);
        let root_tree = parse_statement(
            &mut scope,
            "
          IF 0 = 0 THEN
             IF 1 = 0 THEN
               x:= 1
             ELSE
               x:=2
             END
          ELSE
             x:= 3
          END",
        )
        .unwrap();

        ast::print(&root_tree);

        // assert!(false);
    }

    #[test]
    fn wtf_wtf_wt_fwt_f() {
        let mut scope = scope(vec!["x"]);
        let root_tree = parse_statement(
            &mut scope,
        "IF 0 = 1 THEN
        x:= 1
      ELSE
        IF 0 = 0 THEN
           x := 2;
           IF 0 = 0 THEN
             x := 3
           END
        ELSE
           x := 4
        END
      END").unwrap();

        ast::print(&root_tree);

        // assert!(false);
    }
}
