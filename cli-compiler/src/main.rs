
use risc::instructions::*;
use compiler::scanner::*;
use compiler::parser::*;
use compiler::scope::*;

use codegen::*;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let content = String::from("x:=42;y:=x");

    let mut scanner = Scanner::new(&content);
    let parser = Parser::new();

    let mut scope = Scope::new();
    scope.add("x");
    scope.add("y");
    
    let ast = parser.parse_statement_sequence(&mut scanner, &scope).unwrap();

    let mut codegen = Codegen::new();

    codegen.generate_code(&ast);

    let instruction_bits : Vec<u32> = codegen.instructions.iter().map(|instruction| {
        return Instruction::encode(instruction);
    }).collect();


    let encoded: Vec<u8> = bincode::serialize(&instruction_bits).unwrap();

    let mut output_file = File::create("out.o")?;
    output_file.write_all(&encoded)?;

    return Ok(());
}
