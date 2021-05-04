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

  fn forward(&mut self) -> () {
    self.chars.next();
    self.column_number = self.column_number + 1;
  }

  fn token_at(&self, column: usize, token: Token) -> Option<ScanResult> {
    return Some(Ok(Scan {
      context: self.context(column as u32),
      token,
    }));
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

  fn scan_ident(&mut self, column: usize) -> Option<ScanResult> {
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
    return self.token_at(column, Token::Ident(ident));
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
        Some(&(column, '(')) => return self.scan_single(column, Token::Lparen),
        Some(&(column, ')')) => return self.scan_single(column, Token::Rparen),

        Some(&(column, _first_char)) => {
          return self.scan_ident(column);
        }
        None => return None,
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn assert_scans_all(scanner: &mut LineScanner, tests: Vec<(u32, u32, Token)>) -> () {
    for (l, c, t) in tests {
      assert_scans(scanner, l, c, t);
    }
    assert_done(scanner);
  }

  fn assert_scans(scanner: &mut LineScanner, line: u32, column: u32, token: Token) -> () {
    assert_eq!(
      Ok(Scan {
        context: ScanContext { line: line, column: column },
        token: token
      }),
      scanner.next().unwrap()
    );
  }

  fn assert_scans_error(scanner: &mut LineScanner, line: u32, column: u32, error_type: ScanErrorType) -> () {
    assert_eq!(
      Err(ScanError {
        context: ScanContext { line, column },
        error_type
      }),
      scanner.next().unwrap()
    );
  }

  fn assert_done(scanner: &mut LineScanner) -> () {
    assert_eq!(None, scanner.next());
  }

  #[test]
  fn test_builds_nothing_in_empty_content() {
    let mut line_scanner = LineScanner::new(0, "");
    assert_done(&mut line_scanner);
  }

  #[test]
  fn test_scanner_ignore_whitespaces() {
    let mut line_scanner = LineScanner::new(0, "  ");
    assert_done(&mut line_scanner);
    assert_done(&mut line_scanner);
    assert_eq!(2, line_scanner.column_number);
  }

  #[test]
  fn test_returns_error_on_non_ascii_chars_and_newlines() {
    let mut scanner = LineScanner::new(0, " ❤\n");
    assert_scans_error(&mut scanner, 0, 1, ScanErrorType::InvalidChar('❤'));
    assert_scans_error(&mut scanner, 0, 4, ScanErrorType::UnexpectedNewLine);
    assert_done(&mut scanner);
  }


  #[test]
  fn test_scans_identifier() {
    let content = "  foo()";
    let mut scanner = LineScanner::new(1, &content);

    assert_scans_all(
      &mut scanner,
      vec![
        (1, 2, Token::Ident(String::from("foo"))), // foo
        (1, 5, Token::Lparen),                     // (
        (1, 6, Token::Rparen),                     // )
      ],
    );
  }

  #[test]
  fn test_scans_assignements() {
    let content = "  foo := 742 ; bar()";
    let mut scanner = LineScanner::new(0, &content);
    assert_scans_all(
      &mut scanner,
      vec![
        (0, 2, Token::Ident(String::from("foo"))),  // foo
        (0, 6, Token::Becomes),                     // :=
        (0, 9, Token::Int(742)),                    // 742
        (0, 13, Token::Semicolon),                  // ;
        (0, 15, Token::Ident(String::from("bar"))), // bar
        (0, 18, Token::Lparen),                     // (
        (0, 19, Token::Rparen),                     // )
      ],
    );
  }

  #[test]
  fn test_scans_assignment_to_ident() {
    let mut scanner = LineScanner::new(0, "x:=y");
    assert_scans_all(
      &mut scanner,
      vec![
        (0, 0, Token::Ident(String::from("x"))),
        (0, 1, Token::Becomes),
        (0, 3, Token::Ident(String::from("y")))
      ],
    );
  }

}
