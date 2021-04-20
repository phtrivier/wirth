#[cfg(test)]
mod tests {

  use crate::*;

  #[test]
  fn count_to_3() {
    let debug = true;
    let mut c = load_assembly_file("count_to_3.a", true);
    let max_cycles = 50;
    c.execute(max_cycles, debug);
    assert_eq!(0, c.pc);
    assert_eq!(3, c.regs[0]);
  }

  #[test]
  fn squares() {
    let debug = true;
    let mut c = load_assembly_file("squares.a", true);
    let max_cycles = 99999;
    c.execute(max_cycles, debug);
    assert_eq!(0, c.pc);
    let base = 10;
    assert_eq!(0, c.mem[base + 0]);
    assert_eq!(1, c.mem[base + 1]);
    assert_eq!(4, c.mem[base + 2]);
    assert_eq!(9, c.mem[base + 3]);
  }

  #[test]
  fn primes() {
    let debug = true;
    let mut c = load_assembly_file("primes.a", true);
    let max_cycles = 99999;
    c.execute(max_cycles, debug);
    assert_eq!(0, c.pc);
    let base = 30;

    let expected = [2, 3, 5, 7, 11, 13, 17, 19, 23];
    assert_eq!(expected, c.mem[base+1..base+1+expected.len()]);

  }

  
}