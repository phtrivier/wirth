use ast::ast::{child, info, sibling, Ast};
use ast::tree::{ExpressionOp, NodeInfo, SimpleExpressionOp, TermOp, Tree, TreeNode};
use risc::instructions::OpCode::*;
use risc::instructions::*;

pub struct Codegen {
    pub instructions: Vec<Instruction>,
    rh: usize,
    pub last_expression_operator: Option<ExpressionOp>,
}

impl Codegen {
    pub fn new() -> Codegen {
        Codegen {
            instructions: vec![],
            rh: 0,
            last_expression_operator: None,
        }
    }

    // NOTE(pht) follow the CODE from the codegen at page 51/52, and
    // not the description (it is counter intuitive)
    pub fn generate_code(&mut self, tree: &Ast) {
        // println!("Generating code for ast");
        // ast::ast::print(tree);
        // println!("");

        match tree.as_ref() {
            Tree::Nil => {}
            Tree::Node(node) => {
                match &node.info {
                    NodeInfo::Module => {
                        self.generate_code(sibling(tree).unwrap());
                    }

                    NodeInfo::Declarations => {
                        self.generate_code(sibling(tree).unwrap());
                    }

                    NodeInfo::Declaration => {
                        //
                    }

                    NodeInfo::Var => {
                        //
                    }

                    NodeInfo::Type(_) => {
                        //
                    }

                    NodeInfo::Expression(operator) => {
                        self.generate_code(child(tree).unwrap());
                        self.generate_code(sibling(tree).unwrap());
                        // The "decr by 2" seems a bit too simple for what I do :D
                        self.rh -= 2;

                        println!("Generating expression with operator {:?}", operator);
                        self.last_expression_operator = Some(*operator);

                        self.instructions.push(Instruction::Register {
                            o: OpCode::SUB,
                            a: self.rh,
                            b: self.rh,
                            c: self.rh + 1,
                        });
                    }

                    NodeInfo::IfStatement => {
                        println!("Generating if statement");

                        // This generate the code for the "test" part of the if
                        self.generate_code(child(tree).unwrap());

                        // This has to branch either to the end, or not branch at all
                        let condition = self.last_expression_condition();

                        self.instructions.push(Instruction::BranchOff {
                            cond: condition,
                            link: false,
                            offset: 0, // Offset will be fixedup later
                        });

                        let fixup_index = self.instructions.len() - 1;

                        let then_branch = sibling(tree).unwrap();

                        // This generates the code for the "then" part of the if
                        self.generate_code(then_branch);

                        // This fixes up the jump to the else part of the tree
                        let destination_index = self.instructions.len();

                        let fixup = (destination_index - fixup_index) as i32;

                        self.instructions[fixup_index] = Instruction::BranchOff {
                            cond: condition,
                            link: false,
                            offset: fixup,
                        };

                        // Generate the code for the "else" part, if applicable
                        let else_branch = sibling(then_branch).unwrap();
                        self.generate_code(else_branch);
                    }

                    NodeInfo::Then => {
                        self.generate_code(child(tree).unwrap());
                    }

                    NodeInfo::Else => {
                        // This wants to go to the last expression of the if / then else.
                        self.instructions.push(Instruction::BranchOff {
                            cond: BranchCondition::AW,
                            link: false,
                            offset: 0, // NOTE(pht) in case there is an else block, this offset will have to change
                        });

                        let aw_index = self.instructions.len() - 1;

                        self.generate_code(child(tree).unwrap());

                        let aw_destination_index = self.instructions.len();
                        let aw_fixup = (aw_destination_index - aw_index) as i32;
                        self.instructions[aw_index] = Instruction::BranchOff {
                            cond: BranchCondition::AW,
                            link: false,
                            offset: aw_fixup,
                        };
                    }

                    NodeInfo::WhileStatement => {
                        let test_index = self.instructions.len();

                        // This generate the code for the "test" part of the if
                        self.generate_code(child(tree).unwrap());

                        let condition = self.last_expression_condition();
                        self.instructions.push(Instruction::BranchOff {
                            cond: condition,
                            link: false,
                            offset: 0, // Offset will be fixedup later
                        });
                        let do_index = self.instructions.len() - 1;

                        // This generates the code for the "do" part of the while
                        let do_branch = sibling(tree).unwrap();
                        self.generate_code(do_branch);

                        // Fixup do index
                        let do_offset = (self.instructions.len() as i32) - (do_index as i32);
                        self.instructions[do_index] = Instruction::BranchOff {
                            cond: condition,
                            link: false,
                            offset: do_offset,
                        };

                        // Go back to the top
                        let back_to_test_offset = (self.instructions.len() as i32) - (test_index as i32);
                        self.instructions.push(Instruction::BranchOff {
                            cond: BranchCondition::AW,
                            link: false,
                            offset: -(back_to_test_offset + 1),
                        });
                    }

                    NodeInfo::Do => {
                        self.generate_code(child(tree).unwrap());
                    }

                    NodeInfo::Ident(ident_symbol) => {
                        println!("Generating code for ident symbol {:?}", ident_symbol);

                        let selector = child(tree).unwrap();

                        let mut ident_offset = ident_symbol.adr as u32;
                        match info(selector) {
                            None => {
                                self.instructions.push(Instruction::Memory {
                                    u: MemoryMode::Load,
                                    a: self.rh,
                                    b: 14,
                                    offset: ident_offset,
                                });

                                self.rh += 1;
                            }

                            Some(NodeInfo::Constant(selector_index)) => {
                                ident_offset += selector_index;

                                // R[A] <- M[SP + ident_offset]
                                self.instructions.push(Instruction::Memory {
                                    u: MemoryMode::Load,
                                    a: self.rh,
                                    b: 14,
                                    offset: ident_offset,
                                });

                                self.rh += 1;
                            }

                            Some(NodeInfo::Ident(selector_symbol)) => {
                                let selector_offset = selector_symbol.adr as u32;

                                // General form is
                                // R[A] <- M[R[B] + ident_offset]
                                //
                                // We want to write:
                                // self.instructions.push(Instruction::Memory {
                                //     u: MemoryMode::Load,
                                //     a: self.rh,
                                //     b: X,
                                //     offset: Y
                                // });
                                //
                                // So, that, in the end:
                                // R[X] != RH
                                // M[R[X] + Y] = M[SP + ident_offset + M[SP + selector_offset]]

                                // It would be easy to load the content of M[SP + selector_offset] in a new register:
                                self.instructions.push(Instruction::Memory {
                                    u: MemoryMode::Load,
                                    a: self.rh + 1,
                                    b: 14,
                                    offset: selector_offset,
                                });

                                // Know, we need to add the stack pointer at least once ?
                                self.instructions.push(Instruction::Register {
                                    o: ADD,
                                    a: self.rh + 1,
                                    b: self.rh + 1,
                                    c: 14,
                                });

                                // Finally, we can load the value into the previous register
                                self.instructions.push(Instruction::Memory {
                                    u: MemoryMode::Load,
                                    a: self.rh,
                                    b: self.rh + 1,
                                    offset: ident_offset,
                                });

                                // And we prepare next value
                                self.rh += 1;
                            }

                            _ => {
                                todo!("unsupported type of assignment");
                            }
                        }
                    }

                    // TODO(pht) Constant should be allowed to be negative...
                    &NodeInfo::Constant(value) => {
                        self.instructions.push(Instruction::RegisterIm {
                            o: MOV,
                            a: self.rh,
                            b: 0,
                            im: value as i32,
                        });
                        self.rh += 1;
                    }

                    NodeInfo::Assignement => {
                        if let Tree::Node(TreeNode {
                            info: NodeInfo::Ident(lhs_symbol),
                            child: selector,
                            ..
                        }) = node.child.as_ref()
                        {
                            // Generate code for value to be assigned
                            self.generate_code(&node.sibling);

                            // NOTE(pht): it is not absolutely clear if the rh = rh - 1
                            // has to be done before or after the STW ; but it only
                            // makes sense for me to do it before.
                            self.rh -= 1;

                            let mut offset = lhs_symbol.adr as u32;
                            match info(selector) {
                                None => {
                                    self.instructions.push(Instruction::Memory {
                                        u: MemoryMode::Store,
                                        a: self.rh,
                                        b: 14,
                                        offset,
                                    });
                                }

                                Some(NodeInfo::Constant(selector_index)) => {
                                    // NOTE(pht) this only works for constant selector
                                    offset += selector_index;

                                    self.instructions.push(Instruction::Memory {
                                        u: MemoryMode::Store,
                                        a: self.rh,
                                        b: 14,
                                        offset,
                                    });
                                }

                                // First part
                                // Handle _storing_ at the current value of indent
                                // This is the a[i] := 1 part
                                Some(NodeInfo::Ident(selector_symbol)) => {
                                    // So, what does not work is that the value of the index
                                    // is ignored ; somehow there is nothing that properly loads the value of rh in the thingy :/

                                    self.rh += 1;

                                    // Load the index value into a new register
                                    self.instructions.push(Instruction::Memory {
                                        a: self.rh,
                                        b: 14,
                                        offset: selector_symbol.adr as u32,
                                        u: MemoryMode::Load,
                                    });

                                    // Add the location of the base pointer
                                    // self.rh <- self.rh + offset
                                    self.instructions.push(Instruction::Register {
                                        o: ADD,
                                        a: self.rh,
                                        b: self.rh,
                                        c: 14,
                                    });

                                    // Store the value of the "previous" register at the new location
                                    self.instructions.push(Instruction::Memory {
                                        u: MemoryMode::Store,
                                        a: self.rh - 1,
                                        b: self.rh,
                                        offset,
                                    });

                                    // Then store are the right location

                                    self.rh -= 1;
                                }

                                // You'll have to generate the code for it
                                // The offset cannot stay a constant...
                                _ => {
                                    todo!("Assignement with selector is only implemented for constants and identifiers")
                                }
                            }
                        }
                    }

                    NodeInfo::StatementSequence => {
                        self.generate_code(&node.child);
                        self.generate_code(&node.sibling);
                    }

                    NodeInfo::Term(operator) => {
                        self.generate_code(&node.child);
                        self.generate_code(&node.sibling);
                        let opcode = match operator {
                            TermOp::Times => MUL,
                            TermOp::Div => DIV,
                        };
                        self.rh -= 1;
                        self.instructions.push(Instruction::Register {
                            o: opcode,
                            a: self.rh - 1,
                            b: self.rh - 1,
                            c: self.rh,
                        })
                    }

                    NodeInfo::SimpleExpression(operator) => {
                        self.generate_code(&node.child);
                        self.generate_code(&node.sibling);
                        let opcode = match operator {
                            SimpleExpressionOp::Plus => ADD,
                            SimpleExpressionOp::Minus => SUB,
                        };
                        self.rh -= 1;
                        self.instructions.push(Instruction::Register {
                            o: opcode,
                            a: self.rh - 1,
                            b: self.rh - 1,
                            c: self.rh,
                        })
                    }
                }
            }
        }
    }

