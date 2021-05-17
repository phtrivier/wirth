#![feature(assert_matches)]
use simulator::Simulator;
use simulator::*;
use assembler::*;
use ast::parser::*;

fn from_assembler(s: &str) -> Simulator {
  let content = String::from(s);
  return Simulator::from_assembler(&content).unwrap();
}

#[test]
fn invalid_assembly() {
  let content = String::from("NOT REALLY assembler AT all");
  let s = Simulator::from_assembler(&content);
  assert_matches!(s, Err(AssembleError::SyntaxError{ line_index: 0, line}) if line == "NOT REALLY assembler AT all");
}

#[test]
fn empty_assembly() {
  let s = from_assembler("");
  assert_eq!(s.memory(0, 1), [0]);
}

#[test]
fn invalid_memory_bounds() {
  let s = from_assembler("");
  assert_eq!(s.memory(risc::computer::MEMORY_SIZE + 12, 12), []);
  assert_eq!(s.memory(risc::computer::MEMORY_SIZE - 2, 3), [0, 0]);
}

#[test]
fn incomplete_execution() {
  let mut s = from_assembler("MOV R0,0\nMOV R1,1\nMOV R2,2");
  let execution = Execution{
    program_address: 0,
    stack_base: 0,
    max_cycles: 2
  };
  assert_matches!(s.execute(execution), Err(ExecutionError::MaxCycleReached));
}

fn from_assembler_file(filename: &str) -> Simulator {
  let content = std::fs::read_to_string(filename).unwrap();
  return  Simulator::from_assembler(&content).unwrap();
}

#[test]
fn count_to_3() {
  let mut s = from_assembler_file("count_to_3.a");

  let execution = Execution{
    program_address: 0,
    stack_base: 0,
    max_cycles: 50
  };
  s.execute(execution).unwrap();

  assert_eq!(s.registers(), [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn squares() {
  let mut s = from_assembler_file("squares.a");
  let execution = Execution{
    program_address: 0,
    stack_base: 0,
    max_cycles: 150
  };
  s.execute(execution).unwrap();
  assert_eq!(s.memory(10, 5), [0,1,4,9,16]);
}

#[test]
fn primes() {
  let mut s = from_assembler_file("primes.a");
  let execution = Execution{
    program_address: 0,
    stack_base: 0,
    max_cycles: 99999
  };
  s.execute(execution).unwrap();

  let base = 30;
  let expected = [2, 3, 5, 7, 11, 13, 17, 19, 23];
  assert_eq!(s.memory(base+1, expected.len()), expected);
}

#[test]
fn invalid_oberon() {
  let content = String::from("INVALID OBERON");
  let s = Simulator::from_oberon(&content);
  assert_matches!(s, Err(ParseError::UnexpectedToken(_)));
}

#[test]
fn oberon_assignments() {
  let content = String::from("MODULE Test; VAR x,y: INTEGER; BEGIN x:=42;y:=x END Test.");
  let mut s = Simulator::from_oberon(&content).unwrap();
  let execution = Execution{
    program_address: 0,
    stack_base: 100,
    max_cycles: 5
  };
  s.execute(execution).unwrap();
  assert_eq!(s.memory(execution.stack_base, 3), [0, 42, 42]);
}

#[test]
fn oberon_arithmetic() {
  let content = String::from("MODULE Test; VAR x,y: INTEGER; BEGIN x:=40+2;y:=((x+4)*2)/4-(10/2) END Test.");
  let mut s = Simulator::from_oberon(&content).unwrap();
  let execution = Execution{
    program_address: 0,
    stack_base: 100,
    max_cycles: 20
  };
  s.execute(execution).unwrap();
  assert_eq!(s.memory(execution.stack_base, 3), [0, 42, 18]);
}