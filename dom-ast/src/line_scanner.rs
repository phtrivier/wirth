use std::iter::Peekable;
use std::str::CharIndices;

use crate::token::*;

use std::rc::Rc;

// <@scanner/line-scanner
#[derive(Debug, Clone)]
pub struct LineScanner<'a> {
  line_number: u32,
  column_number: u32,
  chars: Peekable<CharIndices<'a>>,
  pub current: Option<Rc<Scan>>
}
// @>scanner/line-scanner

impl LineScanner<'_> {
  pub fn new<'a>(line_number: u32, line: &'a str) -> LineScanner<'a> {
    LineScanner {
      line_number: line_number,
      column_number: 0,
      chars: line.char_indices().peekable(),
      current: None
    }
  }
  
  pub fn current(&mut self) -> Option<Rc<Scan>> {
    match &self.current {
      None => None,
      Some(rc) => Some(rc.clone())
    }
  }

  fn context(&self, column_number: u32) -> ScanContext {
    return ScanContext {
      line: self.line_number,
      column: column_number as u32,
    };
  }

  fn forward(&mut self) -> () {
    self.chars.next();
    self.column_number = self.column_number + 1;
  }

  fn token_at(&mut self, column: usize, token: Token) -> Option<ScanResult> {

    let scan = Rc::new(Scan {
      context: self.context(column as u32),
      token,
    });

    self.current = Some(scan.clone());

    return Some(Ok(scan.clone()));
  }

  fn error_at(&self, column: usize, error_type: ScanErrorType) -> Option<ScanResult> {
    return Some(Err(ScanError {
      context: self.context(column as u32),
      error_type,
    }));
  }

  fn scan_single(&mut self, column: usize, token: Token) -> Option<ScanResult> {
    self.chars.next();
    return self.token_at(column, token);
  }

  fn skip_whitespaces(&mut self) -> Option<ScanResult> {
    loop {
      let p = self.chars.peek();
      if let Some(&(_column, next_char)) = p {
        if next_char == ' ' || next_char == '\t' {
          self.forward();
          continue;
        } else {
          break;
        }
      } else {
        break;
      }
    }
    return self.next();
  }

  fn scan_word(&mut self, column: usize) -> Option<ScanResult> {
    let mut ident = String::from("");
    loop {
      let p = self.chars.peek();
      if let Some(&(_column, next_char)) = p {
        if next_char.is_ascii_alphabetic() {
          ident.push(next_char);
          self.forward();
        } else {
          break;
        }
      } else {
        break;
      }
    }
    return match &ident.to_ascii_lowercase()[..] {
      "var" => self.token_at(column, Token::Var),
      "module" => self.token_at(column, Token::Module),
      "begin" => self.token_at(column, Token::Begin),
      "end" => self.token_at(column, Token::End),
      _ => self.token_at(column, Token::Ident(ident))
    }
  }

  fn scan_integer(&mut self, column: usize) -> Option<ScanResult> {
    let mut n: u32 = 0;
    loop {
      let p = self.chars.peek();
      if let Some(&(_column, next_char)) = p {
        if let Some(d) = next_char.to_digit(10) {
          n = n * 10 + d;
          self.forward();
          continue;
          // TODO(pht) replace with single 'break' here ?
        } else {
          break;
        }
      } else {
        break;
      }
    }
    return self.token_at(column, Token::Int(n));
  }

  fn scan_sigil(&mut self, column: usize, first_char: char) -> Option<ScanResult> {
    self.forward();
    let p = self.chars.peek();
    match first_char {
      ':' => {
        if let Some(&(_column, '=')) = p {
          self.forward();
          return self.token_at(column, Token::Becomes);
        } else {
          return self.token_at(column, Token::Colon);
        }
      }
      _ => {
        panic!(
          "Programmer error: function `LineScanner::scan_sigil` called with character `{:?}` that does not start a sigil.",
          first_char
        );
      }
    }
  }
}

impl Iterator for LineScanner<'_> {
  type Item = ScanResult;

  fn next<'a>(&mut self) -> Option<ScanResult> {
    loop {
      let peek = self.chars.peek();
      match peek {
        Some(&(_column, c)) if (c == ' ' || c == '\t') => {
          return self.skip_whitespaces();
        }
        Some(&(column, '\n')) => {
          self.chars.next();
          return self.error_at(column, ScanErrorType::UnexpectedNewLine);
        }
        Some(&(column, c)) if !c.is_ascii() => {
          self.chars.next();
          return self.error_at(column, ScanErrorType::InvalidChar(c));
        }
        
        Some(&(column, c)) if c.is_numeric() => {
          return self.scan_integer(column);
        }
        Some(&(column, ':')) => {
          return self.scan_sigil(column, ':');
        }
        Some(&(column, ';')) => return self.scan_single(column, Token::Semicolon),
        Some(&(column, ',')) => return self.scan_single(column, Token::Comma),
        Some(&(column, '(')) => return self.scan_single(column, Token::Lparen),
        Some(&(column, ')')) => return self.scan_single(column, Token::Rparen),
        Some(&(column, '+')) => return self.scan_single(column, Token::Plus),
        Some(&(column, '-')) => return self.scan_single(column, Token::Minus),
        Some(&(column, '*')) => return self.scan_single(column, Token::Times),
        Some(&(column, '/')) => return self.scan_single(column, Token::Div),
        Some(&(column, '.')) => return self.scan_single(column, Token::Period),
        Some(&(column, _first_char)) => {
          return self.scan_word(column);
        }
        None => {
          self.current = None;
          return None;
        }
      }
    }
  }
}