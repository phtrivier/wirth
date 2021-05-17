use risc::instructions::Instruction;

use std::path::PathBuf;
use structopt::StructOpt;

/// Load a binary file (compiled from assembly or oberon-0) and run it in the risc computer
#[derive(StructOpt, Debug)]
#[structopt(name = "cli-risc", version = "0.0.1")]
struct Opt {
    /// Assembly language file
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    let opt = Opt::from_args();
    let filename = opt.input.into_os_string().into_string().expect("Filename is malformed.");

    let mut content = std::fs::read_to_string(&filename).expect(&format!("Unable to open file {:?}", filename));

    match compiler::compile(&mut content) {
        Ok(instructions) => {
            let encoded = Instruction::serialize_all(instructions);
            std::fs::write("out.o", &encoded[..]).expect("Unable to write output to file");
        }
        Err(err) => {
            println!("Compilation error: {:?}", err);
            std::process::exit(-1);
        }
    }
}
