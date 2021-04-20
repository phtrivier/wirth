#![feature(assert_matches)]
pub mod computer;
pub mod computer_test;
pub mod instructions;

pub fn run() -> () {
    let c = computer::Computer::new();
    c.dump_regs();
}
