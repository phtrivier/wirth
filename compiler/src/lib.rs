// mod parser;
mod scanner;
mod tree_parser;


pub fn run() -> () {
    // let content = String::from("1 + 1");
/*    let content = String::from(".Bar[0]");
    return parser::Parser::parse(&content);
    */
    let content = String::from(".Bar[0]");
    let _p = tree_parser::TreeParser::from_string(&content);
}
