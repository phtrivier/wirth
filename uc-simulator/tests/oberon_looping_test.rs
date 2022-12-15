#![feature(assert_matches)]
use simulator::Simulator;
use simulator::*;


#[test]
fn while_statement_is_executed_until_condition_is_false() {
    let content = String::from(
        "
  MODULE Test;
      VAR x,y: INTEGER;
    BEGIN
      x := 0;
      WHILE x = 0 DO
        x:= x + 1;
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
