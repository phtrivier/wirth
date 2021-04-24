use crate::scanner::*;
use crate::scope::*;
use crate::token::*;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum NodeInfo<'a> {
  Assignement,
  Constant(u32),
  Ident(&'a Symbol), // NOTE(pht) I wonder if this could be done with a &Symbol, with the appropriate lifetime ?
}

#[derive(Debug)]
struct Node<'a> {
  info: NodeInfo<'a>,
  child: Rc<Tree<'a>>, // NOTE(pht) I wonder if those could be either Boxes. Or, If I don't want to allocate memory, a reference to a vec ?
  sibling: Rc<Tree<'a>>,
}

impl Node<'_> {
  pub fn ident<'a>(symbol: &'a Symbol) -> Node<'a> {
    Node {
      info: NodeInfo::Ident(symbol),
      child: Rc::new(Tree::Nil),
      sibling: Rc::new(Tree::Nil),
    }
  }

  pub fn constant<'a>(c: u32) -> Node<'a> {
    Node {
      info: NodeInfo::Constant(c),
      child: Rc::new(Tree::Nil),
      sibling: Rc::new(Tree::Nil),
    }
  }
}

#[derive(Debug)]
enum Tree<'a> {
  Node(Node<'a>),
  Nil,
}

// TODO(pht) all parseerror should probably get the corresponding ScanContext
#[derive(Debug)]
enum ParseError {
  ScanError(crate::token::ScanError),
  UndefinedSymbol(String),
  PrematureEof,
  UnexpectedToken(ScanContext),
  Todo,
}

type ParseResult<'a> = Result<Rc<Tree<'a>>, ParseError>;

struct Parser {}

impl Parser {
  pub fn new() -> Parser {
    Parser {}
  }

  pub fn parse_statement<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    let next = self.scan_next(scanner)?;
    if let Scan {
      token: Token::Ident(ident),
      context,
    } = next
    {
      let ident_symbol = self.lookup(scope, &ident)?;
      let subject = Rc::new(Tree::Node(Node::ident(ident_symbol))); // NOTE(pht) this subject is either the ident, or a selector tree from the ident

      let next = self.scan_next(scanner)?;
      if let Scan {
        token: Token::Becomes,
        context,
      } = next
      {
        return self.parse_assignment(subject, context, scanner, scope);
      }
      return Err(ParseError::UnexpectedToken(context));
    }
    return Err(ParseError::Todo); // If statement, etc... 

  }
 
  // NOTE(pht) the first argument of parse_assignemnt should not be the ident_symbol, but a tree 
  fn parse_assignment<'a>(&self, subject: Rc<Tree<'a>>, context: ScanContext, scanner: &mut Scanner, _scope: &'a Scope) -> ParseResult<'a> {
    let next = self.scan_next(scanner)?;
    if let Scan {
      token: Token::Int(constant_value),
      context: _context,
    } = next
    {
      let sibling = Rc::new(Tree::Node(Node::constant(constant_value)));

      return Ok(Rc::new(Tree::Node(Node {
        info: NodeInfo::Assignement,
        child: subject,
        sibling,
      })));
    } else {
      Err(ParseError::UnexpectedToken(context))
    }
  }

  fn scan_next(&self, scanner: &mut Scanner) -> Result<Scan, ParseError> {
    let next_scan_result = scanner.next().ok_or(ParseError::PrematureEof)?;
    match next_scan_result {
      Ok(scan) => Ok(scan),
      Err(scan_error) => Err(ParseError::ScanError(scan_error)),
    }
  }

  fn lookup<'a>(&self, scope: &'a Scope, ident: &str) -> Result<&'a Symbol, ParseError> {
    return scope.lookup(&ident).ok_or_else(|| ParseError::UndefinedSymbol(String::from(ident)));
  }

}

#[cfg(test)]
mod tests {

  use super::*;

  fn scope(symbols: Vec<&str>) -> Scope {
    let mut scope = Scope::new();
    for symbol in symbols {
      scope.add(symbol);
    }
    return scope;
  }

  // Convenience method to allow exctracting the Node from a tree.
  // I don't know if I should use it except in tests ?
  fn tree_node<'a>(tree: &'a Tree) -> Option<&'a Node<'a>> {
    match tree {
      Tree::Node(node) => Some(node),
      Tree::Nil => None,
    }
  }

  fn parse_statement<'a>(scope: &'a Scope, content: &str) -> ParseResult<'a> {
    let mut scanner = Scanner::new(content);
    let p = Parser::new();
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
  fn fails_parsing_statement_for_unknown_identifier() {
    let mut scope = Scope::new();

    let tree = parse_statement(&mut scope, "y:=42");
    assert_matches!(tree.unwrap_err(), ParseError::UndefinedSymbol(s) if s == "y");
  }

  #[test]
  fn parses_statement() {
    let mut scope = scope(vec!["x"]);
    let tree = parse_statement(&mut scope, "x:=42").unwrap();
    assert_matches!(tree.as_ref(), Tree::Node(_));

    let node = tree_node(tree.as_ref()).unwrap();
    assert_eq!(node.info, NodeInfo::Assignement);
    assert_matches!(node.child.as_ref(), Tree::Node(_));

    let child_node = tree_node(node.child.as_ref()).unwrap();
    assert_matches!(child_node.info, NodeInfo::Ident(s) if s.name == "x");

    let sibling_node = tree_node(node.sibling.as_ref()).unwrap();
    assert_matches!(sibling_node.info, NodeInfo::Constant(c) if c == 42);
  }

  #[test]
  fn parse_statement_sequence() {
    let mut scope = scope(vec!["x", "y"]);
    let _tree = parse_statement(&mut scope, "x:=42;y:=0").unwrap();
    
  }

}
