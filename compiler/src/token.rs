
// I must be stupid, because I simply can't make rust find
// and import this module when I put it in a nother file.
// I simply don't understand. 
// use crate::token::Token should work. 
// It does not. 

// <@scanner/tokens
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Times,
    Div,
    Mod,
    And,
    Plus,
    Minus,
    Or,
    Eql,
    Neq,
    Lss,
    Geq,
    Leq,
    Gtr,
    Period,
    Comma,
    Colon,
    Rparen,
    Rbrak,
    Of,
    Then,
    Do,
    Lparen,
    Lbrak,
    Not,
    Becomes,
    Semicolon,
    End,
    Else,
    Elsif,
    If,
    While,
    Array,
    Record,
    Const,
    Type,
    Var,
    Procedure,
    Begin,
    Module,
    False,
    True,
    Repeat,
    Until,
    Number(u32),
    Ident(String),
}
// @>scanner/tokens

// <@scanner/scan
#[derive(Clone, PartialEq, Debug)]
pub struct Scan {
  pub line_number: u32,
  pub column_number: u32,
  pub token: Token,
}
// @>scanner/scan

