use std::iter::Peekable;
use std::str::CharIndices;

use crate::token::*;

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct LineScanner<'a> {
    line_number: u32,
    column_number: u32,
    chars: Peekable<CharIndices<'a>>,
    pub current: Option<Rc<Scan>>,
}

impl LineScanner<'_> {
    pub fn new(line_number: u32, line: &str) -> LineScanner {
        LineScanner {
            line_number,
            column_number: 0,
            chars: line.char_indices().peekable(),
            current: None,
        }
    }

    pub fn current(&mut self) -> Option<Rc<Scan>> {
        self.current.as_ref().cloned()
    }

    fn context(&self, column_number: u32) -> ScanContext {
        ScanContext {
            line: self.line_number,
            column: column_number,
        }
    }

    fn forward(&mut self) {
        self.chars.next();
        self.column_number += 1;
    }

    fn token_at(&mut self, column: usize, token: Token) -> Option<ScanResult> {
        let scan = Rc::new(Scan {
            context: self.context(column as u32),
            token,
        });

        self.current = Some(scan.clone());

        Some(Ok(scan))
    }

    fn error_at(&self, column: usize, error_type: ScanErrorType) -> Option<ScanResult> {
        Some(Err(ScanError {
            context: self.context(column as u32),
            error_type,
        }))
    }

    fn scan_single(&mut self, column: usize, token: Token) -> Option<ScanResult> {
        self.chars.next();
        self.token_at(column, token)
    }

    fn skip_whitespaces(&mut self) -> Option<ScanResult> {
        loop {
            let p = self.chars.peek();
            if let Some(&(_column, next_char)) = p {
                if next_char == ' ' || next_char == '\t' {
                    self.forward();
                    continue;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        self.next()
    }

    fn scan_word(&mut self, column: usize) -> Option<ScanResult> {
        let mut ident = String::from("");
        loop {
            let p = self.chars.peek();
            if let Some(&(_column, next_char)) = p {
                if next_char.is_ascii_alphabetic() {
                    ident.push(next_char);
                    self.forward();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        match &ident.to_ascii_lowercase()[..] {
            "var" => self.token_at(column, Token::Var),
            "module" => self.token_at(column, Token::Module),
            "begin" => self.token_at(column, Token::Begin),
            "end" => self.token_at(column, Token::End),
            "if" => self.token_at(column, Token::If),
            "then" => self.token_at(column, Token::Then),
            "else" => self.token_at(column, Token::Else),
            "elsif" => self.token_at(column, Token::Elsif),
            "while" => self.token_at(column, Token::While),
            "do" => self.token_at(column, Token::Do),
            "array" => self.token_at(column, Token::Array),
            "of" => self.token_at(column, Token::Of),
            _ => self.token_at(column, Token::Ident(ident)),
        }
    }

    fn scan_integer(&mut self, column: usize) -> Option<ScanResult> {
        let mut n: u32 = 0;
        loop {
            let p = self.chars.peek();
            if let Some(&(_column, next_char)) = p {
                if let Some(d) = next_char.to_digit(10) {
                    n = n * 10 + d;
                    self.forward();
                    continue;
                    // TODO(pht) replace with single 'break' here ?
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        self.token_at(column, Token::Int(n))
    }

    fn scan_sigil(&mut self, column: usize, first_char: char) -> Option<ScanResult> {
        self.forward();
        let p = self.chars.peek();
        match first_char {
            ':' => {
                if let Some(&(_column, '=')) = p {
                    self.forward();
                    self.token_at(column, Token::Becomes)
                } else {
                    self.token_at(column, Token::Colon)
                }
            }
            '>' => {
                if let Some(&(_column, '=')) = p {
                    self.forward();
                    self.token_at(column, Token::Geq)
                } else {
                    self.token_at(column, Token::Gtr)
                }
            }
            '<' => {
                if let Some(&(_column, '=')) = p {
                    self.forward();
                    self.token_at(column, Token::Leq)
                } else {
                    self.token_at(column, Token::Lss)
                }
            }
            _ => {
                panic!(
                    "Programmer error: function `LineScanner::scan_sigil` called with character `{:?}` that does not start a sigil.",
                    first_char
                );
            }
        }
    }

    fn skip_comment(&mut self, column: usize) -> Option<ScanResult> {
        loop {
            let mut p = self.chars.peek();
            if p.is_none() {
                return self.error_at(column, ScanErrorType::UnterminatedComment);
            }
            if let Some(&(_column, '*')) = p {
                self.forward();
                p = self.chars.peek();
                if let Some(&(_column, ')')) = p {
                    self.forward();
                    return self.next();
                }
            }
            self.forward();
        }
    }
}

impl Iterator for LineScanner<'_> {
    type Item = ScanResult;

    fn next(&mut self) -> Option<ScanResult> {
        let mut peek = self.chars.peek();
        match peek {
            Some(&(_column, c)) if (c == ' ' || c == '\t') => self.skip_whitespaces(),
            Some(&(column, '\n')) => {
                self.chars.next();
                self.error_at(column, ScanErrorType::UnexpectedNewLine)
            }
            Some(&(column, c)) if !c.is_ascii() => {
                self.chars.next();
                self.error_at(column, ScanErrorType::InvalidChar(c))
            }

            Some(&(column, c)) if c.is_numeric() => self.scan_integer(column),
            Some(&(column, ':')) => self.scan_sigil(column, ':'),
            Some(&(column, '>')) => self.scan_sigil(column, '>'),
            Some(&(column, '<')) => self.scan_sigil(column, '<'),
            Some(&(column, ';')) => self.scan_single(column, Token::Semicolon),
            Some(&(column, ',')) => self.scan_single(column, Token::Comma),
            Some(&(column, '(')) => {
                self.forward();
                peek = self.chars.peek();
                println!("peek {:?}", peek);
                if let Some(&(_column, '*')) = peek {
                    self.skip_comment(column)
                } else {
                    self.token_at(column, Token::Lparen)
                }
            }
            Some(&(column, ')')) => self.scan_single(column, Token::Rparen),
            Some(&(column, '+')) => self.scan_single(column, Token::Plus),
            Some(&(column, '-')) => self.scan_single(column, Token::Minus),
            Some(&(column, '*')) => self.scan_single(column, Token::Times),
            Some(&(column, '/')) => self.scan_single(column, Token::Div),
            Some(&(column, '.')) => self.scan_single(column, Token::Period),
            Some(&(column, '=')) => self.scan_single(column, Token::Eql),
            Some(&(column, '#')) => self.scan_single(column, Token::Neq),
            Some(&(column, '[')) => self.scan_single(column, Token::Lbrak),
            Some(&(column, ']')) => self.scan_single(column, Token::Rbrak),
            Some(&(column, _first_char)) => self.scan_word(column),
            None => {
                self.current = None;
                None
            }
        }
    }
}
