# Goal: generate a program that sets a memory location to 42

# risc

 - [X] Adapt the the 2017 computer version, mine is outdated... 
    - [X] Adapt computer instruction encoding / parsing
    - [X] Adapt computer instruction execution
    - [X] Adapt gui-risc and other executables

Next:
 - [ ] Support negative memory for Input / Output

# assembler

  - [X] Adapt to the 2017 computer version (normaly only the assembler should be affected, when parsing files...)

# scanner

 - [X] Replace Option<Scan> with Option<Result<Scan, ScanError>>. None still represent end of inputs.
  (ScanError is for unexpected char, and unexpected newlines.)
 - [ ] Scan `!=` , idents and numbers to be able to parse an expression like `x := 42`

# parser

  - [X] Parse a constant number
  - [X] Parse a simple ident (and put it in a symbol table for later retrieval, assuming it's a )
  - [X] Parse an assignement
  - [X] Parse an assignment sequence
  - [X] Result should be a tree if things worked

# cli-risc
  - [ ] Modernize example programs and see them run in the client

Next:
  - [ ] Replace structopt with `clap` (If that's better...)

# codegen
  - [ ] From the basic tree, generate a bunch of instructions to assign variables

# compiler
  - [ ] Tie scanner, parser and codegen to generate an assembly program that sets a register to a value.
    Load it with cli-risc, and feel great about yourself. 

# gui-risc

Next:
  - [ ] Rename to simulator-gui
  - [ ] Use druid instead of the raylib stuff ?
  - [ ] Multiple (scrollable) panes: 
    - [ ] Registers
    - [ ] Program memory (starting at 0)
    - [ ] Data memory (custom location)
    - [ ] Small framebuffer to display stuff, for fun ?
      - [ ] GB : 160 x 144 x 2bits = 46080 bits == 1440 words https://en.wikipedia.org/wiki/Game_Boy
      (That leaves be a few Ks to write Tetris, but I can always bump the main memory if I need to ;) )

# misc

Next:
  - [ ] Introduce an actual logger crate ?
  - [ ] Rename `risc` crate to simulator
