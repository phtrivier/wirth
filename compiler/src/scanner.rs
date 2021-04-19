use crate::line_scanner::*;
use crate::token::*;
use std::str::Lines;

#[derive(Debug)]
pub struct Scanner<'a> {
  line_number: u32,
  lines: Lines<'a>,
  line_scanner: LineScanner<'a>,
}

impl Scanner<'_> {
  pub fn new<'a>(s: &'a str) -> Scanner<'a> {
    let mut lines = s.lines();
    match lines.next() {
      Some(line) => {
        return Scanner {
          line_number: 0,
          lines: lines,
          line_scanner: LineScanner::new(0, line),
        }
      }
      None => {
        return Scanner {
          line_number: 0,
          lines: lines,
          line_scanner: LineScanner::new(0, ""),
        }
      }
    }
  }
}

impl Iterator for Scanner<'_> {
  type Item = ScanResult;

  fn next<'a>(&mut self) -> Option<ScanResult> {
    loop {
      match self.line_scanner.next() {
        Some(scan) => {
          return Some(scan);
        }
        None => match self.lines.next() {
          Some(line) => {
            self.line_number = self.line_number + 1;
            self.line_scanner = LineScanner::new(self.line_number, line);
            return self.next();
          }
          None => {
            return None;
          }
        },
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::token::Token;

  #[test]
  fn test_finds_nothing_in_empty_content() {
    let content = "";
    let mut scanner = Scanner::new(&content);
    assert_eq!(None, scanner.next());
  }

  #[test]
  fn test_scan_tokens_on_multiple_lines() {
    let content = " foo \n\n  bar";
    let mut scanner = Scanner::new(&content);
    assert_eq!(
      Ok(Scan {
        context: ScanContext{
          line: 0,
          column: 1
        },
        token: Token::Ident(String::from("foo"))
      }),
      scanner.next().unwrap()
    );
    assert_eq!(
      Ok(Scan {
        context: ScanContext{
          line: 2,
          column: 2
        },
        token: Token::Ident(String::from("bar"))
      }),
      scanner.next().unwrap()
    );
    assert_eq!(None, scanner.next());
  }
}