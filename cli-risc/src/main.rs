use assembler;
use risc;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

mod examples_tests;

fn load_assembly_file(filename: &str, debug: bool) -> risc::computer::Computer {

    let program = fs::read_to_string(filename).expect("Unable to read file.");

    // Assemble a program
    let mut a = assembler::Assembler::new();
    a.assemble(&program).expect("Unable to parse program !");
    if debug {
        println!("{:?}", a.instruction_indexes);
    }

    // Load instructions
    let mut c = risc::computer::Computer::new();
    c.load_instructions(a.instructions);

    return c;
}

/// Execute assembly file in the RISC-wirth computer
#[derive(StructOpt, Debug)]
#[structopt(name = "cli-risc", version = "0.0.1")]
struct Opt {
    /// Assembly language file
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,

    /// Maximum number of instructions to run
    #[structopt(short = "l", long, default_value = "500")]
    instruction_limit: u32,

    /// Memory position to dump instruction from
    #[structopt(long, default_value = "0")]
    instruction_dump_from: usize,

    /// Number of memory position to dump instruction
    #[structopt(long, default_value = "15")]
    instruction_dump_count: usize,

    /// Memory position to dump data from
    #[structopt(long, default_value = "30")]
    memory_dump_from: usize,

    /// Number of memory position to dump data
    #[structopt(long, default_value = "15")]
    memory_dump_count: usize,

    /// Debug mode
    #[structopt(short, long)]
    debug: bool,
}

fn main() {
    let opt = Opt::from_args();

    let filename = opt.input.into_os_string().into_string().expect("Filename is malformed.");
    let mut c = load_assembly_file(&filename, opt.debug);

    let limit = opt.instruction_limit;

    // Dump before
    println!("After loading program:");
    c.dump_regs();
    println!("--- Instructions ---");
    c.dump_mem(opt.instruction_dump_from, opt.instruction_dump_count);

    // Execute
    println!(">>>>>>");
    println!("Executing program...");
    c.execute(limit, opt.debug);
    println!("<<<<<<");

    // Dump after
    println!("After execution:");
    c.dump_regs();
    println!("--- Instructions ---");
    c.dump_mem(opt.instruction_dump_from, opt.instruction_dump_count);
    println!("... Memory ---");
    c.dump_mem(opt.memory_dump_from, opt.memory_dump_count);

    if c.pc == 0 {
        println!("Program run successfully.");
    } else {
        println!("Warning: execution stopped after {:?} instructions", limit);
        std::process::exit(1);
    }
}
