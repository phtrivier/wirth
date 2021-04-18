

// #[derive(Debug)]
// struct Scanner<'a> {
//     line_number: u32,
//     lines: Lines<'a>,
//     line_scanner: Option<LineScanner<'a>>
// }

// impl Scanner <'_> {

//     pub fn new<'a>(content: &'a str) -> Scanner<'a> {
//         let lines = content.lines();
//         Scanner{
//             line_number: 0,
//             lines: lines,
//             line_scanner: LineScanner::new(&mut lines)
//         }
//     }

//     pub fn next<'a>(&mut self) -> Option<Scan> {
//         loop {
//             let peek = self.chars.peek();
//             println!("Peeked {:?}", peek);
//             match peek {
//                 Some(&(_column, c)) if (c == ' ' || c == '\t') => {
//                     println!("Peeked whitespace, will skip");
//                     return self.skip_whitespaces();
//                 }
//                 Some(&(_column, '\n')) => {
//                   println!("Peeked newline, will skip");
//                   return self.skip_newline();
//                 }
//                 Some(_) => {
//                   return None
//                 }
//                 None => {
//                     return None
//                 }
//             }
//         }
//     }

//     fn skip_whitespaces(&mut self) -> Option<Scan> {
//       loop {
//         let next_char = self.chars.next();
//         if let Some((_column, c)) = next_char {
//           if c == ' ' || c == '\t' {
//             continue;
//           } else {
//             break;
//           }
//         } else {
//           break;
//         }
//       }
//       return self.next();
//     }

//     fn skip_newline(&mut self) -> Option<Scan> {
//       self.line_number = self.line_number + 1;
//       self.chars.next();
//       return self.next();
//     }

// }
