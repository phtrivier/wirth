use simulator::Execution;
use simulator::Simulator;

use std::path::PathBuf;
use structopt::StructOpt;

// mod examples_tests;

/// Load a binary file (compiled from assembly or oberon-0) and run it in the risc computer
#[derive(StructOpt, Debug)]
#[structopt(name = "cli-risc", version = "0.0.1")]
struct Opt {
    /// Assembly language file
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,

    /// Treat input as Oberon-0 code, and compile it before execution
    #[structopt(short, long)]
    compile: bool,

    /// Maximum number of cycles to run before failing execution
    #[structopt(short = "m", name = "max-cyles", default_value = "99999")]
    execution_max_cycles: u32,

    /// Stack base address when simulating process
    #[structopt(short = "s", name = "stack-base", long, default_value = "1000")]
    execution_stack_base: usize,

    /// Memory position to dump instruction from
    #[structopt(long, default_value = "0")]
    instruction_dump_from: usize,

    /// Number of memory position to dump instruction
    #[structopt(long, default_value = "15")]
    instruction_dump_count: usize,

    /// Memory position to dump data from
    #[structopt(long, default_value = "1000")]
    memory_dump_from: usize,

    /// Number of memory position to dump data
    #[structopt(long, default_value = "15")]
    memory_dump_count: usize,
    // / Debug mode
    // #[structopt(short, long)]
    // debug: bool,
}

#[cfg(not(tarpaulin_include))]
fn dump_regs(s: &Simulator) {
    for (index, reg) in s.registers().iter().enumerate() {
        println!("REG {:04}: 0x{:08X} 0b{:032b} {:12}", index, reg, reg, reg)
    }
}

#[cfg(not(tarpaulin_include))]
fn dump_mem(s: &Simulator, from: usize, count: usize) {
    let memory = s.memory(from, count);
    for (index, m) in memory.iter().enumerate() {
        println!("MEM {:04}: 0x{:08X} 0b{:032b} {:12?}", from + index, m, m, m);
    }
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let opt = Opt::from_args();

    let filename = opt.input.into_os_string().into_string().expect("Filename is malformed.");

    let content = std::fs::read_to_string(filename).expect("Unable to read from input file.");

    let mut simulator = if opt.compile {
        simulator::Simulator::from_oberon(&content).unwrap()
    } else {
        simulator::Simulator::from_assembler(&content).unwrap()
    };

    // Dump before
    println!("After loading program:");

    dump_regs(&simulator);
    println!("--- Instructions ---");
    dump_mem(&simulator, opt.instruction_dump_from, opt.instruction_dump_count);

    // Execute
    println!(">>>>>>");
    println!("Executing program...");

    let success = simulator
        .execute(Execution {
            program_address: 0,
            max_cycles: opt.execution_max_cycles,
            stack_base: opt.execution_stack_base,
        })
        .is_ok();

    println!("<<<<<<");
    println!("After execution:");
    println!("--- Registers ---:");
    dump_regs(&simulator);
    println!("--- Instructions ---");
    dump_mem(&simulator, opt.instruction_dump_from, opt.instruction_dump_count);
    println!("... Memory ---");
    dump_mem(&simulator, opt.memory_dump_from, opt.memory_dump_count);

    if success {
        println!("Program run successfully.");
    } else {
        println!("Warning: execution stopped after {:?} instructions", opt.execution_max_cycles);
        std::process::exit(1);
    }
}
