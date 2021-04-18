use std::iter::Peekable;
use std::str::CharIndices;
use std::str::Lines;

use crate::token::Token;
use crate::token::Scan;

// <@scanner/line-scanner
#[derive(Debug)]
struct LineScanner<'a> {
  line_number: u32,
  column_number: u32,
  chars: Peekable<CharIndices<'a>>,
}
// @>scanner/line-scanner

impl LineScanner<'_> {
  pub fn new<'a>(line_number: u32, lines: &mut Lines<'a>) -> Option<LineScanner<'a>> {
    match lines.next() {
      None => return None,
      Some(line) => Some(LineScanner {
        line_number: line_number,
        column_number: 0,
        chars: line.char_indices().peekable(),
      }),
    }
  }
  pub fn next<'a>(&mut self) -> Option<Scan> {
    loop {
      let peek = self.chars.peek();
      println!("Peeked {:?}", peek);
      match peek {
        Some(&(_column, c)) if (c == ' ' || c == '\t') => {
          println!("Peeked whitespace, will skip");
          return self.skip_whitespaces();
        }
        Some(&(_column, '\n')) => {
          panic!("Found newline in LineScanner content, should come from a Lines iterator");
        }

        // NOTE(pht) next step would be to try and parse all types of Tokens ; but instead, 
        // I'm going to allow myself to parse a "procedure call" tree node like `foo(bar)`

        Some(&(column, '(')) => { return self.scan_single(column as u32, Token::Lparen)}
        Some(&(column, ')')) => { return self.scan_single(column as u32, Token::Rparen)}
        
        Some(&(column, _first_char)) => {
          return self.scan_ident(column as u32);
        },
        None => return None,
      }
    }
  }

  fn scan_single(&mut self, column_number: u32, token: Token) -> Option<Scan> {
    self.chars.next();
    return Some(Scan{
      line_number: self.line_number, 
      column_number,
      token
    })
  }

  fn skip_whitespaces(&mut self) -> Option<Scan> {
    loop {
      let next_char = self.chars.peek();
      if let Some(&(_column, c)) = next_char {
        if c == ' ' || c == '\t' {
          self.column_number = self.column_number + 1;
          self.chars.next();
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

  fn scan_ident(&mut self, column_number: u32) -> Option<Scan> {
    let mut ident = String::from("");
    loop {
      let p = self.chars.peek();
      if let Some(&(_column, next_char)) = p {
        if next_char.is_ascii_alphanumeric() {
          ident.push(next_char);
          self.chars.next();
        } else {
          break;
        }
      } else {
        break;
      }
    }
    return Some(Scan{
      line_number: self.line_number,
      column_number: column_number as u32,
      token: Token::Ident(ident)
    })
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_scanner_ignore_whitespaces() {
    let content = "  ";
    let mut scanner = LineScanner::new(0, &mut content.lines()).unwrap();
    assert_eq!(None, scanner.next());
    assert_eq!(None, scanner.next());
    assert_eq!(2, scanner.column_number);
  }

  #[test]
  fn test_scanner_extracts_identifier() {
    let content = "  foo()";
    let mut scanner = LineScanner::new(1, &mut content.lines()).unwrap();
    assert_eq!(Some(Scan{line_number: 1, column_number: 2, token: Token::Ident(String::from("foo"))}), scanner.next());
    assert_eq!(Some(Scan{line_number: 1, column_number: 5, token: Token::Lparen}), scanner.next());
    assert_eq!(Some(Scan{line_number: 1, column_number: 6, token: Token::Rparen}), scanner.next());
    assert_eq!(None, scanner.next());   
  }
}