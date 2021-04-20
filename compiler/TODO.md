# Goal: generate a program that sets a register to 42

# computer

 - [ ] Adapt the the 2017 computer version, mine is outdated... grr

# scanner

 - [X] Replace Option<Scan> with Option<Result<Scan, ScanError>>. None still represent end of inputs.
  (ScanError is for unexpected char, and unexpected newlines.)
 - [ ] Scan `!=` , idents and numbers to be able to parse an expression like `x := 42`

# parser

  - [ ] Parse a constant number
  - [ ] Parse a simple ident (and put it in a symbol table for later retrieval, assuming it's a )
  - [ ] Parse an assignement
  - [ ] Result should be a tree if things worked...

# cogen
  - [ ] From the basic tree, generate a bunch of instructions

# misc

  - [ ] Introduce an actual logger crate ?