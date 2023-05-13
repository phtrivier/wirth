# Compiler Error handling

- [ ] Add the notion of "compiling a file" to produce a binary
- [ ] Nicely display the parsing / scanning erors instead of panicking

# Debugging assembled program

- [ ] When running the simulator from an assembly program, make the pc counter match
  with the program location and display that instead of the assembled code

# Debugging compiled program
 
 - [ ] When compiling, optionnaly produce a separate "dbg" file that maps instructions
 to the line of the file that produced the ast instruction

# Boolean expressions

# Procedures

(Nested scopes, etc...)

# Record types

# Separate compilation

# Stuff leftout

- elsif, finally ? 

# Optimization

Rewrite the tree to avoid unnecessary computations, optimize things, etc...

# Improve code coverage
## Libraries
  - [ ] Just add more tests, maybe it's just acceptance tests missing...
## binaries
  - [ ] Extract functions to compute the arguments
  - [ ] Extract function that runs the main based on the arguments
  - [ ] Add integration test that reads simulated arguments and actually calls the lib.
  - [ ] Then simply exclude the "main" function from tarpaulin because it becomes irrelevant

# Goal: Literate Programming system

 - [ ] extend the language with comments
 - [ ] add a tangler that parses certain comments `(*@ Foobar...@*)`and consider them as "Sections" to be completed, from an external list of sections
 - [ ] A web is then a list of sections ; each section having a text part, and a code part.
 
 Tangling is traversing the web once to produce the list of sections, and a second time to produce the tangled file. 
 (This is very much a "single source file" thing ?)
 
 Weaving would be traversing it, but also outputing nicely formatted code. 
 
 However, interactively building the web would be called... `spinning` !
 
 `dom-web` -> structures for a series of strings that can be interactively added, looked-up, etc... to display a web 
  uc-spin -> primitive actions to spin a web. In particular, parse a single source file to produce the web, or raise errors.
 `uc-tangle` -> from a web, produce a single output source file (or multiple source file ?)
  uc-weave  -> from a web, produce a single md file (no fancy formatting)

  bin-tangle -> cli shell over uc-tangle, report error and read from files
  bin-weave -> cli shell over uc-weave
  bin-spin -> much more ambitious: GUI shell over uc-spin, uc-tangle and uc-weave, to actually edit bits of a web, navigate between sections, display a partially tangled web, etc...
  
  bin-simulator -> in a crazy climax, allow the simulator to load not an assembly file, not an oberon file, but directly... A WEB file, and display the execution web line by web line.