    fn last_expression_condition(&mut self) -> BranchCondition {
        match self.last_expression_operator.unwrap() {
            ExpressionOp::Eql => BranchCondition::NE,
            ExpressionOp::Neq => BranchCondition::EQ,
            ExpressionOp::Lss => BranchCondition::GE,
            ExpressionOp::Leq => BranchCondition::GT,
            ExpressionOp::Gtr => BranchCondition::LE,
            ExpressionOp::Geq => BranchCondition::LT,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use ast::ast::{empty, leaf};
    use ast::parser;
    use ast::scanner::*;
    use ast::scope::Scope;

    #[cfg(test)]
    use pretty_assertions::assert_eq;

    #[test]
    fn generate_no_instruction_for_empty_tree() {
        let mut codegen = Codegen::new();
        let tree = empty();
        codegen.generate_code(&tree);
        assert_eq!(codegen.instructions, vec![]);
    }

    #[test]
    fn generate_load_instruction_for_single_ident() {
        let mut codegen = Codegen::new();
        let scope = Scope::new();
        scope.add("x");
        let symbol = scope.lookup("x").unwrap();

        let tree = leaf(NodeInfo::Ident(symbol));

        codegen.generate_code(&tree);

        assert_eq!(
            codegen.instructions,
            vec![Instruction::Memory {
                u: MemoryMode::Load,
                a: 0,
                b: 14,
                offset: 0
            }]
        )
    }

    #[test]
    fn generate_load_instruction_for_assignment() {
        let scope = Scope::new();
        scope.add("x");
        scope.add("y");
        let mut scanner = Scanner::new("y:=42");
        // Necessary because parse_statement_sequence is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_statement_sequence(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        assert_eq!(
            codegen.instructions,
            vec![
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 42 },
                Instruction::Memory {
                    u: MemoryMode::Store,
                    a: 0,
                    b: 14,
                    offset: 1
                }
            ]
        )
    }

