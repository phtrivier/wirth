#[derive(Debug, PartialEq)]
pub struct Symbol {
  name: String
}

pub struct Scope {
  symbols: Vec<Symbol>
}
impl Scope {

  pub fn new() -> Scope {
    Scope{
      symbols: vec![]
    }
  }

  // TODO(pht) adding might fail, actually, if the symbol already exists.
  pub fn add(&mut self, s: &str) -> () {
    self.symbols.push(Symbol{
      name: String::from(s)
    })
  }

  pub fn lookup<'a>(&'a self, _s: &str) -> Option<&'a Symbol> {
    // TODO(pht) actually lookup the symbol in the table,
    // or in child scopes
    match self.symbols.len() {
      0 => None,
      _ => Some(&self.symbols[0])
    }
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
  fn can_find_symbole() {
    let mut s = Scope::new();
    s.add("x");
    assert_eq!(s.lookup("x").unwrap().name, "x");
  }

}
