use assembler;
use risc;
use std::fs;
use clap::{App};

fn main() {
  let matches = App::new("cli-risc")
    .version("0.0")
    .author("Pierre-Henri Trivier <phtrivier@yahoo.fr>")
    .about("Execute assembly language programs for the RISC computer based on Wirth book.")
    .arg("<INPUT>              'Sets the input file to use'")
    .get_matches();

  let filename = matches.value_of("INPUT").unwrap();
  let program = fs::read_to_string(filename).expect("Unable to read file.");

  // Assemble a program
  let mut a = assembler::Assembler::new();
  a.assemble(&program).expect("Unable to parse program !");
  println!("{:?}", a.instruction_indexes);

  // Load instructions
  let mut c = risc::computer::Computer::new();
  c.load_instructions(a.instructions);

  // Dump before
  println!("After loading program:");
  c.dump_regs();
  c.dump_mem(0, 15);

  // Execute
  println!("Executing program...");
  c.execute(50);

  // Dump after
  println!("After execution:");
  c.dump_regs();
  c.dump_mem(0, 15);
  println!("...");
  c.dump_mem(30, 40);

  // Success !
  println!("Value of the Accu R0: {}", c.regs[0]);
}