    #[test]
    fn generate_load_instruction_for_array_assignment_at_constant() {
        let scope = Scope::new();
        scope.add("a");
        let mut scanner = Scanner::new("a[2]:=42");
        // Necessary because parse_statement_sequence is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_statement_sequence(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        assert_eq!(
            codegen.instructions,
            vec![
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 42 },
                Instruction::Memory {
                    u: MemoryMode::Store,
                    a: 0,
                    b: 14,
                    offset: 2
                }
            ]
        )
    }

    #[test]
    fn generate_load_instruction_for_array_assignment_at_variable() {
        let scope = Scope::new();
        scope.add("i"); // At address 0
        scope.add("a"); // At address 1
        let mut scanner = Scanner::new("i:=3;a[i]:=42");
        // Necessary because parse_statement_sequence is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_statement_sequence(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        assert_eq!(
            codegen.instructions,
            vec![
                // -------------------
                // i:=3 (address 0)
                // -------------------
                // Put 3 in R0
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 3 },
                // Move content of R0 to address 0
                Instruction::Memory {
                    a: 0,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Store,
                },
                // -------------------
                // a[i]:=42
                // -------------------
                // PUT 42 in R0
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 42 },
                // Put the content of i (address 0) in R1
                Instruction::Memory {
                    a: 1,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Load,
                },
                Instruction::Register { a: 1, b: 1, o: ADD, c: 14 },
                // Put the content of R0 in address R1 + offset
                Instruction::Memory {
                    a: 0,
                    b: 1,
                    offset: 1,
                    u: MemoryMode::Store,
                },
            ]
        )
    }

    #[test]
    fn generate_load_instruction_for_multiple_assignments() {
        let scope = Scope::new();
        scope.add("x");
        scope.add("y");
        let mut scanner = Scanner::new("y:=42;x:=y");
        // Necessary because parse_statement_sequence is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_statement_sequence(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        assert_eq!(
            codegen.instructions,
            vec![
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 42 },
                Instruction::Memory {
                    u: MemoryMode::Store,
                    a: 0,
                    b: 14,
                    offset: 1
                },
                Instruction::Memory {
                    u: MemoryMode::Load,
                    a: 0,
                    b: 14,
                    offset: 1
                },
                Instruction::Memory {
                    u: MemoryMode::Store,
                    a: 0,
                    b: 14,
                    offset: 0
                },
            ]
        )
    }

    #[test]
    fn generate_instructions_for_multiplication() {
        let scope = Scope::new();
        scope.add("x");
        scope.add("y");
        let mut scanner = Scanner::new("x*y");
        // Necessary because parse_xxx is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_term(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        assert_eq!(
            codegen.instructions,
            vec![
                // Load ident X
                Instruction::Memory {
                    u: MemoryMode::Load,
                    a: 0,
                    b: 14,
                    offset: 0
                },
                // Load ident Y
                Instruction::Memory {
                    u: MemoryMode::Load,
                    a: 1,
                    b: 14,
                    offset: 1
                },
                // Multiply RH,RH,RH+1
                Instruction::Register { o: MUL, a: 0, b: 0, c: 1 }
            ]
        )
    }

    #[test]
    fn generate_instructions_for_branching() {
        let scope = Scope::new();
        scope.add("x");
        let mut scanner = Scanner::new("IF 0 = 1 THEN x:= 1 END");
        // Necessary because parse_xxx is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_if_statement(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        assert_eq!(
            codegen.instructions,
            vec![
                // Load 0
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 0 },
                // Load 1
                Instruction::RegisterIm { o: MOV, a: 1, b: 0, im: 1 },
                // Compare 0 and 1
                Instruction::Register { o: SUB, a: 0, b: 0, c: 1 },
                // Branch if not equals to fixed-up location
                Instruction::BranchOff {
                    cond: BranchCondition::NE,
                    offset: 3,
                    link: false
                },
                // Load 1
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 1 },
                // Assign 1 to x
                Instruction::Memory {
                    u: MemoryMode::Store,
                    a: 0,
                    b: 14,
                    offset: 0
                },
            ]
        )
    }

    #[test]
    fn generate_instructions_for_branching_with_else() {
        let scope = Scope::new();
        scope.add("x");
        let mut scanner = Scanner::new("IF 0 = 1 THEN x:= 1 ELSE x:= 2 END");
        // Necessary because parse_xxx is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_if_statement(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        assert_eq!(
            codegen.instructions,
            vec![
                // Load 0
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 0 },
                // Load 1
                Instruction::RegisterIm { o: MOV, a: 1, b: 0, im: 1 },
                // Compare 0 and 1
                Instruction::Register { o: SUB, a: 0, b: 0, c: 1 },
                // Branch if not equals to the location of the 'else' part
                Instruction::BranchOff {
                    cond: BranchCondition::NE,
                    offset: 3,
                    link: false
                },
                // (Then part)
                // Load 1
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 1 },
                // Assign 1 to x
                Instruction::Memory {
                    u: MemoryMode::Store,
                    a: 0,
                    b: 14,
                    offset: 0
                },
                // Branch to the avoid the 'else' part
                Instruction::BranchOff {
                    cond: BranchCondition::AW,
                    offset: 3,
                    link: false
                },
                // (Else part)
                // Load 2
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 2 },
                // Assign 2 to x
                Instruction::Memory {
                    u: MemoryMode::Store,
                    a: 0,
                    b: 14,
                    offset: 0
                }
            ]
        )
    }

    #[test]
    fn generate_instructions_for_nested_else() {
        let scope = Scope::new();
        scope.add("x");
        let mut scanner = Scanner::new(
            "
        IF 0 = 1 THEN
            IF 0 = 1 THEN
                x:= 1
            ELSE
                x:= 2
            END
        ELSE
            x:= 3
        END",
        );
        // Necessary because parse_xxx is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_if_statement(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        assert_eq!(
            codegen.instructions,
            vec![
                // IF 0 = 1
                // Load 0
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 0 },
                Instruction::RegisterIm { o: MOV, a: 1, b: 0, im: 1 },
                Instruction::Register { o: SUB, a: 0, b: 0, c: 1 },
                Instruction::BranchOff {
                    cond: BranchCondition::NE,
                    offset: 10,
                    link: false
                },
                // THEN
                //   IF 0 = 1
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 0 },
                Instruction::RegisterIm { o: MOV, a: 1, b: 0, im: 1 },
                Instruction::Register { o: SUB, a: 0, b: 0, c: 1 },
                // Branch if not equals to the location of the 'else' part
                Instruction::BranchOff {
                    cond: BranchCondition::NE,
                    offset: 3,
                    link: false
                },
                //  THEN
                //    x := 1
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 1 },
                Instruction::Memory {
                    u: MemoryMode::Store,
                    a: 0,
                    b: 14,
                    offset: 0
                },
                Instruction::BranchOff {
                    cond: BranchCondition::AW,
                    offset: 3,
                    link: false
                },
                //  ELSE
                //    x: = 2
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 2 },
                Instruction::Memory {
                    u: MemoryMode::Store,
                    a: 0,
                    b: 14,
                    offset: 0
                },
                Instruction::BranchOff {
                    cond: BranchCondition::AW,
                    offset: 3,
                    link: false
                },
                //  END
                // ELSE
                //  x:= 3
                // END
                Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 3 },
                Instruction::Memory {
                    u: MemoryMode::Store,
                    a: 0,
                    b: 14,
                    offset: 0
                }
            ]
        );
    }

    #[test]
    fn generate_instructions_for_nested_if_else() {
        let scope = Scope::new();
        scope.add("x");
        let mut scanner = Scanner::new(
            "
            IF 1 = 1 THEN
                x:= 1
            ELSE
                IF 0 = 1 THEN
                  x:= 2
                ELSE
                  x:= 3
                END
            END",
        );
        // Necessary because parse_xxx is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_if_statement(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        assert_eq!(
            codegen.instructions,
            [
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 1 },
                Instruction::RegisterIm { a: 1, b: 0, o: MOV, im: 1 },
                Instruction::Register { a: 0, b: 0, o: SUB, c: 1 },
                Instruction::BranchOff {
                    cond: BranchCondition::NE,
                    offset: 3,
                    link: false,
                },
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 1 },
                Instruction::Memory {
                    a: 0,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Store,
                },
                Instruction::BranchOff {
                    cond: BranchCondition::AW,
                    offset: 10,
                    link: false,
                },
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 0 },
                Instruction::RegisterIm { a: 1, b: 0, o: MOV, im: 1 },
                Instruction::Register { a: 0, b: 0, o: SUB, c: 1 },
                Instruction::BranchOff {
                    cond: BranchCondition::NE,
                    offset: 3,
                    link: false,
                },
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 2 },
                Instruction::Memory {
                    a: 0,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Store,
                },
                Instruction::BranchOff {
                    cond: BranchCondition::AW,
                    offset: 3,
                    link: false,
                },
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 3 },
                Instruction::Memory {
                    a: 0,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Store,
                },
            ]
        );
    }

    #[test]
    fn generate_instructions_for_tripley_nested_if_else_than_make_no_sense() {
        let scope = Scope::new();
        scope.add("x");
        let mut scanner = Scanner::new(
            "
      IF 0 = 1 THEN
        x := 1
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
         ",
        );
        // Necessary because parse_xxx is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_if_statement(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);
        println!("{:#?}", codegen.instructions);

        // TODO(pht) some of the offset in this code seem to be are wrong ; or maybe some AW instructions must be added.
        // Try to find the culprit and fix it.
        assert_eq!(
            codegen.instructions,
            [
                // IF 0 = 1
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 0 },
                Instruction::RegisterIm { a: 1, b: 0, o: MOV, im: 1 },
                Instruction::Register { a: 0, b: 0, o: SUB, c: 1 },
                Instruction::BranchOff {
                    cond: BranchCondition::NE,
                    offset: 3,
                    link: false,
                },
                // THEN
                //
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 1 },
                Instruction::Memory {
                    a: 0,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Store,
                },
                Instruction::BranchOff {
                    cond: BranchCondition::AW,
                    offset: 16,
                    link: false,
                },
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 0 },
                Instruction::RegisterIm { a: 1, b: 0, o: MOV, im: 0 },
                Instruction::Register { a: 0, b: 0, o: SUB, c: 1 },
                Instruction::BranchOff {
                    cond: BranchCondition::NE,
                    offset: 9,
                    link: false,
                },
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 2 },
                Instruction::Memory {
                    a: 0,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Store
                },
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 0 },
                Instruction::RegisterIm { a: 1, b: 0, o: MOV, im: 0 },
                Instruction::Register { a: 0, b: 0, o: SUB, c: 1 },
                Instruction::BranchOff {
                    cond: BranchCondition::NE,
                    offset: 3,
                    link: false,
                },
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 3 },
                Instruction::Memory {
                    a: 0,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Store,
                },
                Instruction::BranchOff {
                    cond: BranchCondition::AW,
                    offset: 3,
                    link: false,
                },
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 4 },
                Instruction::Memory {
                    a: 0,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Store,
                },
            ]
        );
    }

    /*
    #[test]
    fn generate_instructions_for_if_with_condition_on_variables() {
        let scope = Scope::new();
        scope.add("x");
        let mut scanner = Scanner::new("IF x = 0 THEN x := 1 END");
        // Necessary because parse_xxx is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_if_statement(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        println!("{:#?}", codegen.instructions);
        assert!(false);
    }
    */

    #[test]
    fn generate_instructions_for_while_loop() {
        let scope = Scope::new();
        scope.add("x");
        let mut scanner = Scanner::new(
            "
            WHILE x = 0 DO
                x := 1
            END
            ",
        );
        // Necessary because parse_xxx is not the first thing to compile yet
        parser::scan_next(&mut scanner).unwrap();
        parser::scan_next(&mut scanner).unwrap();
        let assignement = parser::parse_while_statement(&mut scanner, &scope).unwrap();

        let mut codegen = Codegen::new();
        codegen.generate_code(&assignement);

        assert_eq!(
            codegen.instructions,
            [
                Instruction::Memory {
                    a: 0,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Load
                },
                Instruction::RegisterIm { a: 1, b: 0, o: MOV, im: 0 },
                Instruction::Register { a: 0, b: 0, o: SUB, c: 1 },
                Instruction::BranchOff {
                    cond: BranchCondition::NE,
                    offset: 3,
                    link: false,
                },
                Instruction::RegisterIm { a: 0, b: 0, o: MOV, im: 1 },
                Instruction::Memory {
                    a: 0,
                    b: 14,
                    offset: 0,
                    u: MemoryMode::Store,
                },
                Instruction::BranchOff {
                    cond: BranchCondition::AW,
                    offset: -7,
                    link: false,
                },
            ]
        );
    }
}
