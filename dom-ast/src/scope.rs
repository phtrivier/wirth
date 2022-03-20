use std::rc::Rc;
use std::cell::RefCell;


#[derive(Debug, PartialEq)]
pub struct Symbol {
  pub name: String,
  // offset from stack base. Very limited at the moment, and assumes all type takes 1 word.
  pub adr: usize
}

struct Content {
  symbols: Vec<Rc<Symbol>>,
  next_adr: usize
}

pub struct Scope {
  content: RefCell<Content>
}
impl Scope {

  pub fn new() -> Scope {
    Scope{
      content: RefCell::new(Content{
        symbols: vec![],
        next_adr: 0  
      })
    }
  }

  pub fn add(&self, s: &str) -> () {
    let mut content = self.content.borrow_mut();

    let symbol = Symbol{
      name: String::from(s),
      adr: content.next_adr
    };

    content.next_adr = content.next_adr + 1;
    content.symbols.push(Rc::new(symbol));
  }

  pub fn lookup<'a>(&'a self, s: &str) -> Option<Rc<Symbol>> {
    let content = self.content.borrow();

    for symbol in content.symbols.iter() {
      if symbol.name == s {
        return Some(symbol.clone());
      }
    }
    return None;
  }
}

#[cfg(test)]
mod tests {
  use std::assert_matches::assert_matches;

  use super::*;

  #[test]
  fn fails_if_symbol_is_missing() {
    let s = Scope::new();
    assert_matches!(s.lookup("x"), None);
  }

  #[test]
  fn can_find_symbol() {
    let s = Scope::new();
    s.add("x");
    assert_eq!(s.lookup("x").unwrap().name, "x");
  }

  #[test]
  fn maintains_addresses_when_adding_symbol() {
    let s = Scope::new();
    s.add("x");
    s.add("y");
    assert_eq!(s.lookup("x").unwrap().adr, 0);
    assert_eq!(s.lookup("y").unwrap().adr, 1);
  }

  fn function_that_adds(s: &str, scope: &mut Scope) {
    scope.add(s);
  }
  
  fn function_that_adds_and_adds(scope: &mut Scope) {
    function_that_adds("x", scope);
    function_that_adds("x", scope);
  }

  #[test]
  fn can_be_called_in_a_loop() {
    let mut s = Scope::new();
    function_that_adds("x", &mut s);
    function_that_adds("y", &mut s);
    function_that_adds_and_adds(&mut s);
  }

}
