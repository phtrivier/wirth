# scanner

 - [ ] Replace Option<Scan> with Result<Scan, ScanError> ; use a Token::Eol and Token::Eof to represent the end
  (ScanError is for unexpected char, only)
 - [ ] Scan `.` ,`(`, `)` to be able to parse a procedure call like `foo.bar()`

# parser

  - [ ] Parse a simple procedure call `foo()`
  - [ ] Parse a procedure call with a selector `foo.bar()`