pub mod computer;

pub fn run() -> () {
    let c = computer::Computer::new();
    c.dump_regs();
    c.execute();
}