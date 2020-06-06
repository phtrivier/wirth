use risc;
use assembler;

fn main() {

    // Assemble a program
    let mut a = assembler::Assembler::new();

    let program = "
    * A program that increments R0 until it's 3
    #FOO    3            ; The number of iterations
            MOVI R0,0,0  ; R0 <- 0
    @LOOP   ADDI R0,R0,1  ; R0 <- R0 + 1
            CMPI R0,#FOO ; If iteration done ?
    * Note that the loop is treated as a displacement
            BNE  @LOOP      
    @END    RET  R14    ; Exits since R14 is null in our case
    ";
    a.assemble(program).expect("Unable to parse program !");

    // Load instructions
    let mut c = risc::computer::Computer::new();
    c.load_instructions(a.instructions);

    // Dump before
    println!("After loading program:");
    c.dump_regs();
    c.dump_mem(0, 15);

    // Execute
    println!("Executing program...");
    c.execute(10);

    // Dump after
    println!("After execution:");
    c.dump_regs();
    c.dump_mem(0, 15);

    // Success !
    println!("Value of the Accu R0: {}", c.regs[0]);
}
