use std::iter::Peekable;
use std::str::CharIndices;

use crate::token::*;

// <@scanner/line-scanner
#[derive(Debug, Clone)]
pub struct LineScanner<'a> {
  line_number: u32,
  column_number: u32,
  chars: Peekable<CharIndices<'a>>,
}
// @>scanner/line-scanner

impl LineScanner<'_> {
  pub fn new<'a>(line_number: u32, line: &'a str) -> LineScanner<'a> {
    LineScanner {
      line_number: line_number,
      column_number: 0,
      chars: line.char_indices().peekable(),
    }
  }

  fn context(&self, column_number: u32) -> ScanContext {
    return ScanContext {
      line: self.line_number,
      column: column_number as u32,
    };
  }

  fn token_at(&self, column: u32, token: Token) -> Option<ScanResult> {
    return Some(Ok(Scan {
      context: self.context(column as u32),
      token,
    }))
  } 

  fn error_at(&self, column: u32, error_type: ScanErrorType) -> Option<ScanResult> {
    return Some(Err(ScanError {
      context: self.context(column as u32),
      error_type,
    }))
  }

  fn scan_single(&mut self, column_number: u32, token: Token) -> Option<ScanResult> {
    self.chars.next();
    return self.token_at(column_number, token);
  }

  fn skip_whitespaces(&mut self) -> Option<ScanResult> {
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

  fn scan_ident(&mut self, column_number: u32) -> Option<ScanResult> {
    let mut ident = String::from("");
    loop {
      let p = self.chars.peek();
      if let Some(&(_column, next_char)) = p {
        if next_char.is_ascii_alphabetic() {
          ident.push(next_char);
          self.chars.next();
        } else {
          break;
        }
      } else {
        break;
      }
    }
    return self.token_at(column_number, Token::Ident(ident));
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
          return self.error_at(column as u32, ScanErrorType::UnexpectedNewLine);
        }
        Some(&(column, c)) if !c.is_ascii() => {
          self.chars.next();
          return self.error_at(column as u32, ScanErrorType::InvalidChar(c));
        }

        // NOTE(pht) next step would be to try and parse all types of Tokens ; but instead,
        // I'm going to allow myself to parse a "procedure call" tree node like `foo(bar)`
        Some(&(column, '(')) => return self.scan_single(column as u32, Token::Lparen),
        Some(&(column, ')')) => return self.scan_single(column as u32, Token::Rparen),

        Some(&(column, _first_char)) => {
          return self.scan_ident(column as u32);
        }
        None => return None,
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_builds_nothing_in_empty_content() {
    let content = "";
    let mut line_scanner = LineScanner::new(0, &content);
    assert_eq!(None, line_scanner.next());
  }

  #[test]
  fn test_scanner_ignore_whitespaces() {
    let content = "  ";
    let mut scanner = LineScanner::new(0, &content);
    assert_eq!(None, scanner.next());
    assert_eq!(None, scanner.next());
    assert_eq!(2, scanner.column_number);
  }

  #[test]
  fn test_returns_error_on_non_ascii_chars_and_newlines() {
    let content = " ❤\n";
    let mut scanner = LineScanner::new(0, &content);
    assert_eq!(
      Err(ScanError {
        context: ScanContext { line: 0, column: 1 },
        error_type: ScanErrorType::InvalidChar('❤')
      }),
      scanner.next().unwrap()
    );
    assert_eq!(
      Err(ScanError {
        context: ScanContext { line: 0, column: 4 },
        error_type: ScanErrorType::UnexpectedNewLine
      }),
      scanner.next().unwrap()
    );
    assert_eq!(None, scanner.next());
  }

  #[test]
  fn test_scanner_extracts_identifier() {
    let content = "  foo()";
    let mut scanner = LineScanner::new(1, &content);
    assert_eq!(
      Ok(Scan {
        context: ScanContext { line: 1, column: 2 },
        token: Token::Ident(String::from("foo"))
      }),
      scanner.next().unwrap()
    );
    assert_eq!(
      Ok(Scan {
        context: ScanContext { line: 1, column: 5 },
        token: Token::Lparen
      }),
      scanner.next().unwrap()
    );
    assert_eq!(
      Ok(Scan {
        context: ScanContext { line: 1, column: 6 },
        token: Token::Rparen
      }),
      scanner.next().unwrap()
    );
    assert_eq!(None, scanner.next());
  }
}
