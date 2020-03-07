use crate::scanner::Scanner;
use crate::scanner::Token;
use crate::scanner::ScanError;

#[derive(Debug)]
pub enum ParseError {
    ScanError(ScanError),
    UnexpectedToken,
    PrematureEOF
}

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    token: Token
}

impl Parser<'_> {
    pub fn parse<'a>(content: &'a String) -> Result<(), ParseError> {
        let mut scanner = Scanner::new(&content);

        // NOTE(pht) the initial scan might be done in the first call...
        // never mind.
        return match scanner.scan() {
            Ok(Some(token)) => {
                println!("Initial token {:?}", token);
                let mut parser = Parser{
                        scanner: scanner,
                        token: token
                };
                // return parser.expression();
                // return parser.term()
                return parser.selector()
            }
            Ok(None) => Err(ParseError::PrematureEOF),
            Err(scan_error) => Err(ParseError::ScanError(scan_error))
        };
    }

    fn next(&mut self) -> Result<(), ParseError> {
        match self.scanner.scan() {
            Ok(Some(token)) => {
                println!("Next token {:?}", token);
                self.token = token;
                return Ok(());
            },
            Ok(None) => Ok(()),
            Err(scan_error) => Err(ParseError::ScanError(scan_error))
        }
    }

    fn expression(&mut self) -> Result<(), ParseError> {
        return self.term()
        // // TODO(pht) full experessions, this only support terms at the moment
        // match self.token {
        //     Token::Int(_) => {
        //         return self.next()
        //     }
        //     _ => {
        //         return Err(ParseError::UnexpectedToken)
        //     }
        // }
    }

    fn selector(&mut self) -> Result<(), ParseError> {
        loop {
            match self.token {
                Token::Lbrak => {
                    let n = self.next();
                    match n {
                        Ok(_) => {
                            let e = self.expression();
                            match e {
                                Ok(_) => {
                                    match self.token {
                                        Token::Rbrak => {
                                            return self.next()
                                        }
                                        _ => {
                                            return Err(ParseError::UnexpectedToken);
                                        }
                                    }
                                },
                                Err(_) => return e
                            }
                        }
                        Err(_) => return n
                    }
                }
                Token::Period => {
                    let n = self.next();
                    match n {
                        Ok(_) => {
                            match self.token {
                                Token::Ident(_) => {
                                    let n = self.next();
                                    match n {
                                        Ok(_) => {
                                            continue;
                                        },
                                        Err(_) => {
                                            return n;
                                        }
                                    }
                                }
                                _ => {
                                    return Err(ParseError::UnexpectedToken)
                                }
                            }
                        }
                        Err(_) => return n
                    }
                }
                _ => {
                    return Err(ParseError::UnexpectedToken);
                }
            }
        }
    }

    fn term(&mut self) -> Result<(), ParseError> {
        match self.token {
            Token::Int(_n) => {
                return self.next();
            }
            _ => Err(ParseError::UnexpectedToken)
        }
    }

}


// impl Parser<'_> {

//     pub fn parse<'a>(content: &'a String) -> Result<(), ParseError> {
//         Err(ParseError::PrematureEOF)
//     }

// }
