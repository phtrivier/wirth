# Sat 2023-05-13

Played with egui, and happilly surprised ! In my case, immediate gui is pretty
adapted. (Although I would have like to be able to do a fancy version that runs
commands with a bit of a delay, but that woul involve too much synchro already.)

Not sure what the next step, is, though...

# Wed 2023-05-10

Played with relm4 to see how hard a GUI would be. 
The framework seems powerfull, and native, system-like looking guis are nice ;
but it seems overkill for what I have in mind.

# Tue 2023-05-09

Played with slint-rs to generate a UI. A bit too complex, espacially given that their
doc is incomplete.

# Sun 2023-05-07

Got my brain together, wrote some prose, and got the assignment working.
I suppose I have everything to write the list of primes example ?
In all cases, the "list of squares" is trivial !

# Sat 2023-05-06

Yea, almost 5 month laters, I finally start working on arrays index by variants.
The assignment parts work surprinsingly well (`a[i] := x`).

But the other sign of the assigneent does not work ! (`x != a[i]`) 

# Fri 2022-12-23

Basic arrays (with only constant indexing.)

# Sun 2022-12-18

Got basic conditionals working to make more intereseting loops. 

However there is a subtle bug when the body of the loop contains assignements to multiple variables ; it seems like the loop is only really run once...

I'm starting to feel like I would need a... disasembler to print the instructions nicely !!

(How, and, obviously, it was an off-by-one error in the offset of the AW instruction. Which is kinda sucky, but, hey, it works, now ?)

# Thu 2022-12-15

So, this was hard, but we made it through if/then/else. 

The trick was that I generated an AW instruction even if there was no Else, which broke the thing.
Nicely, the fact that I broke the tree into an Else statement seems to make the codegen code
easy. 

Just need to test that additionnal statements after the else are really followed...
... and YES IT DOES !!!

So, now, I can do "sequence", and "selection".
I need to add "iteration" -- because I don't have GOTO, nor recursive functions, so I need WHILE, 
and I'll be done with "something that can compute" !

# Mon 2022-12-12

Still not much progress on the nested if side. 

This will require either spending a lot of time on it, or some lateral thinking. 

I tried simplifying the tree to make the "Else" branch explicit.
What is now obvious is that I don't know at which moment I want to do the fixups ; 

It seems like I want to wait unil "all the ifs" are handled ; but when is that ? 
In Statement sequence ?
   
I tried different version where the the fixups are done at the end of either child or sibling of statement sequence.
But it consistently fails because I get an empty stack :/

Or maybe I want to add a single vec in the statement sequence ? 

Or maybe I find a way to give up entirely on the nested if / then else, and 
add _more_ AW branches that just jump to each other. 

This seems to be the idea behind: 

http://craftinginterpreters.com/jumping-back-and-forth.html

Alternatively, I can just scrap the whole project, do rm -rf, and just implement
the toy compiler in "crafting interpreter".

Or I completely scrap it and instead learn how to use llvm as a backend ? 

Which would at least give me the satisfaction of having something executing in a _real_ computer ?

Maybe I need to stop trying to please Jon Blow & Casey Muratori ?
They won't be there the day I die.

Alternatively, I might have to entertain the idea that maybe the code generation works fine, 
but the code _execution_ fails in the triply nested if / then / else ? 

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
