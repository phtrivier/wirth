# Goal: Shuffle code around to get a nice maintainable organization

  - [ ] Extract a lib in the "compiler" crate to go from text to object code
  - [ ] Add an "e2e" test somewhere (where ?) to check the compiler behavior
  - [ ] Make cli-assembler that reads text and produce binary
  - [ ] Make a lib in risc to load binary and run code, use it in cli-risc
  - [ ] Rename crates so that all domain is grouped, and all clis are grouped :D (ultimate yak shaving)

# Goal: Program that does basic arithmetic

## Parser
  - [ ] factor 
  - [ ] term
  - [ ] simple expression / expressions 
  - [ ] expression

## codegen
  - [ ] ADD and SUB and MUL and DIV