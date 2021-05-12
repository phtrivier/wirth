use crate::ast;
use crate::ast::Ast;
use crate::scanner::*;
use crate::scope::*;
use crate::token::*;
use crate::tree::*;

// TODO(pht) remove this type completely and use fns from ast
use crate::tree::TreeNode as Node;

use std::rc::Rc;

#[derive(Debug)]
pub enum ParseError {
  ScanError(crate::token::ScanError),
  UndefinedSymbol(String),
  PrematureEof,
  UnexpectedToken(ScanContext),
  SymbolAlreadyDeclared(String, ScanContext),
  Todo,
}

pub type ParseResult = Result<Rc<Tree>, ParseError>;

pub struct Parser {}

type IdentList = Vec<(String, ScanContext)>;

impl Parser {
  pub fn new() -> Parser {
    Parser {}
  }

  pub fn parse_var_declarations(&self, scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let declarations = self.recur_parse_declaration(scanner, scope)?;
    return Ok(ast::node(NodeInfo::Declarations, declarations, ast::empty()));
  }

  pub fn recur_parse_declaration(&self, scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let current = current_token_or_none(scanner);
    if current == None {
      return Ok(ast::empty());
    }

    let idents = self.parse_ident_list(scanner)?;
    println!("List of idents to declare after first loop {:?}", idents);

    let mut current = current_token(scanner)?;
    println!("After var declarations, current ? {:?}", current);

    if let Scan { token: Token::Colon, .. } = current.as_ref() {
      self.scan_next(scanner)?;
    } else {
      return Err(ParseError::UnexpectedToken(current.context));
    }

    current = current_token(scanner)?;
    if let Scan {
      token: Token::Ident(type_ident),
      context: _type_ident_context,
    } = current.as_ref()
    {
      if type_ident != "INTEGER" {
        return Err(ParseError::UndefinedSymbol(String::from(type_ident)));
      }

      self.scan_next(scanner)?;
      current = current_token(scanner)?;
      if let Scan {
        token: Token::Semicolon,
        ..
      } = current.as_ref() {

        self.scan_next(scanner)?;

        for (ident, ident_context) in idents.iter() {
          add_symbol(scope, &ident, *ident_context)?;
        }
  
        return Self::var_declarations(&mut idents.iter(), scope, Type::Integer, self.recur_parse_declaration(scanner, scope)?);  
      }
    }

    return Err(ParseError::UnexpectedToken(current.as_ref().context));
  }

  fn parse_ident_list(&self, scanner: &mut Scanner) -> Result<IdentList, ParseError> {
    let mut idents: IdentList = vec![];

    let mut current;
    loop {
      current = current_token(scanner)?;
      println!("Scanning var declarations, current ? {:?}", current);
  
      if let Scan {
        token: Token::Ident(ident),
        context: ident_context,
      } = current.as_ref()
      {
        // NOTE(pht) Ideally, I would like not to have to clone the identifier, but since the token can fall
        // out of scope, I don't see a way to do that.
        idents.push((String::from(ident), ident_context.clone()));
        self.scan_next(scanner)?;
  
        current = current_token(scanner)?;
        if let Scan { token: Token::Comma, .. } = current.as_ref() {
          self.scan_next(scanner)?;
          continue;
        } else {
          break;
        }
      }
      break;
    } 
    return Ok(idents);
  }

  pub fn var_declarations(idents: &mut dyn Iterator<Item=&(String, ScanContext)>, scope: &Scope, node_type: crate::tree::Type, final_sibling: Ast) -> ParseResult {

    match idents.next() {
      None => return Ok(final_sibling),
      Some((ident, _ident_context)) => {
        let symbol = lookup(scope, ident)?;
        let child = ast::leaf(NodeInfo::Ident(symbol));
        let sibling = ast::leaf(NodeInfo::Type(node_type));
        let var = ast::node(NodeInfo::Var, child, sibling);

        return Ok(ast::node(NodeInfo::Declaration, var, Self::var_declarations(idents, scope, node_type, final_sibling)?));
      }
    }
  }
  

