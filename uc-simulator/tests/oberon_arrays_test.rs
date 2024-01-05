#![feature(assert_matches)]
use simulator::Simulator;
use simulator::*;

#[test]
fn arrays_values_can_be_set_by_constant_index() {
    let content = String::from(
        "
  MODULE Test;
      VAR x: INTEGER;
          a: ARRAY 3 OF INTEGER;
    BEGIN
     x:= 0;
     a[0] := 1;
     a[1] := 2;
     a[2] := 3
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution {
        stack_base: 100,
        max_cycles: 50,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 5), [0, 0, 1, 2, 3]);
}

#[test]
fn arrays_values_can_be_accessed_by_constant_index() {
    let content = String::from(
        "
  MODULE Test;
      VAR x: INTEGER;
          a: ARRAY 3 OF INTEGER;
    BEGIN
     a[0] := 1;
     a[1] := 2;
     a[2] := 3;
     x := a[2]
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution {
        stack_base: 100,
        max_cycles: 50,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 5), [0, 3, 1, 2, 3]);
}

#[test]
fn arrays_values_can_be_accessed_by_variable_index() {
    let content = String::from(
        "
  MODULE Test;
      VAR x: INTEGER;
          i: INTEGER;
          a: ARRAY 3 OF INTEGER;
    BEGIN
     i := 0;
     a[i] := 5;
     i := i + 1;
     a[i] := 6;
     i := i + 1;
     a[i] := 7;
     i := i - 1;
     x := a[i]
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution { stack_base: 100, max_cycles: 50 };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 6), [0, 6, 1, 5, 6, 7]);
}
