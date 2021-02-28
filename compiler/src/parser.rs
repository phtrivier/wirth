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

// TODO(pht) Using 'Results' for the intermediate step if a bit of a problem.
// Seems like all functions really want to return void, and keep a state with
// the current token , and a potential error.
pub struct Parser<'a> {
    scanner: Scanner<'a>,
    token: Token,
    done: bool,
    error: Option<ParseError>,
}

impl Parser<'_> {
    pub fn parse<'a>(content: &'a str) -> Result<(), ParseError> {
        match Parser::from_string(&content) {
            Ok(mut parser) => {
                parser.statement_sequence();
                return parser.result();
            }
            Err(e) => Err(e),
        }
    }

    pub fn from_string<'a>(content: &'a str) -> Result<Parser<'a>, ParseError> {
        let mut scanner = Scanner::new(&content);

        return match scanner.scan() {
            Ok(Some(token)) => {
                println!("Initial token {:?}", token);
                return Ok(Parser {
                    scanner: scanner,
                    token: token,
                    done: false,
                    error: None,
                });
            }
            Ok(None) => Err(ParseError::PrematureEOF),
            // TODO(pht) find a way to associate the ScanError to the parse error, otherwise it's lost :/
            Err(scan_error) => Err(ParseError::ScanError(scan_error)),
        };
    }

    pub fn result(&mut self) -> Result<(), ParseError> {
        if let Some(e) = self.error {
            return Err(e);
        }
        if !self.done {
            return Err(ParseError::UnexpectedToken);
        }
        return Ok(());
    }

    fn next(&mut self) {
        match self.scanner.scan() {
            Ok(Some(token)) => {
                println!("Next token {:?}", token);
                self.token = token;
                //                self.result = Ok(());
            }
            Ok(None) => {
                self.done = true;
                //              self.result = Ok(());
            }
            Err(scan_error) => {
                // self.result = Err(ParseError::ScanError(scan_error)),
                self.error = Some(ParseError::ScanError(scan_error))
            }
        }
    }

    pub fn factor(&mut self) {
        match &self.token {
            Token::Ident(_i) => {
                self.next();
                self.selector();
            }
            Token::Int(_n) => {
                self.next();
            }
            Token::Lparen => {
                self.next();
                self.expression();

                if let Token::Rparen = self.token {
                    self.next();
                } else {
                    self.error = Some(ParseError::UnexpectedToken);
                }
            }
            Token::Not => {
                self.next();
                self.factor();
            }
            _ => {
                self.error = Some(ParseError::UnexpectedToken);
            }
        }
    }

    pub fn term(&mut self) {
        self.factor();
        loop {
            match self.token {
                Token::Times | Token::Div | Token::Mod | Token::And => {
                    self.next();
                    self.factor();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn simple_expression(&mut self) {
        match self.token {
            Token::Plus => {
                self.next();
                self.term()
            }
            Token::Minus => {
                self.next();
                self.term()
            }
            _ => self.term(),
        }
        loop {
            match self.token {
                Token::Plus | Token::Minus | Token::Or => {
                    self.next();
                    self.term()
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn expression(&mut self) {
        self.simple_expression();
        match self.token {
            Token::Eql | Token::Neq | Token::Lss | Token::Geq | Token::Leq | Token::Gtr => {
                self.next();
                self.simple_expression()
            }
            _ => {}
        }
    }

    pub fn selector(&mut self) {
        loop {
            println!("Selector loop {:?}", self.token);
            match self.token {
                Token::Lbrak | Token::Period => {
                    if let Token::Lbrak = self.token {
                        self.next();
                        self.expression();
                        if let Token::Rbrak = self.token {
                            self.next();
                        }
                    } else {
                        self.next();
                        if let Token::Ident(_) = self.token {
                            self.next();
                        }
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    pub fn try_assignment(&mut self) {
        // TODO(pht) either do this self.next explicitely in all test
        // or find a way to make this more natural...
        self.next();
        if let Token::Becomes = self.token {
            self.next();
            self.expression();
        }
    }

    pub fn try_procedure_call(&mut self) {
        // NOTE(pht) This assumes that try_assignment was called before
        if let Token::Lparen = self.token {
            self.next();
            if let Token::Rparen = self.token {
                self.next()
            } else {
                loop {
                    self.parameter();
                    match self.token {
                        Token::Comma => {
                            self.next();
                            continue;
                        }
                        Token::Rparen => {
                            self.next();
                            break;
                        }
                        // NOTE(pht) this checks form sym > semicolon I don't undertand why
                        _ => break,
                    }
                }
            }
        }
    }

    pub fn try_if_statement(&mut self) {
        // TODO(pht) From ELSIF sym = OSS.if THEN...
    }

    pub fn parameter(&mut self) {
        self.expression();
    }

    /*
    statement = [assignment | ProcedureCall |
    IfStatement | CaseStatement | WhileStatement | RepeatStatement |
    LoopStatement | ForStatement | WithStatement | EXIT | RETURN [expression] ].
    */
    pub fn statement_sequence(&mut self) {
        loop {
            match self.token {
                Token::Ident(_) => {
                    self.try_assignment();
                    self.try_procedure_call();
                    self.try_if_statement();
                }

                Token::Return => {
                    self.next();
                    self.expression();
                }
                _ => {
                    // TODO(pht) handle other cases
                }
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    fn parser<'a>(s: &'a str) -> Parser<'a> {
        return Parser::from_string(&s).unwrap();
    }

    fn assert_parses(p: &mut Parser) {
        assert!(p.result().is_ok(), "Parsing error: {:?}", p.result());
    }

    #[test]
    fn test_selector() {
        for c in [".y", ".[0]"].iter() {
            let mut p = parser(c);
            p.selector();
            assert_parses(&mut p);
        }
    }

    #[test]
    fn test_factor() {
        for c in ["1", "x", "(1)", "~y"].iter() {
            let mut p = parser(c);
            p.factor();
            assert_parses(&mut p);
        }
    }
    #[test]
    fn test_term() {
        for c in ["1", "1*2", "1*2/3&5", "5*3/(1+x)", "2*x"].iter() {
            let mut p = parser(c);
            p.term();
            assert_parses(&mut p);
        }
    }

    #[test]
    fn test_simple_expression() {
        for c in ["1", "1+(1*2)/3", "y[0]+x.z*15"].iter() {
            let mut p = parser(c);
            p.simple_expression();
            assert_parses(&mut p);
        }
    }

    #[test]
    fn test_assignement() {
        for c in ["x:=1", "x:=1", "z:=y[0]", "x:=(x.z*15)+12"].iter() {
            let mut p = parser(c);
            p.try_assignment();
            assert_parses(&mut p);
        }
    }

    #[test]
    fn test_procedure_call() {
        for c in ["foo()", "foo(1)", "foo(bar, baz)", "foo(bar[1],baz*2)"].iter() {
            let mut p = parser(c);
            p.next();
            p.try_procedure_call();
            assert_parses(&mut p);
        }
    }

    /* TODO(pht) fix that
    #[test]
    fn test_if_statement() {
        for c in ["IF x > 0 THEN bar() ELSE y:=12 END"].iter() {
             let mut p = parser(c);
            p.next();
            p.try_if_statement();
            assert_parses(&mut p);
        }
    }
    */

    /*
    #[test]
    fn test_parses_valid_expression() {
        let content = String::from("RETURN 1 + 2 + 3 | 5");
        let c = Parser::parse(&content);
        assert!(c.is_ok(), format!("{:?}", c));
    }

    #[test]
    fn test_breaks_on_invalid_expression() {
        let content = String::from("@1 + x.Bar[0]");
        let c = Parser::parse(&content);
        assert!(c.is_err());
    }
    */
}
