#![feature(assert_matches)]
use simulator::Simulator;
use simulator::*;

#[test]
fn statement_is_not_executed_if_condition_is_false() {
    let content = String::from(
        "
  MODULE Test; 
      VAR x: INTEGER; 
    BEGIN 
      IF 0 = 1 THEN 
        x:= 1 
      END
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution {
        program_address: 0,
        stack_base: 100,
        max_cycles: 20,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 2), [0, 0]);
}

#[test]
fn statement_is_executed_if_condition_is_true() {
    let content = String::from(
        "
  MODULE Test; 
      VAR x: INTEGER; 
    BEGIN 
      IF 1 = 1 THEN 
        x:= 1 
      END
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution {
        program_address: 0,
        stack_base: 100,
        max_cycles: 20,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 2), [0, 1]);
}

#[test]
fn all_statements_are_executed_if_condition_is_true() {
    let content = String::from(
        "
  MODULE Test; 
      VAR x,y: INTEGER; 
    BEGIN 
      IF 1 = 1 THEN 
        x:= 1;
        y:= 2
      END
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution {
        program_address: 0,
        stack_base: 100,
        max_cycles: 20,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 3), [0, 1, 2]);
}

#[test]
#[ignore] // TODO(pht)
fn else_statement_is_executed_if_condition_is_false() {
    let content = String::from(
        "
  MODULE Test; 
      VAR x: INTEGER; 
    BEGIN 
      IF 0 = 1 THEN 
        x:= 1 
      ELSE
        x:= 2
      END
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution {
        program_address: 0,
        stack_base: 100,
        max_cycles: 20,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 2), [0, 2]);
}

/*
#[test]
fn elseif_statement_is_executed_if_other_condition_is_true() {
  let content = String::from("
  MODULE Test;
      VAR x: INTEGER;
    BEGIN
      IF 0 = 1 THEN
        x:= 1
      ELSIF 1 = 1 THEN
        x:= 2
      END
  END Test.");
  let mut s = Simulator::from_oberon(&content).unwrap();
  let execution = Execution{
    program_address: 0,
    stack_base: 100,
    max_cycles: 20
  };
  s.execute(execution).unwrap();
  assert_eq!(s.memory(execution.stack_base, 2), [0, 2]);
}

#[test]
fn elseif_statement_is_not_executed_if_other_condition_is_false() {
  let content = String::from("
  MODULE Test;
      VAR x: INTEGER;
    BEGIN
      IF 0 = 1 THEN
        x:= 1
      ELSIF 0 = 1 THEN
        x:= 2
      ELSE
        x:= 3
      END
  END Test.");
  let mut s = Simulator::from_oberon(&content).unwrap();
  let execution = Execution{
    program_address: 0,
    stack_base: 100,
    max_cycles: 20
  };
  s.execute(execution).unwrap();
  assert_eq!(s.memory(execution.stack_base, 2), [0, 3]);
}


#[test]
fn elseif_statements_are_executed_if_some_conditions_is_true() {
  let content = String::from("
  MODULE Test;
      VAR x: INTEGER;
    BEGIN
      IF 0 = 1 THEN
        x:= 1
      ELSIF 0 = 2 THEN
        x:= 2
      ELSIF 0 = 0 THEN
        x:= 3
      ELSE
        x:= 4
      END
  END Test.");
  let mut s = Simulator::from_oberon(&content).unwrap();
  let execution = Execution{
    program_address: 0,
    stack_base: 100,
    max_cycles: 20
  };
  s.execute(execution).unwrap();
  assert_eq!(s.memory(execution.stack_base, 2), [0, 3]);
}


#[test]
fn elseif_statements_are_not_executed_if_all_other_conditions_are_false() {
  let content = String::from("
  MODULE Test;
      VAR x: INTEGER;
    BEGIN
      IF 0 = 1 THEN
        x:= 1
      ELSIF 0 = 2 THEN
        x:= 2
      ELSIF 0 = 3 THEN
        x:= 3
      ELSE
        x:= 4
      END
  END Test.");
  let mut s = Simulator::from_oberon(&content).unwrap();
  let execution = Execution{
    program_address: 0,
    stack_base: 100,
    max_cycles: 20
  };
  s.execute(execution).unwrap();
  assert_eq!(s.memory(execution.stack_base, 2), [0, 4]);
}
*/
