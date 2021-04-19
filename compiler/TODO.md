# scanner

 - [X] Replace Option<Scan> with Option<Result<Scan, ScanError>>. None still represent end of inputs.
  (ScanError is for unexpected char, and unexpected newlines.)
 - [ ] Scan `.` ,`(`, `)` to be able to parse a procedure call like `foo.bar()`

# parser

  - [ ] Parse a simple procedure call `foo()`
  - [ ] Parse a procedure call with a selector `foo.bar()`

# misc

  - [ ] Introduce an actual logger crate ?