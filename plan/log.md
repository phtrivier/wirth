# Mon 2022-12-12

Still not much progress on the nested if side. 
I tried simplifying the tree to make the "Else" branch explicit.
What is now obvious is that I don't know at which moment I want to do the fixups ; 

It seems like I want to wait unil "all the ifs" are handled ; but when is that ? 
In Statement sequence ?
   
I tried different version where the the fixups are done at the end of either child or sibling of statement sequence.
But it consistently fails because I get an empty stack :/

Or maybe I want to add a single vec in the statement sequence ? 

# Mon 2022-12-05

Kinda loosing hope about nested logical statements. 
I tried to accumulate a list of indicies of instructions to fixups, but
I can not find the proper place to pop from this list. 
Let's not forget it's just code.

# Fri 2022-12-02

Tried again to make the if / then / else work. Still not working.

# Mon 2022-11-28 21:00-22:00

Implemented the else part.

> Sequence ; Selection ; Iteration.

I CAN DO PROGRAMMING !!! (Except, well, nested if / then / else don't work at all.)

# 2022-03-20 

Implemented most of the logical handling of if / then

# Mon 2021-05-17 18:00-20:00

Scanned, parsed and generated code for a basic IF expression that does some computation.
This feels surreal.
# Sun 2021-05-16 20:00-21:00

Cleaned up the chain of `if let Scan{ .... } = ` in `parse_module`.

# Tue 2021-05-13 18:00-19:00
And, finaly, after learning about closures and accepting to just "pass a function that lets you complete the tree", 
I was able to parse a module that does arithmetic. Now, everything works pretty much as I expected.

# Wed 2021-05-12 13:00-14:00
- For Helene's birthday, the elements gave to me the power of recursion, and understanding
how to produce a `Declarations` tree that contains a list of `Declaration` in the right order.
 
# Tue 2021-05-11 12:00-14:30
- Managed to extract the parse_var_declarations function, and make it recursive.
But calling `scope.add` multiple times (or passing the mutable reference to several functions) still does not work. 
I don't understand what kind of bug Rust is trying to protect me from, and
I can't find a minimal example to explain the problem. 

Also, where do I ask this question ?

# Mon 2021-05-10 18:00-19:00
- Wasted an hour trying to refactor the function to parse var_declaration.
Again, I'm missing something about the lifetime of stuff, because
the compiler tells me my naive version makes the data "owned" by the parser... not sure why... 

# Sun 2021-05-09 14:00-18:00
- Parsing declaration. Learned about loop imutability, though I'm not 100% sure I understood
everything...

# Sun 2021-05-09 11:00-12:45
- Scanning declaration, starting parsing them
# Fri 2021-05-06 17:00-18:00
- Finished arithmetic, including codegen and parenthesized expressions.
Crazy how things go well when you let yourself "do the things that seems like the right solution.)
# Thu 2021-05-06 19:00-20:00
- Refactored current to use Rc<Scan> everywhere.
- Started arithmetic (factor and terms), I'm getting slighly ambigous trees, but that's okay.
Parens will resolve things.
# Wed 2021-05-05 18:00-20:00
- Finally caved in, and extracted an (ugly) function that gets the 'current' token.
Normally, I now have what I need to parse an expression without loosing too much of my brain.
# Tue 2021-05-04 13:00-14:00
- Used uc-simulator in bin-simulator and bin-simulator-gui
- Added extra command line options and a dump of memory in the gui
Shuffling done, now on to parsing more things !
# Mon 2021-05-03 19:00-20:15
- Extracted uc-simulator and moved assembler tests to the right place
# Sat 2021-05-01 18:30-20:30
- Extracted binary crates and added arguments.
The whole chain of compiler -> simulate or assembler -> simulate now works...
# Fri 2021-04-30 17:30-18:00
- more code shuffling, but my arm hurts...
# Thu 2021-04-29 18:00-18:45
- Done part of the shuffling, but my head hurts and I'm loosing patience because of 
  passing Read / Write. At least that's something to learn !
# Thu 2021-04-29 13:00-14:00
- Yak-shaving thinking about code reorganisation... 
# Wed 2021-04-28 20:00-20:45
- Loaded generated binary code into cli-risc
Full loop is now done, but organisation needs to be cleaned up.
# Wed 2021-04-28 13:30-14:00
- Introduced some helper functions for tree
- Added possibility to parse ident := ident
# Tue 2021-04-27 18:00-20:00
- Introduced codegen for basic assignements to contants
# Mon 2021-04-26 12:30-13:30
- Updated example programs in cli-risc to computer-2017
- Added a MOD instruction because otherwise nothing was working
# Sun 2021-04-25 19:00-23:00
- Updated assember
# Sun 2021-04-25 15:00-18:00
- Updated computer
# Sat 2021-04-24 19:00-23:00
- Extracted a parse tree for assignment
# Sat 2021-04-24 15:00-18:00
- Added scan errors
