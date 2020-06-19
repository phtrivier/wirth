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
        let mut scanner = Scanner::new(&content);

        // TODO(pht)  The initial scan might is a bit cumbersome... but I want the
        // token to be absolutely set !
        return match scanner.scan() {
            Ok(Some(token)) => {
                println!("Initial token {:?}", token);
                let mut parser = Parser {
                    scanner: scanner,
                    token: token,
                    done: false,
                    error: None,
                };
                parser.expression();
                if let Some(e) = parser.error {
                    return Err(e);
                }
                if !parser.done {
                    return Err(ParseError::UnexpectedToken);
                }
                return Ok(());
            }
            Ok(None) => Err(ParseError::PrematureEOF),
            // TODO(pht) find a way to associate the ScanError to the parse error, otherwise it's lost :/
            Err(scan_error) => Err(ParseError::ScanError(scan_error)),
        };
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

    fn term(&mut self) {
        match self.token {
            Token::Int(_n) => {
                self.next();
            }
            _ => self.error = Some(ParseError::UnexpectedToken),
        }
    }

    fn simple_expression(&mut self) {
        /*
                PROCEDURE SimpleExpression(VAR x: OSG.Item);
                VAR y: OSG.Item; op: INTEGER;
                BEGIN
                    IF sym = OSS.plus THEN OSS.Get(sym); term(x)
                    ELSIF sym = OSS.minus THEN OSS.Get(sym); term(x); OSG.Op1(OSS.minus, x)
                ELSE term(x)
                    END;
                WHILE (sym >= OSS.plus) & (sym <= OSS.or) DO
                    op := sym; OSS.Get(sym);
                IF op = OSS.or THEN OSG.Op1(op, x) END ;
                term(y); OSG.Op2(op, x, y)
                    END
                    END SimpleExpression;
        */
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

    /*
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
     */
}
