#![feature(assert_matches)]
use simulator::Simulator;
use simulator::*;

#[test]
fn assembled_code() {
        let program = "
    * A program that incruments R1 until it's a certain value
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

    let mut s = Simulator::from_assembler(program).unwrap();
        let execution = Execution {
        program_address: 0,
        stack_base: 100,
        max_cycles: 50,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.registers()[1], 6);
}
