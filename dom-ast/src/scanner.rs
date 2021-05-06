use crate::line_scanner::*;
use crate::token::*;
use std::str::Lines;
use std::rc::Rc;

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

  pub fn current(&mut self) -> Option<Rc<Scan>> {
    return self.line_scanner.current();
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
