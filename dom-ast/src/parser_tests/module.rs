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

  fn parse_module(scope: &Scope, content: &str) -> ParseResult {
    let mut scanner = Scanner::new(content);
    parser::scan_next(&mut scanner).unwrap();
    parser::parse_module(&mut scanner, scope)
  }
  
  #[test]
  fn can_not_parse_invalid_module() {
    let mut scope = Scope::new();
    let root_tree = parse_module(&mut scope, "x");
    assert_matches!(root_tree, Err(ParseError::UnexpectedToken(_)));
  }
  
  #[test]
  fn can_not_redefine_module_name() {
    let mut scope = scope(vec!["x"]);
    let root_tree = parse_module(&mut scope, "MODULE x; END x.");
    assert_matches!(root_tree, Err(ParseError::SymbolAlreadyDeclared(s, _)) if s == "x");
  }
  
  #[test]
  fn can_not_parse_module_without_ending() {
    let mut scope = Scope::new();
    let root_tree = parse_module(&mut scope, "MODULE x;");
    assert_matches!(root_tree, Err(ParseError::PrematureEof));
  }
  
  #[test]
  fn can_not_parse_module_with_invalid_ending_name() {
    let mut scope = Scope::new();
    let root_tree = parse_module(&mut scope, "MODULE ModuleName; END OtherModuleName");
    assert_matches!(root_tree, Err(ParseError::UnexpectedBlockEnding{ expected, found}) if expected == "ModuleName" && found == "OtherModuleName");
  }
  
  #[test]
  fn can_not_parse_module_without_ending_period() {
    let mut scope = Scope::new();
    let root_tree = parse_module(&mut scope, "MODULE ModuleName; END ModuleName");
    assert_matches!(root_tree, Err(ParseError::PrematureEof));
  }
  
  #[test]
  fn can_not_parse_module_with_anything_after_the_period() {
    let mut scope = Scope::new();
    let root_tree = parse_module(&mut scope, "MODULE ModuleName; END ModuleName. 42");
    assert_matches!(root_tree, Err(ParseError::UnexpectedToken(_)));
  }
  
  #[test]
  fn can_parse_module_without_declarations_or_body() {
    let mut scope = Scope::new();
    let root_tree = parse_module(&mut scope, "MODULE ModuleName; END ModuleName.").unwrap();
  
    let mut root = ast::Path::root();
    assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::Module);
  
    let path = root.child();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "ModuleName");
  
    let path = root.sibling();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Declarations);
  
    assert!(ast::is_empty(ast::child(ast::sibling(&root_tree).unwrap()).unwrap()));
  }
  
  #[test]
  fn can_parse_module_without_body() {
    let mut scope = Scope::new();
    let root_tree = parse_module(&mut scope, "MODULE ModuleName; VAR x: INTEGER; y: INTEGER; END ModuleName.").unwrap();
  
    let mut root = ast::Path::root();
    assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::Module);
  
    println!("{:?}", root_tree);
  
    let path = root.child();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "ModuleName");
  
    let path = root.sibling();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Declarations);
  
    let path = root.sibling().child();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Declaration);
  }
  
  #[test]
  fn can_parse_module_wit_declarations_and_body() {
    let mut scope = Scope::new();
    let root_tree = parse_module(&mut scope, "MODULE ModuleName; VAR x,y: INTEGER; z: INTEGER; BEGIN x:= 1; y:= 2 END ModuleName.").unwrap();
  
    let mut root = ast::Path::root();
    assert_matches!(root.follow(&root_tree).unwrap(), NodeInfo::Module);
  
    let path = root.child();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Ident(ident) if ident.name == "ModuleName");
  
    let path = root.sibling();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Declarations);
  
    let path = root.sibling().child();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Declaration);
  
    let path = root.sibling().child().sibling();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Declaration);
  
    let path = root.sibling().sibling();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::StatementSequence);
  
    let path = root.sibling().sibling().child();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Assignement);
  
    let path = root.sibling().sibling().sibling();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::StatementSequence);
  
    let path = root.sibling().sibling().sibling().child();
    assert_matches!(path.follow(&root_tree).unwrap(), NodeInfo::Assignement);
  }
  

}
