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
      if let Scan { token: Token::Semicolon, .. } = *(current.unwrap()) {
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

    if let Scan { token: Token::Ident(ident), .. } = current.as_ref() {
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

  pub fn parse_expression<'a>(&self, _context: ScanContext, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    return self.parse_simple_expression(scanner, scope);
  }

  pub fn parse_simple_expression<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    let mut tree = self.parse_term(scanner, scope)?;
    println!("parse_simple_expression ; parsed term {:?}", tree);

    loop {
      let current = self.current_or_none(scanner);
      println!("parse_simple_expression in loop, current ? {:?}", current);

      if let Some(scan) = current.as_ref() {
        let operator: Option<SimpleExpressionOp> = match scan.as_ref() {
          Scan { token: Token::Plus, .. } => Some(SimpleExpressionOp::Plus),
          Scan { token: Token::Minus, .. } => Some(SimpleExpressionOp::Minus),
          _ => None,
        };

        match operator {
          Some(operator) => {
            println!("parse_simple_expression in loop, + found");
            self.scan_next(scanner)?;
            let sibling = self.parse_term(scanner, scope)?;
            let node = Node {
              info: NodeInfo::SimpleExpression(operator),
              child: tree,
              sibling: sibling,
            };
            tree = Rc::new(Tree::Node(node));
            continue;
          }
          None => {
            break;
          }
        }
      }
      break; 
    }
    
    return Ok(tree);
  }

  pub fn parse_term<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    let mut tree = self.parse_factor(scanner, scope)?;
    loop {
      let current = self.current_or_none(scanner);
      println!("parse_term loop, current ? {:?}", current);

      match current {
        None => {
          break;
        }
        Some(scan) => {
          let operator: Option<TermOp> = match scan.as_ref() {
            Scan { token: Token::Times, .. } => Some(TermOp::Times),
            Scan { token: Token::Div, .. } => Some(TermOp::Div),
            _ => None,
          };

          match operator {
            Some(operator) => {
              println!("parse_simple_expression in loop, + found");
              self.scan_next(scanner)?;
              let sibling = self.parse_factor(scanner, scope)?;
              let node = Node {
                info: NodeInfo::Term(operator),
                child: tree,
                sibling: sibling,
              };
              tree = Rc::new(Tree::Node(node));
              continue;
            }
            None => {
              break;
            }
          }
        }
      }
    }
    return Ok(tree);
  }

  pub fn parse_factor<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    let mut current = self.current(scanner)?;

    if let Scan {
      token: Token::Int(constant_value),
      ..
    } = current.as_ref()
    {
      self.scan_next(scanner)?;
      return Ok(Rc::new(Tree::Node(Node::constant(*constant_value))));
    }

    if let Scan { token: Token::Ident(ident), .. } = current.as_ref() {
      let symbol = self.lookup(scope, &ident)?;

      self.scan_next(scanner)?;
      return Ok(Rc::new(Tree::Node(Node::ident(symbol))));
    }

    if let Scan { token: Token::Lparen, context} = current.as_ref() {
      self.scan_next(scanner)?;
      let expression = self.parse_expression(*context, scanner, scope);

      current = self.current(scanner)?;
      if let Scan { token: Token::Rparen, ..} = current.as_ref() {
        self.scan_next(scanner)?;
        return expression;
      }
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
      },
    }
  }

  fn current(&self, scanner: &mut Scanner) -> Result<Rc<Scan>, ParseError> {
    let current = scanner.current();
    match current {
      Some(scan) => Ok(scan),
      None => Err(ParseError::PrematureEof),
    }
  }

  fn current_or_none(&self, scanner: &mut Scanner) -> Option<Rc<Scan>> {
    return scanner.current();
  }

  fn lookup<'a>(&self, scope: &'a Scope, ident: &str) -> Result<&'a Symbol, ParseError> {
    return scope.lookup(&ident).ok_or_else(|| ParseError::UndefinedSymbol(String::from(ident)));
  }
}
