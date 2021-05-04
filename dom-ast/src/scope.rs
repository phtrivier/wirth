#[derive(Debug, PartialEq)]
pub struct Symbol {
  pub name: String,
  // offset from stack base. Very limited at the moment, and assumes all type takes 1 word.
  pub adr: usize
}

pub struct Scope {
  symbols: Vec<Symbol>,
  next_adr: usize
}
impl Scope {

  pub fn new() -> Scope {
    Scope{
      symbols: vec![],
      next_adr: 0
    }
  }

  // TODO(pht) adding might fail, actually, if the symbol already exists.
  pub fn add(&mut self, s: &str) -> () {
    let symbol = Symbol{
      name: String::from(s),
      adr: self.next_adr
    };
    self.symbols.push(symbol);
    self.next_adr = self.next_adr + 1;
  }

  pub fn lookup<'a>(&'a self, s: &str) -> Option<&'a Symbol> {
    for symbol in self.symbols.iter() {
      if symbol.name == s {
        return Some(&symbol);
      }
    }
    return None;
  }
}



#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn fails_if_symbol_is_missing() {
    let s = Scope::new();
    assert_matches!(s.lookup("x"), None);
  }

  #[test]
  fn can_find_symbol() {
    let mut s = Scope::new();
    s.add("x");
    assert_eq!(s.lookup("x").unwrap().name, "x");
  }

  #[test]
  fn maintains_addresses_when_adding_symbol() {
    let mut s = Scope::new();
    s.add("x");
    s.add("y");
    assert_eq!(s.lookup("x").unwrap().adr, 0);
    assert_eq!(s.lookup("y").unwrap().adr, 1);
  }

}
