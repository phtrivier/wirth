mod scanner;

use scanner::Scanner;

/**
 * A sort of Oberon Compiler
 */

fn parse_content(content: String) {
  let mut scanner = Scanner::new(&content);

  let mut scanned = scanner.scan();
  loop {
    match scanned {
      Ok(None) => {
        break;
      }
      Ok(Some(token)) => {
        println!("Line {} - Token: {:?}", scanner.line(), token);
        scanned = scanner.scan();
      }
      Err(err) => {
        println!("Line {} - Scanning error: {:?}", scanner.line(), err);
        break;
      }
    }
  }
}

fn main() {

    let _raw_content = r#"
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

    // let content = String::from(raw_content);
    // parse_content(content);

    let broken_content = String::from(r#"
    MODULE (*
    "#);
    parse_content(broken_content);
}
