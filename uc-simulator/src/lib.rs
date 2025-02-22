#![feature(assert_matches)]
use assembler::AssembleError;
use compiler::ParseError;
use risc::computer::Computer;

#[derive(Debug)]
pub struct Simulator {
    computer: Computer,
}

#[derive(Debug)]
pub enum ExecutionError {
    MaxCycleReached,
}

#[derive(Debug, Copy, Clone)]
pub struct Execution {
    pub stack_base: usize,
    pub max_cycles: u32,
}

impl Simulator {
    pub fn from_assembler(s: &str) -> Result<Simulator, AssembleError> {
        let instructions = assembler::assemble(s)?;
        let mut computer = Computer::new();
        computer.load_instructions(instructions);
        Ok(Simulator { computer })
    }

    pub fn from_oberon(s: &str) -> Result<Simulator, ParseError> {
        let instructions = compiler::compile(s)?;
        let mut computer = Computer::new();
        computer.load_instructions(instructions);
        Ok(Simulator { computer })
    }

    pub fn registers(&self) -> &[i32] {
        &self.computer.regs[..]
    }

    pub fn memory(&self, start: usize, count: usize) -> &[i32] {
        if start > risc::computer::MEMORY_SIZE {
            &[]
        } else {
            let upper_bound = std::cmp::min(risc::computer::MEMORY_SIZE, start + count);
            &self.computer.mem[start..upper_bound]
        }
    }

    pub fn execute(&mut self, execution: Execution) -> Result<(), ExecutionError> {
        self.start(execution.stack_base as i32);

        self.computer.execute(execution.max_cycles);

        if self.computer.pc == 0 {
            Ok(())
        } else {
            Err(ExecutionError::MaxCycleReached)
        }
    }

    pub fn start(&mut self, stack_base: i32) {
        self.computer.regs[14] = stack_base;
    }

    pub fn execute_next(&mut self) {
        self.computer.execute_next();
    }

    pub fn pc(&self) -> usize {
        self.computer.pc
    }
}
