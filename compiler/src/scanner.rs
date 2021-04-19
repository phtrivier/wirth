use std::str::Lines;
use crate::token::Scan;
use crate::token::Token;
use crate::line_scanner::LineScanner;

#[derive(Debug)]
struct Scanner<'a> {
    line_number: u32,
    lines: Lines<'a>,
    line_scanner: Option<LineScanner<'a>>
}

impl Scanner<'_> {
  pub fn new<'a>(s: &'a str) -> Option<Scanner<'a>> {
    let mut lines = s.lines();
    let line_scanner = LineScanner::new(0, &mut lines);

    if let Some(_) = line_scanner {
      return Some(Scanner{
        line_number: 0,
        lines: lines,
        line_scanner
      })
    } else {
      return  None
    }
  }
}

impl Iterator for Scanner<'_> {
  type Item = Scan;

  fn next<'a>(&mut self) -> Option<Scan> {
    return None;
  }
}

mod tests {
  use super::*;
  
  #[test]
  fn test_finds_nothing_in_empty_content() {
    let content = "";
    let mut scanner = Scanner::new(&content).unwrap();
    assert_eq!(None, scanner.next());    
  }

  // #[test]
  // fn test_scan_tokens_on_multiple_lines() {
  //   let content = " foo \n  bar";
  //   let mut scanner = Scanner::new(&content).unwrap();
  //   assert_eq!(Scan{line_number: 0, column_number: 1, token: Token::Ident(String::from("foo"))}, scanner.next().unwrap());
  //   assert_eq!(Scan{line_number: 1, column_number: 2, token: Token::Ident(String::from("bar"))}, scanner.next().unwrap());
  //   assert_eq!(None, scanner.next());
  // }
}

// impl Scanner <'_> {

//     pub fn new<'a>(content: &'a str) -> Scanner<'a> {
//         let lines = content.lines();
//         Scanner{
//             line_number: 0,
//             lines: lines,
//             line_scanner: LineScanner::new(&mut lines)
//         }
//     }

//     pub fn next<'a>(&mut self) -> Option<Scan> {
//         loop {
//             let peek = self.chars.peek();
//             println!("Peeked {:?}", peek);
//             match peek {
//                 Some(&(_column, c)) if (c == ' ' || c == '\t') => {
//                     println!("Peeked whitespace, will skip");
//                     return self.skip_whitespaces();
//                 }
//                 Some(&(_column, '\n')) => {
//                   println!("Peeked newline, will skip");
//                   return self.skip_newline();
//                 }
//                 Some(_) => {
//                   return None
//                 }
//                 None => {
//                     return None
//                 }
//             }
//         }
//     }

//     fn skip_whitespaces(&mut self) -> Option<Scan> {
//       loop {
//         let next_char = self.chars.next();
//         if let Some((_column, c)) = next_char {
//           if c == ' ' || c == '\t' {
//             continue;
//           } else {
//             break;
//           }
//         } else {
//           break;
//         }
//       }
//       return self.next();
//     }

//     fn skip_newline(&mut self) -> Option<Scan> {
//       self.line_number = self.line_number + 1;
//       self.chars.next();
//       return self.next();
//     }

// }
