#[feature(generic_associated_types)]
use std::str::CharIndices;
use std::iter::Peekable;


#[derive(Debug)]
enum Token {
    Ident(String)
}

#[derive(Debug)]
struct Scan {
    line: u32,
    column: u32,
    token: Token
}

struct LineScanner<'a> {
    line: u32,
    chars: Peekable<CharIndices<'a>>
}

impl LineScanner <'_> {
    // fn new<'a>(l: u32, s: &'a str) -> LineScanner {
    fn new<'a>(line: u32, s: &'a str) -> LineScanner<'a> {
        LineScanner{
            line: line,
            chars: s.char_indices().peekable()
        }
    }

    fn scan(&mut self) -> Option<Scan> {
    
        loop {
            // TODO(pht) peekable ?
            match self.chars.peek() {
                Some(&(_, ' ')) => {
                    self.chars.next();
                    continue;
                }
                Some(&(column, c)) => {
                    self.chars.next();
                
                    let mut pouet = String::from("");
                    pouet.push(c);
                    
                    // NOTE(pht) would continue
                    self.chars.next();
                    pouet.push(c);
                    
                    /*
                    self.chars.for_each(|x| {
                        s.push(x);
                    })
                
                    let pouet = self.chars.into_iter().take_while(|(i,c)| {
                        return !c.is_whitespace();
                    });
                    */
                
                    return Some(Scan{
                        line: self.line,
                        column: column as u32,
                        token: Token::Ident(pouet)
                    })
                }
                None => {
                    return None;
                }
            }
        }
    
        // loop {
        //     match self.chars[self.i] {
        //         ' ' => {
        //             // would be a loop until we find something that's
        //             // not whitespace
        //             self.i = self.i + 1;
        //             continue;
        //         }
        //         _c => {
        //             let start = self.i;
                    
        //             // would loop until self.i is a desirable character
        //             self.i = self.i + 1 + 1;
                    
        //             let end = self.i;
        //             let s = &self.chars[start..end];
        //             let ident = s.into_iter().collect::<String>();
        //             return Some(Scan{
        //                 line: self.line,
        //                 column: start as u32,
        //                 token: Token::Ident(ident)
        //             });
                    
        //         }
        //     }
        // }
        /*
        let s = &self.chars[0..2];
        let ident = s.into_iter().collect::<String>();
        return Some(Token::Ident(ident));
        */
    }
}


fn main() {
    let c = " IF TO";
    let mut s = LineScanner::new(0, &c);
    
    match s.scan() {
        Some(Scan{
            line: _line,
            column: _column,
            token: token
        }) => {
            match token {
                Token::Ident(value) => {
                    println!("Identifier {:?}", value);
                }
            }
        }
        _ => {
            panic!("wtf ?")
        }
    }
    
    // println!("{:?}", s.scan());
    
    
    // println!("{:?}", s.scan());
    /*
    let c = " I";
    let mut s = LineScanner::new(0, &c);
    println!("{:?}", s.scan());
    println!("{:?}", s.scan());
    */
    
}
