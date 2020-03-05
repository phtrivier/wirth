use std::str::Chars;

#[derive(Debug)]
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
    Leq,
    Gtr,
    Geq,
    Period,
    Int(u32),
    False,
    True,
    Not,
    Lparen,
    Rparen,
    Lbrak,
    Rbrak,
    Ident(String),
    If,
    While,
    Repeat,
    Comma,
    Colon,
    Becomes,
    Then,
    Of,
    Do,
    Semicolon,
    End,
    Else,
    Elsif,
    Until,
    Array,
    Record,
    Const,
    Type,
    Var,
    Procedure,
    Begin,
    Module,
}

pub enum ScanError {
    UnfinishedComment(),
    PrematureEof(u32),
    InvalidChar(char, u32)
}

pub struct Scanner<'a> {
    line: u32,
    peek: Option<char>,
    chars: Chars<'a>,
}

impl Scanner<'_> {
    pub fn new<'a>(content: &'a String) -> Scanner<'a> {
        let mut chars = content.chars();
        let peek = chars.next();
        Scanner{
            line: 1,
            peek: peek,
            chars: chars
        }
    }

    pub fn line(&self) -> u32 {
        return self.line;
    }

    fn read_next(&mut self) -> () {
        self.peek = self.chars.next();
    }

    fn is_single_special_char(&self, c: char) -> bool {
        match c {
            '&' | '+' | '-' | '=' | '#' | ')' | '[' | ']' | ',' | ';' | '.'  | '~' | '*' => true,
            _ => false
        }
    }

    fn is_first_special_char_of_combination(&self, c: char) -> bool {
        match c {
            '<' | '>' | ':' | '(' => true,
            _ => false
        }
    }

    fn skip_whitespace(&mut self) -> Result<Option<Token>, ScanError> {
        self.read_next();
        return self.scan();
    }

    fn skip_newline(&mut self) -> Result<Option<Token>, ScanError> {
        self.line = self.line + 1;
        self.read_next();
        return self.scan();        
    }

    fn scan_number(&mut self, d: char) -> Result<Option<Token>, ScanError> {
        let mut v = d.to_digit(10).unwrap();
        loop {
            self.read_next();
            match self.peek {
                Some(d) => {
                    match d.to_digit(10) {
                        Some(n) => {
                            v = 10*v + n;
                        }
                        _ => { break; }
                    }
                },
                None => { break; }
            }
        }
        return Ok(Some(Token::Int(v)));
    }

    fn skip_comment(&mut self) -> Result<Option<Token>, ScanError> {
        loop {
            self.read_next();
            match self.peek{
                Some('*') => {
                    self.read_next();
                    match self.peek {
                        Some(')') => {
                            self.read_next();
                            return self.scan()
                        },
                        Some(_) => {
                            continue;
                        }
                        None => {
                            return Err(ScanError::UnfinishedComment());
                        }
                    }
                },
                Some(_) => {
                    continue;
                },
                None => {
                    return Err(ScanError::UnfinishedComment());
                }
            }
        }
    }

    pub fn scan<'a>(&mut self) ->  Result<Option<Token>, ScanError> {
        match self.peek {
            Some(s) if s == ' ' || s == '\t' => {
                return self.skip_whitespace()
            },
            Some(nl) if nl == '\n' => {
                return self.skip_newline()
            },
            Some(d) if d.is_digit(10) => {
                return self.scan_number(d)
            },
            Some(c) if c.is_alphanumeric()=> {
                let mut w = String::from(c.to_string());
                loop {
                    self.read_next();
                    match self.peek {
                        Some(c) if c.is_alphanumeric() => {
                            w.push(c);
                        }
                        _ => { break; }
                    }
                }

                let token = match w.as_str() {
                    "DIV" => Some(Token::Div),
                    "MOD" => Some(Token::Mod),
                    "OR" => Some(Token::Or),
                    "OF" => Some(Token::Of),
                    "THEN" => Some(Token::Then),
                    "DO" => Some(Token::Do),
                    "UNTIL" => Some(Token::Until),
                    "END" => Some(Token::End),
                    "ELSE" => Some(Token::Else),
                    "ELSIF" => Some(Token::Elsif),
                    "IF" => Some(Token::If),
                    "WHILE" => Some(Token::While),
                    "REPEAT" => Some(Token::Repeat),
                    "ARRAY" => Some(Token::Array),
                    "RECORD" => Some(Token::Record),
                    "CONST" => Some(Token::Const),
                    "TYPE" => Some(Token::Type),
                    "VAR" => Some(Token::Var),
                    "PROCEDURE" => Some(Token::Procedure),
                    "BEGIN" => Some(Token::Begin),
                    "MODULE" => Some(Token::Module),
                    "TRUE" => Some(Token::True),
                    "FALSE" => Some(Token::False),
                    _ => Some(Token::Ident(w))
                };
                return Ok(token)
            },
            Some(c) if self.is_single_special_char(c) => {
                let result = match c {
                    '&' => Ok(Some(Token::And)),
                    '+' => Ok(Some(Token::Plus)),
                    '-' => Ok(Some(Token::Minus)),
                    '=' => Ok(Some(Token::Eql)),
                    '#' => Ok(Some(Token::Neq)),
                    '*' => Ok(Some(Token::Times)),
                    '.' => Ok(Some(Token::Period)),
                    ',' => Ok(Some(Token::Comma)),
                    ';' => Ok(Some(Token::Semicolon)),
                    ')' => Ok(Some(Token::Rparen)),
                    ']' => Ok(Some(Token::Rbrak)),
                    '[' => Ok(Some(Token::Lbrak)),
                    '~' => Ok(Some(Token::Not)),
                    _ => Err(ScanError::InvalidChar(c, self.line()))
                };
                self.read_next();
                return result;
            },
            Some(c) if self.is_first_special_char_of_combination(c) => {
                self.read_next();

                match (c, self.peek) {
                    ('(', Some('*')) => {
                        return self.skip_comment()
                    },
                    ('(', _) => {
                        return Ok(Some(Token::Lparen)) 
                    },
                    ('>', Some('=')) => {
                        self.read_next();
                        return Ok(Some(Token::Geq));
                    },
                    ('>', _) => {
                        return Ok(Some(Token::Gtr));
                    },
                    ('<', Some('=')) => {
                        self.read_next();
                        return Ok(Some(Token::Leq));
                    },
                    ('<', _) => {
                        return Ok(Some(Token::Lss));
                    },
                    (':', Some('=')) => {
                        self.read_next();
                        return Ok(Some(Token::Becomes));
                    },
                    (':', _) => {
                        return Ok(Some(Token::Colon));
                    }
                    _ => panic!("#2 Unexpected char {:?}", self.peek)
                };
            },
            _ => Ok(None)
        } 
    }
}
