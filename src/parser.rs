use crate::scanner::Scanner;
use crate::scanner::Token;
use crate::scanner::ScanError;

#[derive(Debug)]
pub enum ParseError {
    ScanError(ScanError),
    UnexpectedToken(Token),
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
                let parser = Parser{
                        scanner: scanner,
                        token: token
                };
                return parser.expression();
            }
            Ok(None) => Err(ParseError::PrematureEOF),
            Err(scan_error) => Err(ParseError::ScanError(scan_error))
        };
    }

    fn next(mut self) -> Result<(), ParseError> {
        match self.scanner.scan() {
            Ok(Some(token)) => {
                self.token = token;
                return Ok(());
            },
            Ok(None) => Ok(()),
            Err(scan_error) => Err(ParseError::ScanError(scan_error))
        }
    }

    fn term(self) -> Result<(), ParseError> {
        match self.token {
            Token::Int(_n) => {
                return self.next();
            }
            _ => Err(ParseError::UnexpectedToken(self.token))
        }
    }

}


// impl Parser<'_> {

//     pub fn parse<'a>(content: &'a String) -> Result<(), ParseError> {
//         Err(ParseError::PrematureEOF)
//     }

// }
