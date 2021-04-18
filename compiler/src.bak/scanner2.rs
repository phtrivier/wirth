use std::str::Lines;
use std::str::Chars;
use std::str::CharIndices;

#[derive(Debug)]
pub enum Token {
  Int { value: u32 },
  Ident { value: String },
}

#[derive(Debug)]
pub struct Scan {
  line: u32,
  column: u32,
  token: Token,
}

#[derive(Debug)]
pub enum ScanError {
  BadChar(char),
}

pub struct LineScanner<'a> {
  line_number: u32,
  char_number: u32,
  chars: Chars<'a>,
}

impl LineScanner<'_> {
  pub fn new<'a>(content: &'a str, line_number: u32) -> LineScanner<'a> {
    LineScanner {
      line_number: line_number,
      chars: content.chars(),
      char_number: 0
    }
  }
}

impl Iterator for LineScanner<'_> {
  type Item = Scan;

  fn next(&mut self) -> Option<Self::Item> {
    // loop {
    //   let c = self.chars.next();
    //   self.char_number = self.char_number+1;
    //   match c {
    //     Some(' ') => {
    //       continue;
    //     }
    //     Some(single_char) => {

    //       self.chars.

    //     }
    //     None => {
    //       return None;
    //     }
    //   }
    // }
    return None
  }
}

/*
pub struct Scanner<'a> {
  content: &'a str
}

impl Scanner<'_> {
  pub fn new<'a>(content: &'a str) -> Scanner<'a> {
    Scanner{
      content: content
    }
  }
}
*/

#[cfg(test)]
mod tests {
  use super::*;


  /*
  let v: Vec<_> = "abcXXXabcYYYabc".match_indices("abc").collect();
assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);

let v: Vec<_> = "1abcabc2".match_indices("abc").collect();
assert_eq!(v, [(1, "abc"), (4, "abc")]);

let v: Vec<_> = "ababa".match_indices("aba").collect();
assert_eq!(v, [(0, "aba")]); // only the first `aba`
  */

  // @<intro/reality_check
  #[test]
  fn test_fake_news() {
    assert_eq!(true, true)
  }
  // >@intro/reality_check

    /*
    let mut w = word_ite.next();
    assert_matches!(w, Some((2, _)));
    assert_eq!(w.unwrap().1, "HELLO");

    w = word_ite.next();
    assert_matches!(w, Some((11, _)));
    assert_eq!(w.unwrap().1, "WORLD");
    */
  

  /*
  #[test]
  fn line_scanner_can_scan_ident_token() {
    let content = String::from("   HELLO  WORLD");
    let mut line_scanner = LineScanner::new(&content, 0);
    let scan = line_scanner.next().unwrap();
    assert_matches!(
      scan,
      Scan {
        line: 0,
        column: 4,
        token: Token::Ident { value: _value }
      }
    );
  }
  */

  /*
  #[test]
  fn scanner_can_scan_ident_token() {
    let content = "\t \t
      HELLO
       WORLD
    ";

    let mut scanner = Scanner::new(&content);

    assert_matches!(scanner.scan(), Ok(Token::Ident{value: "HELLO"}));

    // println!("{:?}", scanner.lines.next());
  }
  */
}
