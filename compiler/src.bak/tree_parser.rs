// use crate::scanner::ScanError;
// use crate::scanner::Scanner;
// use crate::scanner::Token;

// // NOTE(pht) I have to make this copyable otherwise, returning a ParseError as the
// // right side of the error is cumbersome.
// // But copying should be okay for this.
// #[derive(Debug, Clone)]
// pub enum ParseError {
//     ScanError(ScanError),
//     UnexpectedToken, // Not(pht) I can't make ParseError Copy, because String can not be Copy
//     PrematureEOF,
// }

// pub enum Object {
//     Const{value: u32},
//     Var{adr: u32, level: u32}, // No idea what Var means here !
//     // Ident{name: String}, // NOTE(pht) : this will have to refer to some sort of hashmap at some point
// }

// pub enum NodeOp {
//     None
// }

// pub enum Tree {
//     Node{op: NodeOp,
//         object: Object,
//         left: Box<Tree>,
//         right: Box<Tree>},
//     Nil
// }

// impl Tree {
//     pub fn constant(op: NodeOp, v: u32) -> Tree {
//         Tree::Node{
//             op: op,
//             object: Object::Const{value: v},
//             left: Box::new(Tree::Nil),
//             right: Box::new(Tree::Nil)
//         }
//     }
// }


// // TODO(pht) Using 'Results' for the intermediate step if a bit of a problem.
// // Seems like all functions really want to return void, and keep a state with
// // the current token , and a potential error.
// pub struct TreeParser<'a> {
//     scanner: Box<Scanner<'a>>,
//     token: Option<Token>,
//     done: bool,
// }

// impl TreeParser<'_> {
//     // pub fn parse<'a>(content: &'a str) -> Result<(), ParseError> {
//     //     match TreeParser::from_string(&content) {
//     //         Ok(mut parser) => {
//     //             // TODO(pht) this parses the "root" element.
//     //             // In the finished version, it will be module(), I suppose ?
//     //             parser.statement_sequence();
//     //             return parser.result();
//     //         }
//     //         Err(e) => Err(e),
//     //     }
//     // }

//     pub fn from_string<'a>(content: &'a str) -> Result<TreeParser<'a>, ParseError> {
//         let mut scanner = Box::new(Scanner::new(&content));
//         let first = scanner.scan();
//         return match first {
//             Ok(Some(token)) => {
//                 // println!("Initial token {:?}", token);
//                 return Ok(TreeParser {
//                     scanner: scanner,
//                     token: Some(token),
//                     done: false
//                 });
//             }
//             Ok(None) => Err(ParseError::PrematureEOF),
//             // TODO(pht) find a way to associate the ScanError to the parse error, otherwise it's lost :/
//             Err(scan_error) => Err(ParseError::ScanError(scan_error)),
//         };
//     }

//     // pub fn result(&mut self) -> Result<(), ParseError> {
//     //     if let Some(e) = self.error {
//     //         return Err(e);
//     //     }
//     //     if !self.done {
//     //         return Err(ParseError::UnexpectedToken);
//     //     }
//     //     return Ok(());
//     // }

//     fn next<'a>(&mut self) -> Result<(), ParseError>{
//         match self.scanner.scan() {
//             Ok(Some(token)) => {
//                 println!("Next token {:?}", token);
//                 self.token = Some(token);
//                 return Ok(());
//             }
//             Ok(None) => {
//                 self.token = None;
//                 self.done = true;
//                 return Ok(());
//             }
//             Err(scan_error) => {
//                 return Err(ParseError::ScanError(scan_error))
//             }
//         }
//     }

//     pub fn factor(&mut self) -> Result<Tree, ParseError> {
//         match self.token {
//             // Token::Ident(_i) => {
//             //     self.next();
//             //     self.selector();
//             // }
//             Some(Token::Int{v: n}) => {
//                 self.next()?;
//                 return Ok(Tree::constant(NodeOp::None, n));
//             }
//             // Token::Lparen => {
//             //     self.next();
//             //     self.expression();

//             //     if let Token::Rparen = self.token {
//             //         self.next();
//             //     } else {
//             //         self.error = Some(ParseError::UnexpectedToken);
//             //     }
//             // }
//             // Token::Not => {
//             //     self.next();
//             //     self.factor();
//             // }
//             _ => {
//                 return Err(ParseError::UnexpectedToken)
//                 // self.error = Some(ParseError::UnexpectedToken);
//             }
//         }
//     }

// }

// #[cfg(test)]
// mod tests {

//     use super::*;

//     fn tree_parser<'a>(s: &'a str) -> TreeParser<'a> {
//         return TreeParser::from_string(&s).unwrap();
//     }

//     // fn assert_parses(p: &mut TreeParser) {
//     //     assert!(p.result().is_ok(), "Parsing error: {:?}", p.result());
//     // }

//     // #[test]
//     // fn test_selector() {
//     //     for c in [".y", ".[0]"].iter() {
//     //         let mut p = parser(c);
//     //         p.selector();
//     //         assert_parses(&mut p);
//     //     }
//     // }

//     #[test]
//     fn test_factor() {
//         let mut p = tree_parser("1");
//         let f = p.factor().unwrap();
//         // TODO(pht) assert f is a Tree
//     }
// }
