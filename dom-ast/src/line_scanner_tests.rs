
#[cfg(test)]
mod tests {
  use crate::token::*;
  use crate::line_scanner::*;

  fn assert_scans_all(scanner: &mut LineScanner, tests: Vec<(u32, u32, Token)>) -> () {
    for (l, c, t) in tests {
      assert_scans(scanner, l, c, t);
    }
    assert_done(scanner);
  }

  fn assert_scans(scanner: &mut LineScanner, line: u32, column: u32, token: Token) -> () {
    assert_eq!(
      Scan {
        context: ScanContext { line: line, column: column },
        token: token
      },
      *(scanner.next().unwrap().unwrap().as_ref())
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
    let content = "foo";
    let mut scanner = LineScanner::new(1, &content);

    assert_scans(&mut scanner, 1, 0, Token::Ident(String::from("foo")));

    assert_eq!(*(scanner.current().unwrap().as_ref()), Scan {
      context: ScanContext { line: 1, column: 0 },
      token: Token::Ident(String::from("foo"))
    });

    assert_eq!(scanner.next(), None);
    assert_eq!(scanner.current(), None);

  }

  #[test]
  fn test_skips_whitespace() {
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

  #[test]
  fn test_scans_arithmetic() {
    let mut scanner = LineScanner::new(0, "(x/42)+(y*12)-3");
    assert_scans_all(
      &mut scanner,
      vec![
        (0, 0, Token::Lparen),
        (0, 1, Token::Ident(String::from("x"))),
        (0, 2, Token::Div),
        (0, 3, Token::Int(42)),
        (0, 5, Token::Rparen),
        (0, 6, Token::Plus),
        (0, 7, Token::Lparen),
        (0, 8, Token::Ident(String::from("y"))),
        (0, 9, Token::Times),
        (0, 10, Token::Int(12)),
        (0, 12, Token::Rparen),
        (0, 13, Token::Minus),
        (0, 14, Token::Int(3)),
      ],
    );
  }

}