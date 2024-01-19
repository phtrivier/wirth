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

    fn parse_statement(scope: &Scope, content: &str) -> ParseResult {
        let mut scanner = Scanner::new(content);
        parser::scan_next(&mut scanner)?;
        parser::parse_statement(&mut scanner, scope)
    }

    #[test]
    fn fails_on_invalid_while_expressions() {
        let mut scope = scope(vec!["x"]);
        let mut error = parse_statement(&mut scope, "WHILE").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);

        error = parse_statement(&mut scope, "WHILE 0").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);

        error = parse_statement(&mut scope, "WHILE 0 = 1").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);

        error = parse_statement(&mut scope, "WHILE 0 = 1 DO x:= 1; x:= 2").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
    }

    #[test]
    fn can_parse_complete_while_expression() {
        let mut scope = scope(vec!["x"]);
        let root_tree = parse_statement(&mut scope, "WHILE 0 = 1 DO x:= 1; x:= 2 END").unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::WhileStatement);

        let path = root.child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Expression(ExpressionOp::Eql));

        let path = root.sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Do);

        let path = root.sibling().child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::StatementSequence);
    }
}
