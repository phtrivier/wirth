# Done: generate a program that sets a memory location to 42

## risc

 - [X] Adapt the the 2017 computer version, mine is outdated... 
    - [X] Adapt computer instruction encoding / parsing
    - [X] Adapt computer instruction execution
    - [X] Adapt gui-risc and other executables

## assembler

  - [X] Adapt to the 2017 computer version (normaly only the assembler should be affected, when parsing files...)

## scanner

 - [X] Replace Option<Scan> with Option<Result<Scan, ScanError>>. None still represent end of inputs.
  (ScanError is for unexpected char, and unexpected newlines.)
 - [X] Scan `!=` , idents and numbers to be able to parse an expression like `x := 42`

## parser

  - [X] Parse a constant number
  - [X] Parse a simple ident (and put it in a symbol table for later retrieval, assuming it's a )
  - [X] Parse an assignement
  - [X] Parse an assignment sequence
  - [X] Result should be a tree if things worked

## codegen
  - [X] From the basic tree, generate a bunch of instructions to assign constants to variables 
  - [X] Assign constant to variable and variable to variable

## cli-compiler
  - [X] Tie scanner, parser and codegen to generate an object file for a program

## cli-risc
  - [X] Modernize example programs and see them run in the client
  - [X] Load the object with cli-risc, and feel great about yourself. 
