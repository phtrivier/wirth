use compiler;

/**
 * A sort of Oberon Compiler
 */

fn main() {
    match compiler::run() {
        Ok(_) => println!("Parsing successful."),
        Err(err) => println!("Parsing error {:?}", err)
    }
}
