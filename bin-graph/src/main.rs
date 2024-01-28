use pretty_env_logger;

use log::debug;
use std::path::PathBuf;
use structopt::StructOpt;

/// Load an oberon file and ouputs a graphviz file
#[derive(StructOpt, Debug)]
#[structopt(name = "bin-graph", version = "0.0.1")]
struct Opt {
    /// Assembly language file
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,
}

#[cfg(not(tarpaulin_include))]
fn main() {
    use bin_graph::to_dot;

    pretty_env_logger::init();

    let opt = Opt::from_args();
    let filename = opt.input.into_os_string().into_string().expect("Filename is malformed.");

    let content = std::fs::read_to_string(&filename).unwrap_or_else(|_| panic!("Unable to open file {:?}", filename));

    match compiler::build_ast(&content) {
        Ok(ast) => {
            debug!("Built ast {:?}", ast);

            println!("{:}", to_dot(&ast));

        }
        Err(err) => {
            println!("Parsing error: {:?}", err);
            std::process::exit(-1);
        }
    }
}
