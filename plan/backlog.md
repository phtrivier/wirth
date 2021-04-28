
# Backlog

## gui-risc

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

## misc

  - [ ] Introduce an actual logger crate ?
  - [ ] Rename `risc` crate to simulator
  - [ ] Support negative memory for Input / Output in cli
