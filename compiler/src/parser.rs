use crate::scanner::*;
use crate::scope::*;
use crate::token::*;
use crate::ast::*;
use std::rc::Rc;

// TODO(pht) all parseerror should probably get the corresponding ScanContext
#[derive(Debug)]
pub enum ParseError {
  ScanError(crate::token::ScanError),
  UndefinedSymbol(String),
  PrematureEof,
  UnexpectedToken(ScanContext),
  Todo,
}

pub type ParseResult<'a> = Result<Rc<Tree<'a>>, ParseError>;

pub struct Parser {}

impl Parser {
  pub fn new() -> Parser {
    Parser {}
  }

  pub fn parse_statement_sequence<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {

    let first_statement = self.parse_statement(scanner, scope)?;
    let next = self.scan_next(scanner);
    if let Ok(Scan { token : Token::Semicolon, context: _context}) = next {
      return Ok(Rc::new(Tree::Node(Node {
        info: NodeInfo::StatementSequence,
        child: first_statement,
        sibling: self.parse_statement_sequence(scanner, scope)?,
      })));
    } else {
      return Ok(Rc::new(Tree::Node(Node {
        info: NodeInfo::StatementSequence,
        child: first_statement,
        sibling: Rc::new(Tree::Nil),
      })));
    }

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
 
  fn parse_assignment<'a>(&self, subject: Rc<Tree<'a>>, context: ScanContext, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {

    let object = self.parse_expression(context, scanner, scope)?;

    return Ok(Rc::new(Tree::Node(Node {
      info: NodeInfo::Assignement,
      child: subject,
      sibling: object,
    })));

  }

  fn parse_expression<'a>(&self, context: ScanContext, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    // NOTE(pht) at the moment, expression := ident | integer...
    let next = self.scan_next(scanner)?;
    if let Scan{token: Token::Int(constant_value),
      context: _context,
    } = next {
      return Ok(Rc::new(Tree::Node(Node::constant(constant_value))));
    }

    if let Scan{token: Token::Ident(ident),
      context: _context,
    } = next {
      let symbol = self.lookup(scope, &ident)?;
      return Ok(Rc::new(Tree::Node(Node::ident(symbol))));
    }
    
    return Err(ParseError::UnexpectedToken(context));
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
