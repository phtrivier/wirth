use crate::scanner::*;

#[derive(Debug)]
struct Symbol {
  name: String
}

struct Scope {
}
impl Scope {

  pub fn new() -> Scope {
    Scope{
    }
  }

  pub fn lookup(&self, s: &str) -> Option<Symbol> {
    match s {
      "x" => Some(Symbol{
          name: String::from("x")
        }),
      _ => None
    }
  }
}

#[derive(Debug)]
enum ParseError {
  UndefinedSymbol(String)
}

#[derive(Debug)]
enum Tree {
  Nil
}

type ParseResult = Result<Tree, ParseError>;

struct Parser {}

impl Parser {

  pub fn new() -> Parser {
    Parser{}
  }

  pub fn parse_statement(&self, ident: &str, scanner: &Scanner, scope: &Scope) -> ParseResult {
    let symbol = scope.lookup(ident);
    if let None = symbol  {
      return Err(ParseError::UndefinedSymbol(String::from(ident)));
    }

    return Ok(Tree::Nil)
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
}