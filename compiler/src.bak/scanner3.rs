// Very ugly version that would work, but completely negate all intereset in having a Char iterator, since 
// I turn the chars() into a vec of 'char. Which would break for uTF8, I don't even know...

#[feature(generic_associated_types)]
use std::str::CharIndices;

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

struct LineScanner {
    line: u32,
    i: usize,
    chars: Vec<char>
}

impl LineScanner {
    // fn new<'a>(l: u32, s: &'a str) -> LineScanner {
    fn new(l: u32, s: &str) -> LineScanner {
        LineScanner{
            line: l,
            i: 0,
            chars: s.chars().collect()
        }
    }

    fn scan(&mut self) -> Option<Scan> {
    
        loop {
            match self.chars[self.i] {
                ' ' => {
                    // would be a loop until we find something that's
                    // not whitespace
                    self.i = self.i + 1;
                    continue;
                }
                _c => {
                    let start = self.i;
                    
                    // would loop until self.i is a desirable character
                    self.i = self.i + 1 + 1;
                    
                    let end = self.i;
                    let s = &self.chars[start..end];
                    let ident = s.into_iter().collect::<String>();
                    return Some(Scan{
                        line: self.line,
                        column: start as u32,
                        token: Token::Ident(ident)
                    });
                    
                }
            }
        }
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
    println!("{:?}", s.scan());
    println!("{:?}", s.scan());
    
    let c = " I";
    let mut s = LineScanner::new(0, &c);
    println!("{:?}", s.scan());
    println!("{:?}", s.scan());
    
}
