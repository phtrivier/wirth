
#[cfg(test)]
mod tests {

  use crate::scope::*;
  use crate::scanner::*;
  use crate::tree::*;
  use crate::parser::*;

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

}
