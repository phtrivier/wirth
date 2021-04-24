use crate::scanner::*;
use crate::scope::*;
use crate::token::*;
use std::rc::Rc;


#[derive(Debug, PartialEq)]
enum NodeInfo<'a> {
  Assignement,
  Constant(u32),
  Ident(&'a Symbol) // NOTE(pht) I wonder if this could be done with a &Symbol, with the appropriate lifetime ?
}

#[derive(Debug)]
struct Node<'a>{
  info: NodeInfo<'a>,
  child: Rc<Tree<'a>>, // NOTE(pht) I wonder if those could be either Boxes. Or, If I don't want to allocate memory, a reference to a vec ?
  sibling: Rc<Tree<'a>>
}

impl Node<'_> {
  pub fn ident<'a>(symbol: &'a Symbol) -> Node<'a> {
    Node{
      info: NodeInfo::Ident(symbol),
         child: Rc::new(Tree::Nil),
         sibling: Rc::new(Tree::Nil)
    }
  }

  pub fn constant<'a>(c: u32) -> Node<'a> {
    Node {
      info: NodeInfo::Constant(c),
      child: Rc::new(Tree::Nil),
      sibling: Rc::new(Tree::Nil)
    }
  }
}

#[derive(Debug)]
enum Tree<'a> {
  Node(Node<'a>),
  Nil
}

// TODO(pht) all parseerror should probably get the corresponding ScanContext
#[derive(Debug)]
enum ParseError {
  UndefinedSymbol(String),
  PrematureEof,
  UnexpectedToken(ScanContext),
  Wtf
}


type ParseResult<'a> = Result<Rc<Tree<'a>>, ParseError>;

struct Parser {}

impl Parser {

  pub fn new() -> Parser {
    Parser{}
  }

  pub fn parse_statement<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {

    // TODO(pht) extract this as a function to try and parse, and fail with an error... 
    let next_scan_result = scanner.next().ok_or(ParseError::PrematureEof)?;
    let next_scan = next_scan_result.unwrap(); // TODO(pht) convert the error to a ParseResult
    
    match next_scan {
      Scan{ token : Token::Ident(ident), context } => {
          // TODO(pht) extract this as a function to try and lookup the symbol, and fail with an error... 
          let ident_symbol = scope.lookup(&ident).ok_or_else(|| { ParseError::UndefinedSymbol(String::from(ident)) })?;

          let next_scan_result = scanner.next().ok_or(ParseError::PrematureEof)?;
          let next_scan = next_scan_result.unwrap(); // TODO(pht) convert the error to a ParseResult

          match next_scan {

            Scan{ token: Token::Becomes, context: _context } => {

              let next_scan_result = scanner.next().ok_or(ParseError::PrematureEof)?;
              let next_scan = next_scan_result.unwrap(); // TODO(pht) convert the error to a ParseResult

              
              match next_scan {

                Scan{ token: Token::Int(constant_value), context: _context } => {

                  let child = Rc::new(Tree::Node(Node::ident(ident_symbol)));

                  let sibling = Rc::new(Tree::Node(Node::constant(constant_value)));

                  return Ok(Rc::new(Tree::Node(Node{
                    info: NodeInfo::Assignement,
                    child,
                    sibling
                  })));

                }

                _ => {

                  return Err(ParseError::UnexpectedToken(context))

                }


              }



            }

            _ => {
              return Err(ParseError::UnexpectedToken(context))
            }

          }


      }
      _ => {
        return Err(ParseError::Wtf);
      }
    }
        
    /*
    if let None = next {
      return Err(ParseError::PrematureEof);
    }

    let symbol = scope.lookup(ident);
    if let None = symbol  {
      return Err(ParseError::UndefinedSymbol(String::from(ident)));
    }

    
    let child = Rc::new(Tree::Node(Node::ident(symbol.unwrap())));

    let sibling = Rc::new(Tree::Node(Node::constant(42)));

    return Ok(Rc::new(Tree::Node(Node{
      info: NodeInfo::Assignement,
      child,
      sibling
    })))
    */

  }
}

#[cfg(test)]
mod tests {

  use super::*;

  // Convenience method to allow exctracting the Node from a tree.
  // I don't know if I should use it except in tests ?
  fn tree_node<'a>(tree: &'a Tree) -> Option<&'a Node<'a>> {
    match tree {
      Tree::Node(node) => Some(node),
      Tree::Nil => None
    }
  }

  #[test]
  fn fails_on_premature_eof() {
    let mut scope = Scope::new();
    scope.add("x");
    
    let mut scanner = Scanner::new("");
    let p = Parser::new();
    let tree = p.parse_statement(&mut scanner, &scope);
    assert_matches!(tree.unwrap_err(), ParseError::PrematureEof);
/* TODO(pht)
    let mut scanner = Scanner::new("x");
    let p = Parser::new();
    let tree = p.parse_statement(&mut scanner, &scope);
    assert_matches!(tree.unwrap_err(), ParseError::PrematureEof);

    let mut scanner = Scanner::new("x:=");
    let p = Parser::new();
    let tree = p.parse_statement(&mut scanner, &scope);
    assert_matches!(tree.unwrap_err(), ParseError::PrematureEof);
    */
  }


  #[test]
  fn fails_parsing_statement_for_unknown_identifier() {
    let scope = Scope::new();
    let mut scanner = Scanner::new("y:=42");

    let p = Parser::new();
    let tree = p.parse_statement(&mut scanner, &scope);
    let e = tree.unwrap_err();
    assert_matches!(e, ParseError::UndefinedSymbol(s) if s == "y");
  }

  #[test]
  fn parses_statement() {
    let mut scope = Scope::new();
    scope.add("x");
    
    let mut scanner = Scanner::new("x:=42");

    let p = Parser::new();
    let tree = p.parse_statement(&mut scanner, &scope).unwrap();
    assert_matches!(tree.as_ref(), Tree::Node(_));

    let node = tree_node(tree.as_ref()).unwrap();
    assert_eq!(node.info, NodeInfo::Assignement);
    assert_matches!(node.child.as_ref(), Tree::Node(_));

    let child_node = tree_node(node.child.as_ref()).unwrap();
    assert_matches!(child_node.info, NodeInfo::Ident(s) if s.name == "x");

    let sibling_node = tree_node(node.sibling.as_ref()).unwrap();
    assert_matches!(sibling_node.info, NodeInfo::Constant(c) if c == 42);
  }

}