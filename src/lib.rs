mod scanner;
mod parser;

use parser::Parser;

pub fn run() -> Result<(), parser::ParseError>{
    // let content = String::from("1 + 1");
    let content = String::from(".Bar[0]");
    return Parser::parse(&content);
}
