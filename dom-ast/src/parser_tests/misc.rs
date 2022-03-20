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

  fn scope(symbols: Vec<&str>) -> Scope {
    let scope = Scope::new();
    for symbol in symbols {
      scope.add(symbol);
    }
    return scope;
  }
  fn parse_statement<'a>(scope: &'a Scope, content: &str) -> ParseResult {
    let mut scanner = Scanner::new(content);
    parser::scan_next(&mut scanner)?;
    return parser::parse_statement(&mut scanner, scope);
  }

  #[test]
  fn fails_on_premature_eof() {
    let mut scope = scope(vec!["x"]);
    for content in vec!["", "x", "x:="] {
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
    return parser::parse_statement_sequence(&mut scanner, scope);
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
    return parser::parse_factor(&mut scanner, scope);
  }

  #[test]
  fn can_parse_factor() {
    let mut scope = scope(vec!["x", "y"]);
    let tree = parse_factor(&mut scope, "42").unwrap();
    assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Constant(42));

    let tree = parse_factor(&mut scope, "x").unwrap();
    assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");
  }

  
  fn parse_term<'a>(scope: &'a Scope, content: &str) -> ParseResult {
    let mut scanner = Scanner::new(content);
    parser::scan_next(&mut scanner)?;
    return parser::parse_term(&mut scanner, scope);
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
    return parser::parse_simple_expression(&mut scanner, scope);
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
  
  fn finish_parsing(_scanner: &mut Scanner, _scope: &Scope) -> ParseResult {
    return Ok(ast::empty());
  }

  fn parse_var_declarations<'a>(scope: &'a mut Scope, content: &str) -> ParseResult {
    let mut scanner = Scanner::new(content);
    // Advance to the "VAR"
    parser::scan_next(&mut scanner)?;
    // Consume the "VAR"
    parser::scan_next(&mut scanner)?;
    return parser::parse_var_declarations(&mut scanner, scope, &mut finish_parsing);
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
    assert_matches!(root.follow(&child_tree).unwrap(), NodeInfo::Declaration);

    let mut path = root.child();
    assert_matches!(path.follow(&child_tree).unwrap(), NodeInfo::Var);

    path = root.child().child();
    assert_matches!(path.follow(&child_tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");

    path = root.child().sibling();
    assert_matches!(path.follow(&child_tree).unwrap(), NodeInfo::Type(Type::Integer));

    assert_matches!(scope.lookup("x").unwrap().as_ref(), Symbol{name, ..} if name == "x");
  }

  
  #[test]
  fn can_parse_multiple_var_declaration() {
    let mut scope = Scope::new();
    let root_tree = parse_var_declarations(&mut scope, "VAR x,y: INTEGER;").unwrap();

    assert_matches!(ast::info(&root_tree).unwrap(), NodeInfo::Declarations);

    let tree = ast::child(&root_tree).unwrap();
    let mut root = ast::Path::root();
    assert_matches!(root.follow(&tree).unwrap(), NodeInfo::Declaration);

    let mut path = root.child();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Var);

    path = root.child().child();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");

    path = root.child().sibling();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Type(Type::Integer));

    path = root.sibling();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Declaration);

    path = root.sibling().child();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Var);

    path = root.sibling().child().child();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "y");

    path = root.sibling().child().sibling();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Type(Type::Integer));
  }
  
  #[test]
  fn can_parse_multiple_var_declarations() {
    let mut scope = Scope::new();

    let root_tree = parse_var_declarations(&mut scope, "VAR x,y: INTEGER; z: INTEGER;").unwrap();
    assert_matches!(ast::info(&root_tree).unwrap(), NodeInfo::Declarations);

    let tree = ast::child(&root_tree).unwrap();

    let mut root = ast::Path::root();
    assert_matches!(root.follow(&tree).unwrap(), NodeInfo::Declaration);

    let mut path = root.sibling();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Declaration);

    path = root.sibling().sibling();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Declaration);

    path = root.sibling().sibling().child();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Var);
    
    path = root.sibling().sibling().child().child();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "z");
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

}