  pub fn parse_statement_sequence(&self, scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    println!("parse_statement_sequence {:?}", current_token(scanner));

    let first_statement = self.parse_statement(scanner, scope)?;
    let current = current_token(scanner);
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

  pub fn parse_statement(&self, scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    println!("parse_statement {:?}", current_token(scanner));
    let current = current_token(scanner)?;

    if let Scan { token: Token::Ident(ident), .. } = current.as_ref() {
      let ident_symbol = lookup(scope, &ident)?;
      let subject = Rc::new(Tree::Node(Node::ident(ident_symbol))); // NOTE(pht) this subject is either the ident, or a selector tree from the ident

      println!("Current before calling scan_next {:?}", current_token(scanner));
      self.scan_next(scanner)?;
      println!("Current after calling scan_next {:?}", current_token(scanner));

      let what = current_token(scanner)?;

      println!("What ? {:?}", what);
      if what.as_ref().token == Token::Becomes {
        self.scan_next(scanner)?;
        return self.parse_assignment(subject, what.context, scanner, scope);
      }
      return Err(ParseError::UnexpectedToken(what.context));
    }

    return Err(ParseError::Todo); // If statement, etc...
  }

  fn parse_assignment(&self, subject: Rc<Tree>, context: ScanContext, scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    println!("parse_assignment {:?}", current_token(scanner));

    let object = self.parse_expression(context, scanner, scope)?;

    return Ok(Rc::new(Tree::Node(Node {
      info: NodeInfo::Assignement,
      child: subject,
      sibling: object,
    })));
  }

  pub fn parse_expression(&self, _context: ScanContext, scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    return self.parse_simple_expression(scanner, scope);
  }

  pub fn parse_simple_expression(&self, scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let mut tree = self.parse_term(scanner, scope)?;
    println!("parse_simple_expression ; parsed term {:?}", tree);

    loop {
      let current = current_token_or_none(scanner);
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

  pub fn parse_term(&self, scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let mut tree = self.parse_factor(scanner, scope)?;
    loop {
      let current = current_token_or_none(scanner);
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

  pub fn parse_factor(&self, scanner: &mut Scanner, scope: &Scope) -> ParseResult{
    let mut current = current_token(scanner)?;

    if let Scan {
      token: Token::Int(constant_value),
      ..
    } = current.as_ref()
    {
      self.scan_next(scanner)?;
      return Ok(Rc::new(Tree::Node(Node::constant(*constant_value))));
    }

    if let Scan { token: Token::Ident(ident), .. } = current.as_ref() {
      let symbol = lookup(scope, &ident)?;

      self.scan_next(scanner)?;
      return Ok(Rc::new(Tree::Node(Node::ident(symbol.clone()))));
    }

    if let Scan { token: Token::Lparen, context } = current.as_ref() {
      self.scan_next(scanner)?;
      let expression = self.parse_expression(*context, scanner, scope);

      current = current_token(scanner)?;
      if let Scan { token: Token::Rparen, .. } = current.as_ref() {
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
}

fn current_token(scanner: &mut Scanner) -> Result<Rc<Scan>, ParseError> {
  let current = scanner.current();
  match current {
    Some(scan) => Ok(scan),
    None => Err(ParseError::PrematureEof),
  }
}

fn current_token_or_none(scanner: &mut Scanner) -> Option<Rc<Scan>> {
  return scanner.current();
}

fn add_symbol(scope: &Scope, ident: &str, context: ScanContext) -> Result<Rc<Symbol>, ParseError> {
  match scope.lookup(&ident) {
    None => {
      scope.add(ident);
      return lookup(scope, ident);
    }
    Some(_symbol) => {
      return Err(ParseError::SymbolAlreadyDeclared(String::from(ident), context));
    }
  }
}

fn lookup(scope: &Scope, ident: &str) -> Result<Rc<Symbol>, ParseError> {
  return scope.lookup(&ident).ok_or_else(|| ParseError::UndefinedSymbol(String::from(ident)));
}
