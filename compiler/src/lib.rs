mod parser;
mod parser_test;
mod scanner;

pub fn run() -> Result<(), parser::ParseError> {
    // let content = String::from("1 + 1");
    let content = String::from(".Bar[0]");
    return parser::Parser::parse(&content);
}
