mod scanner;
mod parser;

use parser::Parser;

pub fn run() -> Result<(), parser::ParseError>{
    let content = String::from("1 + 1");
    return Parser::parse(&content);
}
