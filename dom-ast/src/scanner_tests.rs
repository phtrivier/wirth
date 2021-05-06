
#[cfg(test)]
mod tests {
  use crate::token::*;
  use crate::scanner::*;

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
      Scan {
        context: ScanContext{
          line: 0,
          column: 1
        },
        token: Token::Ident(String::from("foo"))
      },
      *(scanner.next().unwrap().unwrap().as_ref())
    );

    assert_eq!(
      Scan {
        context: ScanContext{
          line: 0,
          column: 1
        },
        token: Token::Ident(String::from("foo"))
      },
      *(scanner.current().unwrap().as_ref())
    );
    
    assert_eq!(
      Scan {
        context: ScanContext{
          line: 2,
          column: 2
        },
        token: Token::Ident(String::from("bar"))
      },
      *(scanner.next().unwrap().unwrap().as_ref())
    );

    assert_eq!(
      Scan {
        context: ScanContext{
          line: 2,
          column: 2
        },
        token: Token::Ident(String::from("bar"))
      },
      *(scanner.current().unwrap().as_ref())
    );
    
    assert_eq!(None, scanner.next());
  }
}