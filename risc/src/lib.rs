mod computer;

pub fn run() -> () {
    let c = computer::Computer::new();
    c.dumpRegs();
    c.execute();
}