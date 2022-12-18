#![feature(assert_matches)]
use simulator::Simulator;
use simulator::*;

#[test]
fn full_program_generate_list_of_5_squares() {
    let content = String::from(
        "
  MODULE Test;
  VAR i: INTEGER;
      squares: ARRAY 5 OF INTEGER;
  BEGIN
    i := 0;
    WHILE i < 5 DO
      squares[i] := i * i;
      i := i + 1
    END
  END Test.",
    );
    let mut s = Simulator::from_oberon(&content).unwrap();
    let execution = Execution {
        program_address: 0,
        stack_base: 100,
        max_cycles: 100,
    };
    s.execute(execution).unwrap();
    assert_eq!(s.memory(execution.stack_base, 7), [0, 5, 0, 1, 4, 9, 16]);
}
