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
    pub fn parse<'a>(content: &'a String) -> Result<(), ParseError> {
        match Parser::from_string(&content) {
            Ok(mut parser) => {
                parser.expression();
                return parser.result();
            }
            Err(e) => Err(e),
        }
    }

    pub fn from_string<'a>(content: &'a String) -> Result<Parser<'a>, ParseError> {
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

    /*
    fn current(&mut self) -> Result<(), ParseError> {
        return self.result;
    }
    */

    /* TODO(pht) implement those...

    PROCEDURE factor(VAR x: OSG.Item);
    VAR obj: OSG.Object;
    BEGIN (*sync*)
    IF sym < OSS.lparen THEN OSS.Mark("ident?");
      REPEAT OSS.Get(sym) UNTIL sym >= OSS.lparen
    END ;
    IF sym = OSS.ident THEN find(obj); OSS.Get(sym); OSG.MakeItem(x, obj); selector(x)
    ELSIF sym = OSS.number THEN OSG.MakeConstItem(x, OSG.intType, OSS.val); OSS.Get(sym)
    ELSIF sym = OSS.lparen THEN
     OSS.Get(sym); expression(x);
     IF sym = OSS.rparen THEN OSS.Get(sym) ELSE OSS.Mark(")?") END
     ELSIF sym = OSS.not THEN OSS.Get(sym); factor(x); OSG.Op1(OSS.not, x)
     ELSE OSS.Mark("factor?"); OSG.MakeItem(x, guard)
    END
    END factor;
    PROCEDURE term(VAR x: OSG.Item);
    VAR y: OSG.Item; op: INTEGER;
    BEGIN factor(x);
    WHILE (sym >= OSS.times) & (sym <= OSS.and) DO
     op := sym; OSS.Get(sym);
     IF op = OSS.and THEN OSG.Op1(op, x) END ;
     factor(y); OSG.Op2(op, x, y)
    END
    END term;

     */

    pub fn factor(&mut self) {
        /* NOTE(pht) the following is done in the code to handle
        error, but trying to parse until next valid char.
        I'm not doing that !!
        if self.is_lower_than_lparen() {
            println!("Skipping everything until lparen...");
            self.next();
            while self.is_lower_than_lparen() {
                println!("... still skipping...");
                self.next();
            }
        }
        */

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

                // TODO(pht) should be self.expression(), but hard to test piece by piece :/;
                self.next();
                // ----------

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
        /* TODO(pht)
                factor(x);
                WHILE (sym >= OSS.times) & (sym <= OSS.and) DO
                  op := sym; OSS.Get(sym);
                  IF op = OSS.and THEN OSG.Op1(op, x) END ;
                  factor(y); OSG.Op2(op, x, y)
               END
        */
        self.factor();
        loop {
            match self.token {
                Token::Times|
                Token::Div|
                Token::Mod|
                Token::And => {
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
        /* TODO(pht) handle the case white + - |
        while self.token in []
        */
    }

    fn expression(&mut self) {
        /*
                ROCEDURE expression(VAR x: OSG.Item);
                VAR y: OSG.Item; op: INTEGER;
                BEGIN SimpleExpression(x);
                IF (sym >= OSS.eql) & (sym <= OSS.gtr) THEN
                op := sym; OSS.Get(sym); SimpleExpression(y); OSG.Relation(op, x, y)
                END
                END expression;
        */
        self.simple_expression();
        match self.token {
            Token::Eql | Token::Neq | Token::Lss | Token::Geq | Token::Leq | Token::Gtr => {
                self.next();
                self.simple_expression()
            }
            _ => {

                // NOTe(pht) I'm not sure what this means, actually ?
            }
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
                            // TODO(pht) this will need to look at the type of the ident
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
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_factor() {
        for c in ["1", "x", "(1)", "~y"].iter() {
            println!("----");
            let content = String::from(*c);
            let mut p = Parser::from_string(&content).unwrap();
            p.factor();
            assert!(p.result().is_ok(), format!("{:?}", p.result()));
        }
    }
    
    #[test]
    fn test_term() {
        for c in ["1", "1*2", "1*2/3&5", "5*3/(y+1)"].iter() {
            println!("----");
            let content = String::from(*c);
            let mut p = Parser::from_string(&content).unwrap();
            p.term();
            assert!(p.result().is_ok(), format!("{:?}", p.result()));
        }
    }

    /*
    #[test]
    fn test_selector() {
        let content = String::from(".y");
        let mut p = Parser::from_string(&content).unwrap();
        p.selector();
        assert!(p.result().is_ok(), format!("{:?}", p.result()));

        let content = String::from(".[0]");
        p = Parser::from_string(&content).unwrap();
        p.selector();
        assert!(p.result().is_ok(), format!("{:?}", p.result()));
    }

    #[test]
    fn test_parses_valid_expression() {
        let content = String::from("1 + 2 + 3 | 5");
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
