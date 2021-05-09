#[cfg(test)]
mod tests {

  use crate::ast;
  use crate::parser::*;
  use crate::scanner::*;
  use crate::scope::*;
  use crate::token::*;
  use crate::tree::*;

  fn scope(symbols: Vec<&str>) -> Scope {
    let mut scope = Scope::new();
    for symbol in symbols {
      scope.add(symbol);
    }
    return scope;
  }
  fn parse_statement<'a>(scope: &'a Scope, content: &str) -> ParseResult<'a> {
    let mut scanner = Scanner::new(content);
    let p = Parser::new();
    p.scan_next(&mut scanner)?;
    return p.parse_statement(&mut scanner, scope);
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
    let tree = parse_statement(&mut scope, "x:=42").unwrap();
    assert_matches!(tree.as_ref(), Tree::Node(_));

    let node = Tree::get_node(&tree).unwrap();

    assert_eq!(node.info, NodeInfo::Assignement);
    assert_matches!(node.child.as_ref(), Tree::Node(_));

    let child_node = Tree::get_node(&node.child).unwrap();
    assert_matches!(child_node.info, NodeInfo::Ident(s) if s.name == "x");

    let sibling_node = Tree::get_node(&node.sibling).unwrap();
    assert_matches!(sibling_node.info, NodeInfo::Constant(c) if c == 42);
  }

  fn parse_statement_sequence<'a>(scope: &'a Scope, content: &str) -> ParseResult<'a> {
    let mut scanner = Scanner::new(content);
    let p = Parser::new();
    p.scan_next(&mut scanner)?;
    return p.parse_statement_sequence(&mut scanner, scope);
  }

  #[test]
  fn can_parse_statement_sequence() {
    let mut scope = scope(vec!["x", "y"]);
    let tree = parse_statement_sequence(&mut scope, "x:=42;\ny:=x").unwrap();
    assert_matches!(tree.as_ref(), Tree::Node(_));

    let first_statement = Tree::get_node(&tree).unwrap();
    assert_eq!(first_statement.info, NodeInfo::StatementSequence);
    assert_matches!(first_statement.child.as_ref(), Tree::Node(_));

    let first_assignment = Tree::get_child(&tree).unwrap();
    assert_matches!(Tree::get_node(first_assignment).unwrap().info, NodeInfo::Assignement);

    let second_statement = Tree::get_sibling(&tree).unwrap();
    assert_eq!(Tree::get_node(second_statement).unwrap().info, NodeInfo::StatementSequence);
    let second_assignment = Tree::get_child(&second_statement).unwrap();
    assert_matches!(Tree::get_node(second_assignment).unwrap().info, NodeInfo::Assignement);
  }

  fn parse_factor<'a>(scope: &'a Scope, content: &str) -> ParseResult<'a> {
    let mut scanner = Scanner::new(content);
    let p = Parser::new();
    p.scan_next(&mut scanner)?;
    return p.parse_factor(&mut scanner, scope);
  }

  #[test]
  fn can_parse_factor() {
    let mut scope = scope(vec!["x", "y"]);
    let tree = parse_factor(&mut scope, "42").unwrap();
    assert_matches!(tree.as_ref(), Tree::Node(_));

    let first_statement = Tree::get_node(&tree).unwrap();
    assert_eq!(first_statement.info, NodeInfo::Constant(42));

    let tree = parse_factor(&mut scope, "x").unwrap();
    assert_matches!(tree.as_ref(), Tree::Node(_));

    let first_statement = Tree::get_node(&tree).unwrap();
    assert_matches!(first_statement.info, NodeInfo::Ident(ident) if ident.name == "x");
  }

  fn parse_term<'a>(scope: &'a Scope, content: &str) -> ParseResult<'a> {
    let mut scanner = Scanner::new(content);
    let p = Parser::new();
    p.scan_next(&mut scanner)?;
    return p.parse_term(&mut scanner, scope);
  }

  #[test]
  fn can_parse_term_with_one_level() {
    let mut scope = scope(vec!["x", "y"]);
    let tree = parse_term(&mut scope, "x*42").unwrap();
    assert_matches!(tree.as_ref(), Tree::Node(_));

    let first_statement = Tree::get_node(&tree).unwrap();
    assert_eq!(first_statement.info, NodeInfo::Term(TermOp::Times));

    let child = Tree::get_child_node(&tree).unwrap();
    assert_matches!(child.info, NodeInfo::Ident(ident) if ident.name == "x");

    let sibling = Tree::get_sibling_node(&tree).unwrap();
    assert_eq!(sibling.info, NodeInfo::Constant(42));
  }

  #[test]
  fn can_parse_term_with_multiple_operators() {
    let mut scope = scope(vec!["x", "y"]);

    // NOTE: the tree here is a bit ambiguous, so the user will have to use parentheses.
    let tree = parse_term(&mut scope, "x/42*y").unwrap();

    let first_statement = Tree::get_node(&tree).unwrap();
    assert_eq!(first_statement.info, NodeInfo::Term(TermOp::Times));

    let child_node = Tree::get_child_node(&tree).unwrap();
    assert_matches!(child_node.info, NodeInfo::Term(TermOp::Div));

    let child_child = Tree::get_child(&tree).unwrap();
    let child_child_node = Tree::get_child_node(&child_child).unwrap();
    assert_matches!(child_child_node.info, NodeInfo::Ident(ident) if ident.name == "x");

    let sibling_node = Tree::get_sibling_node(&tree).unwrap();
    assert_matches!(sibling_node.info, NodeInfo::Ident(ident) if ident.name == "y");
  }

  // NOTE(pht) maybe those functions can be automagically created with macros ?
  fn parse_simple_expression<'a>(scope: &'a Scope, content: &str) -> ParseResult<'a> {
    let mut scanner = Scanner::new(content);
    let p = Parser::new();
    p.scan_next(&mut scanner)?;
    return p.parse_simple_expression(&mut scanner, scope);
  }

  #[test]
  fn can_parse_simple_expression_with_one_level() {
    let mut scope = scope(vec!["x", "y"]);
    let tree = parse_simple_expression(&mut scope, "x*y+42").unwrap();
    assert_matches!(tree.as_ref(), Tree::Node(_));

    let first_statement = Tree::get_node(&tree).unwrap();
    assert_eq!(first_statement.info, NodeInfo::SimpleExpression(SimpleExpressionOp::Plus));

    let child = Tree::get_child_node(&tree).unwrap();
    assert_matches!(child.info, NodeInfo::Term(TermOp::Times));

    let sibling = Tree::get_sibling_node(&tree).unwrap();
    assert_eq!(sibling.info, NodeInfo::Constant(42));
  }

  #[test]
  fn can_parse_simple_expression_with_multiple_level() {
    let mut scope = scope(vec!["x", "y"]);
    let tree = parse_simple_expression(&mut scope, "x*y+42*13-12").unwrap();
    assert_matches!(tree.as_ref(), Tree::Node(_));

    let first_statement = Tree::get_node(&tree).unwrap();
    assert_eq!(first_statement.info, NodeInfo::SimpleExpression(SimpleExpressionOp::Minus));

    let child = Tree::get_child_node(&tree).unwrap();
    assert_eq!(child.info, NodeInfo::SimpleExpression(SimpleExpressionOp::Plus));
  }

  #[test]
  fn can_parse_term_with_parens() {
    let mut scope = scope(vec!["x", "y"]);
    let tree = parse_term(&mut scope, "(x*42)").unwrap();
    assert_matches!(tree.as_ref(), Tree::Node(_));

    let first_statement = Tree::get_node(&tree).unwrap();
    assert_eq!(first_statement.info, NodeInfo::Term(TermOp::Times));

    let child = Tree::get_child_node(&tree).unwrap();
    assert_matches!(child.info, NodeInfo::Ident(ident) if ident.name == "x");

    let sibling = Tree::get_sibling_node(&tree).unwrap();
    assert_eq!(sibling.info, NodeInfo::Constant(42));
  }

  fn parse_var_declarations<'a>(scope: &'a mut Scope, content: &str) -> ParseResult<'a> {
    let mut scanner = Scanner::new(content);
    let p = Parser::new();
    // Advance to the "VAR"
    p.scan_next(&mut scanner)?;
    // Consume the "VAR"
    p.scan_next(&mut scanner)?;
    return p.parse_var_declarations(&mut scanner, scope);
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
    assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Declaration);

    let mut root = ast::Path::root();
    let mut path = root.child();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Var);

    path = root.child().child();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "x");

    path = root.child().sibling();
    assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Type(Type::Integer));

    assert_matches!(scope.lookup("x"), Some(Symbol{name, ..}) if name == "x");
  }

  #[test]
  fn can_parse_multiple_var_declaration() {
    let mut scope = Scope::new();
    let tree = parse_var_declarations(&mut scope, "VAR x,y: INTEGER;").unwrap();

    // TODO(pht) this actually returns a Declarations list, and I have to add child level everywhere
    assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Declaration);

    let mut root = ast::Path::root();

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

  // #[test]
  // fn can_parse_multiple_var_declarations() {
  //   let mut scope = Scope::new();
  //   let tree = parse_var_declarations(&mut scope, "VAR x,y: INTEGER; z: INTEGER").unwrap();
  //   assert_matches!(ast::info(&tree).unwrap(), NodeInfo::Declaration);
        
  //   let mut root = ast::Path::root();

  //   let mut path = root.sibling();
  //   assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Declaration);

  //   path = root.sibling().sibling();
  //   assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Declaration);

  //   path = root.sibling().sibling().child();
  //   assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Var);
    
  //   path = root.sibling().sibling().child().child();
  //   assert_matches!(path.follow(&tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "z");
  // }
}
