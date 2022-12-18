#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

    use crate::ast;
    use crate::parser;
    use crate::parser::*;
    use crate::scanner::*;
    use crate::scope::*;
    use crate::tree::*;

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
    fn fails_on_premature_eof() {
        let mut scope = scope(vec!["x"]);
        for content in &["", "x", "x:="] {
            let tree = parse_statement(&mut scope, content);
            assert_matches!(tree.unwrap_err(), ParseError::PrematureEof, "Expected PrematureEof while parsing {}", content);
        }
    }

    #[test]
    fn fails_on_scan_eof() {
        let mut scope = Scope::new();
        let tree = parse_statement(&mut scope, " ❤");
        assert_matches!(tree.unwrap_err(), ParseError::ScanError(_));
    }

    #[test]
    fn fails_parsing_statement_for_unknown_identifier() {
        let mut scope = Scope::new();

        let tree = parse_statement(&mut scope, "y:=42");
        assert_matches!(tree.unwrap_err(), ParseError::UndefinedSymbol(s) if s == "y");
    }

    #[test]
    fn can_parse_statement() {
        let mut scope = scope(vec!["x"]);
        let root_tree = parse_statement(&mut scope, "x:=42").unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::Assignement);

        let path = root.child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Ident(s) if s.name == "x");

        let path = root.sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Constant(42));
    }

    fn parse_statement_sequence<'a>(scope: &'a Scope, content: &str) -> ParseResult {
        let mut scanner = Scanner::new(content);
        parser::scan_next(&mut scanner)?;
        parser::parse_statement_sequence(&mut scanner, scope)
    }

    #[test]
    fn can_parse_statement_sequence() {
        let mut scope = scope(vec!["x", "y"]);
        let root_tree = parse_statement_sequence(&mut scope, "x:=42;\ny:=x").unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::StatementSequence);

        let path = root.child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Assignement);

        let path = root.sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::StatementSequence);

        let path = root.sibling().child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Assignement);
    }

    fn parse_factor<'a>(scope: &'a Scope, content: &str) -> ParseResult {
        let mut scanner = Scanner::new(content);
        parser::scan_next(&mut scanner)?;
        parser::parse_factor(&mut scanner, scope)
    }

    #[test]
    fn can_parse_factor() {
        let mut scope = scope(vec!["x", "y"]);
        let tree = parse_factor(&mut scope, "42").unwrap();
        assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Constant(42));

        let tree = parse_factor(&mut scope, "x").unwrap();
        assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");
    }

    #[test]
    fn can_parse_factor_with_constant_selector() {
        let mut scope = scope(vec!["x"]);

        let tree = parse_factor(&mut scope, "x[0]").unwrap();
        assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");

        let mut root = ast::Path::root();

        let path = root.child();
        assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Constant(index) if *index == 0);
    }

    #[test]
    fn can_parse_factor_with_variable_selector() {
        let mut scope = scope(vec!["x", "i"]);

        let tree = parse_factor(&mut scope, "x[i]").unwrap();
        assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");

        let mut root = ast::Path::root();

        let path = root.child();
        assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "i");
    }

    fn parse_term<'a>(scope: &'a Scope, content: &str) -> ParseResult {
        let mut scanner = Scanner::new(content);
        parser::scan_next(&mut scanner)?;
        parser::parse_term(&mut scanner, scope)
    }

    #[test]
    fn can_parse_term_with_one_level() {
        let mut scope = scope(vec!["x", "y"]);
        let root_tree = parse_term(&mut scope, "x*42").unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::Term(TermOp::Times));

        let path = root.child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");

        let path = root.sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Constant(42));
    }

    #[test]
    fn can_parse_term_with_multiple_operators() {
        let mut scope = scope(vec!["x", "y"]);

        // NOTE: the tree here is a bit ambiguous, so the user will have to use parentheses.
        let root_tree = parse_term(&mut scope, "x/42*y").unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::Term(TermOp::Times));

        let path = root.child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Term(TermOp::Div));

        let path = root.child().child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");

        let path = root.child().sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Constant(42));

        let path = root.sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "y");
    }

    // NOTE(pht) maybe those functions can be automagically created with macros ?
    fn parse_simple_expression<'a>(scope: &'a Scope, content: &str) -> ParseResult {
        let mut scanner = Scanner::new(content);
        parser::scan_next(&mut scanner)?;
        parser::parse_simple_expression(&mut scanner, scope)
    }

    #[test]
    fn can_parse_simple_expression_with_one_level() {
        let mut scope = scope(vec!["x", "y"]);
        let root_tree = parse_simple_expression(&mut scope, "x*y+42").unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::SimpleExpression(SimpleExpressionOp::Plus));

        let path = root.child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Term(TermOp::Times));

        let path = root.sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Constant(42));
    }

    #[test]
    fn can_parse_simple_expression_with_multiple_level() {
        let mut scope = scope(vec!["x", "y"]);
        let root_tree = parse_simple_expression(&mut scope, "x*y+42*13-12").unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::SimpleExpression(SimpleExpressionOp::Minus));

        let path = root.child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::SimpleExpression(SimpleExpressionOp::Plus));
    }

    #[test]
    fn can_parse_term_with_parens() {
        let mut scope = scope(vec!["x", "y"]);
        let root_tree = parse_term(&mut scope, "(x*42)").unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::Term(TermOp::Times));

        let path = root.child();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");

        let path = root.sibling();
        assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Constant(42));
    }
}
