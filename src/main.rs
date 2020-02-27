use std::str::Chars;

/**
 * A sort of Oberon Compiler
 */

#[derive(Debug)]
enum Token {
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
    Module
}

struct Scanner<'a> {
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

    fn skip_whitespace(&mut self) -> Option<Token> {
        self.read_next();
        return self.scan();
    }

    fn skip_newline(&mut self) -> Option<Token> {
        self.line = self.line + 1;
        self.read_next();
        return self.scan();        
    }

    fn scan_number(&mut self, d: char) -> Option<Token> {
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
        return Some(Token::Int(v));
    }

    fn skip_comment(&mut self) -> Option<Token> {
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
                            panic!("Unfinished comment");
                        }
                    }
                },
                Some(_) => {
                    continue;
                },
                None => {
                    panic!("Unfinished comment")
                }
            }
        }
    }

    pub fn scan<'a>(&mut self) -> Option<Token> {
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

                return match w.as_str() {
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
            },
            Some(c) if self.is_single_special_char(c) => {
                let result = match c {
                    '&' => Some(Token::And),
                    '+' => Some(Token::Plus),
                    '-' => Some(Token::Minus),
                    '=' => Some(Token::Eql),
                    '#' => Some(Token::Neq),
                    '*' => Some(Token::Times),
                    '.' => Some(Token::Period),
                    ',' => Some(Token::Comma),
                    ';' => Some(Token::Semicolon),
                    ')' => Some(Token::Rparen),
                    ']' => Some(Token::Rbrak),
                    '[' => Some(Token::Lbrak),
                    '~' => Some(Token::Not),
                    _ => panic!("#1 Unexpected char {:?}", c)
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
                       return Some(Token::Lparen) 
                    },
                    ('>', Some('=')) => {
                        self.read_next();
                        return Some(Token::Geq);
                    },
                    ('>', _) => {
                        return Some(Token::Gtr);
                    },
                    ('<', Some('=')) => {
                        self.read_next();
                        return Some(Token::Leq);
                    },
                    ('<', _) => {
                        return Some(Token::Lss);
                    },
                    (':', Some('=')) => {
                        self.read_next();
                        return Some(Token::Becomes);
                    },
                    (':', _) => {
                        return Some(Token::Colon);
                    }
                    _ => panic!("#2 Unexpected char {:?}", self.peek)
                };

            },
            _ => None
        } 
    }
}

fn main() {

    let raw_content = r#"
    (* A sample of Oberon code *)
    MODULE Samples;

     (* Multiply three integers together *)
     PROCEDURE Multiply*;
       VAR x, y, z: INTEGER;
     BEGIN OpenInput; ReadInt(x); ReadInt(y); z := 0;
       WHILE x > 0 DO
         IF x MOD 2 = 1 THEN z := z + y END ;
         y := 2*y; x := x DIV 2
       END ;
       WriteInt(x, 4); WriteInt(y, 4); WriteInt(z, 6); WriteLn
     END Multiply;
    END Samples;
    "#;

    let content = String::from(raw_content);

    let mut scanner = Scanner::new(&content);

    let mut token = scanner.scan();
    while !token.is_none() {
        println!("Line {} - Token: {:?}", scanner.line, token);
        token = scanner.scan()
    }
}
