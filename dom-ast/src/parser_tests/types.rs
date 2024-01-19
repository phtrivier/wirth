#[cfg(test)]
mod tests {

    use std::assert_matches::assert_matches;

    use crate::ast;
    use crate::parser;
    use crate::parser::*;
    use crate::scanner::*;
    use crate::scope::*;
    use crate::token::*;
    use crate::tree::*;

    // NOTE(pht) move some of those to utils ?
    fn scope(symbols: Vec<&str>) -> Scope {
        let scope = Scope::new();
        for symbol in symbols {
            scope.add(symbol);
        }
        scope
    }

    fn parse_var_declarations(scope: &mut Scope, content: &str) -> ParseResult {
        let mut scanner = Scanner::new(content);
        // Advance to the "VAR"
        parser::scan_next(&mut scanner)?;
        // Consume the "VAR"
        parser::scan_next(&mut scanner)?;
        parser::parse_var_declarations(&mut scanner, scope, &mut finish_parsing)
    }

    #[test]
    fn fails_on_var_redeclaration() {
        let mut scope = Scope::new();
        scope.add("x");
        let tree = parse_var_declarations(&mut scope, "VAR x: INTEGER;");
        assert_matches!(tree, Err(ParseError::SymbolAlreadyDeclared(ident, ScanContext{ line: 0, column: 4})) if ident == "x");
    }

    #[test]
    fn can_parse_single_var_declaration() {
        let mut scope = Scope::new();
        let tree = parse_var_declarations(&mut scope, "VAR x: INTEGER;").unwrap();
        assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Declarations);

        let child_tree = ast::child(&tree).unwrap();
        let mut root = ast::Path::root();
        assert_matches!(root.follow(child_tree).unwrap(), NodeInfo::Declaration);

        let mut path = root.child();
        assert_matches!(path.follow(child_tree).unwrap(), NodeInfo::Var);

        path = root.child().child();
        assert_matches!(path.follow(child_tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");

        path = root.child().sibling();
        assert_matches!(path.follow(child_tree).unwrap(), NodeInfo::Type(VarType::Integer));

        assert_matches!(scope.lookup("x").unwrap().as_ref(), Symbol{name, ..} if name == "x");
    }

    #[test]
    fn can_parse_multiple_var_declaration() {
        let mut scope = Scope::new();
        let root_tree = parse_var_declarations(&mut scope, "VAR x,y: INTEGER;").unwrap();

        assert_matches!(ast::info(&root_tree).unwrap(), NodeInfo::Declarations);

        let tree = ast::child(&root_tree).unwrap();
        let mut root = ast::Path::root();
        assert_matches!(root.follow(tree).unwrap(), NodeInfo::Declaration);

        let mut path = root.child();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Var);

        path = root.child().child();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");

        path = root.child().sibling();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Type(VarType::Integer));

        path = root.sibling();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Declaration);

        path = root.sibling().child();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Var);

        path = root.sibling().child().child();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "y");

        path = root.sibling().child().sibling();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Type(VarType::Integer));
    }

    #[test]
    fn can_parse_multiple_var_declarations() {
        let mut scope = Scope::new();

        let root_tree = parse_var_declarations(&mut scope, "VAR x,y: INTEGER; z: INTEGER;").unwrap();
        assert_matches!(ast::info(&root_tree).unwrap(), NodeInfo::Declarations);

        let tree = ast::child(&root_tree).unwrap();

        let mut root = ast::Path::root();
        assert_matches!(root.follow(tree).unwrap(), NodeInfo::Declaration);

        let mut path = root.sibling();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Declaration);

        path = root.sibling().sibling();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Declaration);

        path = root.sibling().sibling().child();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Var);

        path = root.sibling().sibling().child().child();
        assert_matches!(path.follow(tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "z");
    }

    #[test]
    fn can_parse_empty_declarations() {
        let mut scope = Scope::new();
        let mut scanner = Scanner::new("");

        let root_tree = parser::parse_declarations(&mut scanner, &mut scope, &mut finish_parsing).unwrap();
        assert_matches!(ast::info(&root_tree).unwrap(), NodeInfo::Declarations);

        assert!(ast::is_empty(ast::child(&root_tree).unwrap()));
    }

    #[test]
    fn can_parse_declarations() {
        let mut scope = Scope::new();
        let mut scanner = Scanner::new("VAR x,y: INTEGER;");
        parser::scan_next(&mut scanner).unwrap();
        let root_tree = parser::parse_declarations(&mut scanner, &mut scope, &mut finish_parsing).unwrap();
        assert_matches!(ast::info(&root_tree).unwrap(), NodeInfo::Declarations);
        assert!(!ast::is_empty(ast::child(&root_tree).unwrap()));
    }

    #[test]
    fn fails_on_incomplete_array_declaration() {
        let mut scope = scope(vec![]);
        let mut error = parse_var_declarations(&mut scope, "VAR a: ARRAY").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
        error = parse_var_declarations(&mut scope, "VAR a: ARRAY 4").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
        error = parse_var_declarations(&mut scope, "VAR a: ARRAY 4 OF").unwrap_err();
        assert_matches!(error, ParseError::PrematureEof);
    }

    #[test]
    fn can_parse_single_array_declaration() {
        let mut scope = scope(vec![]);
        let tree = parse_var_declarations(&mut scope, "VAR a: ARRAY 4 OF INTEGER;").unwrap();

        assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Declarations);

        let child_tree = ast::child(&tree).unwrap();
        let mut root = ast::Path::root();
        assert_matches!(root.follow(child_tree).unwrap(), NodeInfo::Declaration);

        let mut path = root.child();
        assert_matches!(path.follow(child_tree).unwrap(), NodeInfo::Var);

        path = root.child().child();
        assert_matches!(path.follow(child_tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "a");

        path = root.child().sibling();
        assert_matches!(path.follow(child_tree).unwrap(), NodeInfo::Type(VarType::Array(capacity)) if *capacity == 4);

        assert_matches!(scope.lookup("a").unwrap().as_ref(), Symbol{name, size, ..} if name == "a" && *size == 4);
    }

    fn finish_parsing(_scanner: &mut Scanner, _scope: &Scope) -> ParseResult {
        Ok(ast::empty())
    }
}
