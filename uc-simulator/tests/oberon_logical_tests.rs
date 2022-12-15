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
fn variable_statement_is_executed_if_condition_is_true() {
    let content = String::from(
        "
  MODULE Test;
      VAR x: INTEGER;
    BEGIN
      x := 1;
      IF x = 1 THEN
        x:= 2
      END;
      IF x = 42 THEN
        x:= 3
      ELSE
        x:=4
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
    assert_eq!(s.memory(execution.stack_base, 2), [0, 4]);
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
fn else_statement_is_executed_if_condition_is_false() {
    let content = String::from(
        "
  MODULE Test; 
      VAR x,y: INTEGER;
    BEGIN 
      IF 0 = 1 THEN 
        x:= 1 
      ELSE
        x:= 2;
        y:= 3
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
    assert_eq!(s.memory(execution.stack_base, 3), [0, 2, 3]);
}

#[test]
fn more_nested_else_if() {
    let content = String::from(
        "
  MODULE Test;
      VAR x,y: INTEGER;
    BEGIN
      IF 0 = 1 THEN
        x:= 1
      ELSE
        IF 0 = 0 THEN
           x := 2;
           IF 0 = 0 THEN
             x := 3
           END
        ELSE
           x := 4
        END
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
    assert_eq!(s.memory(execution.stack_base, 3), [0, 3, 0]);
}

#[test]
fn more_nested_else_if_but_the_other_way_around() {
    let content = String::from(
        "
  MODULE Test;
      VAR x: INTEGER;
    BEGIN
      IF 1 = 1 THEN
        x := 1
      ELSE
        x := 5;

        IF 0 = 0 THEN
           x := 2
        ELSE
           x := 3
        END
      END
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution {
        program_address: 0,
        stack_base: 100,
        max_cycles: 50,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 2), [0, 1]);
}

#[test]
fn nested_else_statement_is_executed_if_condition_is_false() {
    let content = String::from(
        "
  MODULE Test;
      VAR x,y: INTEGER;
    BEGIN
      IF 0 = 1 THEN
        x:= 1
      ELSE
        IF 0 = 2 THEN
           x := 2;
           IF 0 = 0 THEN
             y := 3
           END
        ELSE
           IF 0 = 1 THEN
             x := 4
           ELSE
             y := 3;
             IF 0 = 0 THEN
               y:= 4
             END;
             x:=5
           END
        END
      END
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution {
        program_address: 0,
        stack_base: 100,
        max_cycles: 50,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 3), [0, 5, 4]);
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
