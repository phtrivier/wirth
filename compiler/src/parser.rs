use crate::scanner::ScanError;
use crate::scanner::Scanner;
use crate::scanner::Token;

// NOTE(pht) I have to make this copyable otherwise, returning a ParseError as the
// right side of the error is cumbersome.
// But copying should be okay for this.
#[derive(Debug, Copy, Clone)]
pub enum ParseError {
    ScanError(ScanError),
    UnexpectedToken,
    PrematureEOF,
}

pub struct Parser<'a> {
    scanner: Scanner<'a>,
    token: Token,
    result: Result<(), ParseError>,
}

impl Parser<'_> {
    pub fn parse<'a>(content: &'a String) -> Result<(), ParseError> {
        let mut scanner = Scanner::new(&content);

        // NOTE(pht) the initial scan might be done in the first call...
        // never mind.
        return match scanner.scan() {
            Ok(Some(token)) => {
                println!("Initial token {:?}", token);
                let mut parser = Parser {
                    scanner: scanner,
                    token: token,
                    result: Ok(()),
                };
                // return parser.expression();
                // return parser.term()
                match parser.selector() {
                    Ok(_) => Ok(()),
                    e => e,
                }
            }
            Ok(None) => Err(ParseError::PrematureEOF),
            // TODO(pht) find a way to associate the ScanError to the parse error, otherwise it's lost :/
            Err(scan_error) => Err(ParseError::ScanError(scan_error)),
        };
    }

    fn next(&mut self) -> () {
        match self.scanner.scan() {
            Ok(Some(token)) => {
                println!("Next token {:?}", token);
                self.token = token;
                self.result = Ok(());
            }
            Ok(None) => {
                self.result = Ok(());
            }
            Err(scan_error) => self.result = Err(ParseError::ScanError(scan_error)),
        }
    }

    fn current(&mut self) -> Result<(), ParseError> {
        return self.result;
    }

    fn expression(&mut self) -> Result<(), ParseError> {
        return self.term();
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
                    self.next();
                    if let Ok(_) = self.current() {
                        if let Ok(_) = self.expression() {
                            match self.token {
                                Token::Rbrak => {
                                    self.next();
                                    return self.current();
                                }
                                _ => {
                                    return Err(ParseError::UnexpectedToken);
                                }
                            }
                        }
                    }
                    return self.current();
                }
                Token::Period => {
                    self.next();
                    if let Ok(_) = self.current() {
                        match self.token {
                            Token::Ident(_) => {
                                self.next();
                                if let Ok(_) = self.current() {
                                    continue;
                                } else {
                                    return self.current();
                                }
                            }
                            _ => return Err(ParseError::UnexpectedToken),
                        }
                    }
                    return self.current();
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
                self.next();
                return self.current();
            }
            _ => Err(ParseError::UnexpectedToken),
        }
    }
}
