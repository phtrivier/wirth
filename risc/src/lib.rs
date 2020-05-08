pub mod computer;
pub mod instructions;

pub fn run() -> () {
    let c = computer::Computer::new();
    c.dump_regs();
}
