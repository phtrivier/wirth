#![feature(assert_matches)]
use simulator::Simulator;
use simulator::*;

#[test]
fn multiple_assignments_can_be_chained() {
    let content = String::from(
        "
  MODULE Test;
      VAR x,y: INTEGER;
    BEGIN
        x := 1;
        y := 2;
        x := 3
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution {
        stack_base: 100,
        max_cycles: 20,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 3), [0, 3, 2]);
}
