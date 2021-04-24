use crate::scanner::*;
use crate::scope::*;
use std::rc::Rc;


#[derive(Debug, PartialEq)]
enum NodeInfo<'a> {
  Assignement,
  Constant(u32),
  Ident(&'a Symbol) // NOTE(pht) I wonder if this could be done with a &Symbol, with the appropriate lifetime ?
}

#[derive(Debug)]
enum Tree<'a> {
  Node{
    info: NodeInfo<'a>,
    child: Rc<Tree<'a>>,
    sibling: Rc<Tree<'a>>
  },
  Nil
}

#[derive(Debug)]
enum ParseError {
  UndefinedSymbol(String)
}


type ParseResult<'a> = Result<Rc<Tree<'a>>, ParseError>;

struct Parser {}

impl Parser {

  pub fn new() -> Parser {
    Parser{}
  }

  pub fn parse_statement<'a>(&self, ident: &str, _scanner: &Scanner, scope: &'a Scope) -> ParseResult<'a> {
    let symbol = scope.lookup(ident);
    if let None = symbol  {
      return Err(ParseError::UndefinedSymbol(String::from(ident)));
    }

    return Ok(Rc::new(Tree::Node{
      info: NodeInfo::Assignement,
      child: Rc::new(Tree::Node{
        info: NodeInfo::Ident(symbol.unwrap()),
        child: Rc::new(Tree::Nil),
        sibling: Rc::new(Tree::Nil)
      }),
      sibling: Rc::new(Tree::Node{
        info: NodeInfo::Constant(42),
        child: Rc::new(Tree::Nil),
        sibling: Rc::new(Tree::Nil)
      })
    }));
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn fails_parsing_statement_for_unknown_identifier() {
    let scope = Scope::new();
    // TODO(pht) : prepare s with symbol "x" to softcode
    let scanner = Scanner::new("y:=42");

    let p = Parser::new();
    let tree = p.parse_statement("y", &scanner, &scope);
    let e = tree.unwrap_err();
    assert_matches!(e, ParseError::UndefinedSymbol(s) if s == "y");
  }

  #[test]
  fn parses_statement() {
    let mut scope = Scope::new();
    scope.add("x");
    // TODO(pht) : prepare s with symbol "x" to softcode
    let scanner = Scanner::new("x:=42");

    let p = Parser::new();
    let tree = p.parse_statement("x", &scanner, &scope).unwrap();
    assert_matches!(tree.as_ref(), Tree::Node{ info, child: _child, sibling: _sibling } if NodeInfo::Assignement == *info);

  }

}