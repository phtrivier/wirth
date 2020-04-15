// mod scanner;
// mod parser;

// use scanner::Scanner;
// use parser::Parser;

// use std::env;
// use std::io::BufReader;
// use std::fs::File;
// use std::io::prelude::*;

/**
 * A sort of Oberon Compiler
 */

// fn parse_content(content: String) {
    /*
  let mut scanner = Scanner::new(&content);

  let mut scanned = scanner.scan();
  loop {
    match scanned {
      Ok(None) => {
        break;
      }
      Ok(Some(token)) => {
        println!("Line {} - Token: {:?}", scanner.line(), token);
        scanned = scanner.scan();
      }
      Err(err) => {
        println!("Line {} - Scanning error: {:?}", scanner.line(), err);
        std::process::exit(-1)
      }
    }
}*/

    // const content = String::from("1")

    // match Parser::parse(content) {
    //     Ok(_) => println!("Parsing successful");
    //     Err(err) => println!("Parsing error: {:?}"; err)
    // }

//}

fn main() {
    // let args: Vec<String> = env::args().collect();
    // if args.len() != 2 {
    //   println!("Usage:");
    //   println!("  wirth [FILE]"); 
    //   println!("");
    //   println!("Missing argument [FILE]");
    //   std::process::exit(-1);
    // }
    // let filename = &args[1];
    // let file = File::open(filename).expect("Unable to open file");
    // let mut buf_reader = BufReader::new(file);
    // let mut contents = String::new();
    // buf_reader.read_to_string(&mut contents).expect("Unable to read file content");
    // parse_content(contents);

    match wirth::run() {
        Ok(_) => println!("Parsing successful."),
        Err(err) => println!("Parsing error {:?}", err)
    }
}
