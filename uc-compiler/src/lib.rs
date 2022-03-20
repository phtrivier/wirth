use ast::parser;
use ast::scanner::*;
use ast::scope::*;
use risc::instructions::*;

mod codegen;

// NOTE(pht) alternatively, I might want a "CompileError", when there will be other possible errors...
// or just rename the thing...
pub use ast::parser::ParseError;

pub fn compile(input: &str) -> std::result::Result<Vec<Instruction>, ParseError> {
    let mut scanner = Scanner::new(input);

    let scope = Scope::new();
    // Still needed because `parsemodule` is not yet `compile`
    parser::scan_next(&mut scanner)?;
    let ast = parser::parse_module(&mut scanner, &scope)?;

    let mut codegen = codegen::Codegen::new();
    codegen.generate_code(&ast);

    let mut instructions = codegen.instructions;

    // NOTE(pht) add the two instructions to return, in any case, at the end
    // of the generated code.
    // Or this could be done by codegen ?
    instructions.push(Instruction::RegisterIm {
        o: OpCode::MOV,
        a: 15,
        b: 0,
        im: 0,
    });
    instructions.push(Instruction::Branch {
        cond: BranchCondition::AW,
        c: 15,
        link: false,
    });

    Ok(instructions)
}
