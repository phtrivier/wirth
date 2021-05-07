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

# Done: Shuffle code around to get a nice maintainable organization

## Shuffling
  - [X] It's possible to refer to a package with foo = "../lib-foo".
        And it's possible to change the name of the generated binary for a package.
        So what I want is multiple lib crates, with a split between domain / uc / binaries : 
        - dom-risc  (instructions, encode / decode to instructions, read and write instructions to file)
        - dom-ast        
        - uc-parser         (from a string, create an ast)
        - uc-assembler      (from a reader of string, produce a list of instructions)
        - uc-compiler       (from a reader of string, produce a list of instructions)
        - uc-simulator      (from a list of instructions and a configuration, simulate a computer)
        - bin-assembler     (from a filename of assembly code, produces a binary instruction file)
        - bin-compiler      (from a filename of binary code, produces a file or error messages)
        - bin-simulator     (from a filename of a binary code, simulate the compiler and print output)
        - bin-simulator-gui (from a filename of a binary code, simulate the compiler and display output)

## Improve simulator

  - [X] Extract in uc_simulator with the notion of loading either an assembly file (and compile it on the fly) or a binary file
      - load_instructions(....)
      - status(mem_from, mem_count) -> (registers, memory slice)
      - simulate_all(max_cycle) -> Result((), ComputationNeverEnded 
      
  - [X] Move the `example_tests` to the most appropriate place (in uc_simulator, now)

  - [X] Use uc_simulator in bin_simulator and bin_simulator_gui using flags to load assembler or binary file

# Goal: Extract a function to get the 'current' token during parse

## Scanner / Parser

- [X] Change all types to allow this 'current' function
- [X] Clarify that scan_next() just advances, and current() returns an Option
# Goal: Program that does basic arithmetic
## Parser
  - [X] factor 
  - [X] term
  - [X] simple expression / expressions 
  - [X] expression
  - [X] expression in parenthesis

## codegen
  - [X] ADD and SUB and MUL and DIV