
use std::rc::Rc;

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Times,
    Div,
    // Mod,
    // And,
    Plus,
    Minus,
    // Or,
    // Eql,
    // Neq,
    // Lss,
    // Geq,
    // Leq,
    // Gtr,
    Period,
    Comma,
    Colon,
    Rparen,
    // Rbrak,
    // Of,
    // Then,
    // Do,
    Lparen,
    // Lbrak,
    // Not,
    Becomes,
    Semicolon,
    End,
    // Else,
    // Elsif,
    // If,
    // While,
    // Array,
    // Record,
    // Const,
    // Type,
    Var,
    // Procedure,
    Begin,
    Module,
    // False,
    // True,
    // Repeat,
    // Until,
    
    Int(u32),
    Ident(String),
}
// @>scanner/tokens

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ScanContext {
  pub line: u32,
  pub column: u32
}

// <@scanner/scan
#[derive(Clone, PartialEq, Debug)]
pub struct Scan {
  pub context: ScanContext,
  pub token: Token,
}

#[derive(Clone, PartialEq, Debug)]
pub enum ScanErrorType {
  InvalidChar(char), // char is not ascii
  UnexpectedNewLine,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ScanError {
  pub context: ScanContext,
  pub error_type: ScanErrorType
}

pub type ScanResult = Result<Rc<Scan>, ScanError>;