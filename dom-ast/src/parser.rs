use crate::scanner::*;
use crate::scope::*;
use crate::token::*;
use crate::tree::*;
use std::rc::Rc;

#[derive(Debug)]
pub enum ParseError {
  ScanError(crate::token::ScanError),
  UndefinedSymbol(String),
  PrematureEof,
  UnexpectedToken(ScanContext),
  Todo,
}

// I have to decide with myself if this goes into uc-parser. Sleep on it.
pub type ParseResult<'a> = Result<Rc<Tree<'a>>, ParseError>;

pub struct Parser {}

impl Parser {
  pub fn new() -> Parser {
    Parser {}
  }

  pub fn parse_statement_sequence<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    println!("parse_statement_sequence {:?}", self.current(scanner));

    let first_statement = self.parse_statement(scanner, scope)?;
   
    let current = self.current(scanner);
    println!("parse_statement current ? {:?}", current);

    if current.is_ok() {
      if let Scan { token : Token::Semicolon, ..} = *(current.unwrap()) {

        let _next = self.scan_next(scanner);

        return Ok(Rc::new(Tree::Node(Node {
          info: NodeInfo::StatementSequence,
          child: first_statement,
          sibling: self.parse_statement_sequence(scanner, scope)?,
        })));
      } 
    }

    return Ok(Rc::new(Tree::Node(Node {
      info: NodeInfo::StatementSequence,
      child: first_statement,
      sibling: Rc::new(Tree::Nil),
    })));
  
  }

  pub fn parse_statement<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    println!("parse_statement {:?}", self.current(scanner));
    let current = self.current(scanner)?;

    if let Scan {
      token: Token::Ident(ident),
      ..
    } = current.as_ref()
    {
      let ident_symbol = self.lookup(scope, &ident)?;
      let subject = Rc::new(Tree::Node(Node::ident(ident_symbol))); // NOTE(pht) this subject is either the ident, or a selector tree from the ident

      println!("Current before calling scan_next {:?}", self.current(scanner));
      self.scan_next(scanner)?;
      println!("Current after calling scan_next {:?}", self.current(scanner));

      let what = self.current(scanner)?;      

      println!("What ? {:?}", what);
      if what.as_ref().token == Token::Becomes {
          self.scan_next(scanner)?;        
          return self.parse_assignment(subject, what.context, scanner, scope);
      }
      return Err(ParseError::UnexpectedToken(what.context));

    }
    return Err(ParseError::Todo); // If statement, etc... 

  }
 
  fn parse_assignment<'a>(&self, subject: Rc<Tree<'a>>, context: ScanContext, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    println!("parse_assignment {:?}", self.current(scanner));

    let object = self.parse_expression(context, scanner, scope)?;

    return Ok(Rc::new(Tree::Node(Node {
      info: NodeInfo::Assignement,
      child: subject,
      sibling: object,
    })));

  }

  pub fn parse_expression<'a>(&self, context: ScanContext, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    println!("parse_expression {:?}", self.current(scanner));

    // NOTE(pht) at the moment, expression := ident | integer...
    // let next = self.scan_next(scanner)?;
    let current = self.current(scanner)?;

    if let Scan{token: Token::Int(constant_value),
      ..
    } = current.as_ref() {

      self.scan_next(scanner)?;
      return Ok(Rc::new(Tree::Node(Node::constant(*constant_value))));
    }

    if let Scan{token: Token::Ident(ident), ..
    } = current.as_ref() {
      let symbol = self.lookup(scope, &ident)?;

      self.scan_next(scanner)?;
      return Ok(Rc::new(Tree::Node(Node::ident(symbol))));
    }
    
    return Err(ParseError::UnexpectedToken(context));
  }

  pub fn parse_factor<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    let current = self.current(scanner)?;

    if let Scan{token: Token::Int(constant_value),
      ..
    } = current.as_ref() {

      self.scan_next(scanner)?;
      return Ok(Rc::new(Tree::Node(Node::constant(*constant_value))));
    }

    if let Scan{token: Token::Ident(ident), ..
    } = current.as_ref() {
      let symbol = self.lookup(scope, &ident)?;

      self.scan_next(scanner)?;
      return Ok(Rc::new(Tree::Node(Node::ident(symbol))));
    }

    return Err(ParseError::UnexpectedToken(current.context));
  }

  pub fn scan_next(&self, scanner: &mut Scanner) -> Result<(), ParseError> {
    println!("Advancing scanner from token {:?}", scanner.current());
    match scanner.next() {
      None => Ok(()),
      Some(scan_result) => match scan_result {
        Ok(_scan) => Ok(()),
        Err(scan_error) => Err(ParseError::ScanError(scan_error)),
      }
    }
  }

  fn current(&self, scanner: &mut Scanner) -> Result<Rc<Scan>, ParseError> {
    let current = scanner.current();
    match current {
      Some(scan) => Ok(scan),
      None => Err(ParseError::PrematureEof)?
    }
  }

  fn lookup<'a>(&self, scope: &'a Scope, ident: &str) -> Result<&'a Symbol, ParseError> {
    return scope.lookup(&ident).ok_or_else(|| ParseError::UndefinedSymbol(String::from(ident)));
  }

}
