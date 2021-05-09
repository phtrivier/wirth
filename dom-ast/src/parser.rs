use crate::ast;
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

// I have to decide with myself if this goes into uc-parser. Sleep on it.
pub type ParseResult<'a> = Result<Rc<Tree<'a>>, ParseError>;

pub struct Parser {}

type IdentList = Vec<(String, ScanContext)>;

impl Parser {
  pub fn new() -> Parser {
    Parser {}
  }

  pub fn parse_var_declarations<'a, 'b>(&'a self, scanner: &mut Scanner, scope: &'b mut Scope) -> ParseResult<'b> {
    // let current = current_token(scanner)?;
    // if let Scan {
    //   token: Token::Ident(..),
    //   ..
    // } = current.as_ref() {

    //   let declarations = self.parse_single_line_of_var_declarations(scanner, scope)?;

    //   let next_declarations = self.parse_var_declarations(scanner, scope)?;
    //   return Ok(ast::node(NodeInfo::Declarations, declarations, next_declarations));

    // } else {
    //   return Ok(ast::empty());
    // }
    return self.parse_single_line_of_var_declarations(scanner, scope);

  }

  pub fn parse_single_line_of_var_declarations<'a, 'b>(&self, scanner: &mut Scanner, scope: &'b mut Scope) -> ParseResult<'b> {
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
    println!("After consumeing comma, current ? {:?}", current);
    
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
        for (ident, ident_context) in idents.iter() {
          add_symbol(scope, &ident, *ident_context)?;
        }
  
        return Self::var_declarations(&mut idents.iter(), scope, Type::Integer);  
      }
    }

    return Err(ParseError::UnexpectedToken(current.as_ref().context));
  }


  fn parse_ident_list<'a>(&self, scanner: &mut Scanner) -> Result<IdentList, ParseError> {
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

  pub fn var_declarations<'a>(idents: &mut dyn Iterator<Item=&(String, ScanContext)>, scope: &'a Scope, node_type: crate::tree::Type) -> ParseResult<'a> {

    match idents.next() {
      None => return Ok(ast::empty()),
      Some((ident, _ident_context)) => {
        let symbol = lookup(scope, ident)?;
        let child = ast::leaf(NodeInfo::Ident(symbol));
        let sibling = ast::leaf(NodeInfo::Type(node_type));
        let var = ast::node(NodeInfo::Var, child, sibling);

        let next_declaration = Self::var_declarations(idents, scope, node_type)?;

        return Ok(ast::node(NodeInfo::Declaration, var, next_declaration));
      }
    }
  }
  

  pub fn parse_statement_sequence<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
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

  pub fn parse_statement<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
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

  fn parse_assignment<'a>(&self, subject: Rc<Tree<'a>>, context: ScanContext, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
    println!("parse_assignment {:?}", current_token(scanner));

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

  pub fn parse_term<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
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

  pub fn parse_factor<'a>(&self, scanner: &mut Scanner, scope: &'a Scope) -> ParseResult<'a> {
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
      return Ok(Rc::new(Tree::Node(Node::ident(symbol))));
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

fn add_symbol<'a>(scope: &'a mut Scope, ident: &str, context: ScanContext) -> Result<&'a Symbol, ParseError> {
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

fn lookup<'a>(scope: &'a Scope, ident: &str) -> Result<&'a Symbol, ParseError> {
  return scope.lookup(&ident).ok_or_else(|| ParseError::UndefinedSymbol(String::from(ident)));
}
