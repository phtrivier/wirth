use risc::instructions::Instruction::*;
use risc::instructions::OpCode::*;
use risc::instructions::BranchCondition::*;

use assembler::assemble;

#[test]
fn assemble_full_program() {
    let program = "
    * A program that incruments R1 until it's 6
    #FOO    3             ; Number of iterations remaining
    #BAR    2
            MOV  R0,#FOO   ; R0 <- 3
            MOV  R1,0      ; 
    @LOOP   ADD  R1,R1,#BAR   ; R1 <- R1 + 2
            SUB  R0,R0,1      ; R0 <- R0 - 1
            BEQ  @END      ; IF R0 == 0 GOTO @END
    * A comment that should be ignored
            B    @LOOP
    @END    MOV  R2,0      ; Put 0 in the R2, and branch to 0 to exit
            B    R2
    ";
    let instructions = assemble(&program).unwrap();

    assert_eq!(8, instructions.len());

    assert_eq!([
      RegisterIm{ o: MOV, a: 0, b: 0, im: 3},  //
      RegisterIm{ o: MOV, a: 1, b: 0, im: 0},   //
      RegisterIm{ o: ADD, a: 1, b: 1, im: 2},   //
      RegisterIm{ o: SUB, a: 0, b: 0, im: 1},   //
      BranchOff{cond: EQ, link: false, offset: 2},   //
      BranchOff{cond: AW, link: false, offset: -4},  //
      RegisterIm{ o: MOV, a: 2, b: 0, im: 0},      //
      Branch{cond: AW, link: false, c: 2}, //
    ], &instructions[..]);


}