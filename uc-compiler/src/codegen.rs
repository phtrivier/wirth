use risc::instructions::*;
use risc::instructions::OpCode::*;
use ast::ast::{Ast, sibling, child};
use ast::tree::{Tree, TreeNode, NodeInfo, TermOp, SimpleExpressionOp};

pub struct Codegen {
  pub instructions: Vec<Instruction>,
  rh: usize
}

impl Codegen {
  pub fn new() -> Codegen {
    Codegen{
      instructions: vec![],
      rh: 0
    }
  }

  // NOTE(pht) follow the CODE from the codegen at page 51/52, and
  // not the description (it is counter intuitive)
  pub fn generate_code(&mut self, tree: &Ast) {

    // println!("Generating code for ast");
    // ast::ast::print(tree);
    // println!("");

    match tree.as_ref() {
      Tree::Nil => {},
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

          NodeInfo::Expression(_operator) => {
            self.generate_code(child(tree).unwrap());
            self.generate_code(sibling(tree).unwrap());
            // The "decr by 2" seems a bit too simple for what I do :D
            self.rh -= 2;

            // TODO(pht) take the operator into account ?
            self.instructions.push(Instruction::Register{
              o: OpCode::SUB,
              a: self.rh,
              b: self.rh,
              c: self.rh +1
            });

          }

          NodeInfo::IfStatement => {
            self.generate_code(child(tree).unwrap());

            self.instructions.push(Instruction::BranchOff{
              cond: BranchCondition::NE,
              link: false,
              offset: 0 // Offset will be fixedup later
            });

            let fixup_index = self.instructions.len() - 1;

            self.generate_code(sibling(tree).unwrap());

            let destination_index = self.instructions.len();

            let fixup = (destination_index - fixup_index - 1) as i32;

            self.instructions[fixup_index] = Instruction::BranchOff{
              cond: BranchCondition::NE,
              link: false,
              offset: fixup // Offset will be fixedup later
            };
          }

          NodeInfo::Then => {
            self.generate_code(child(tree).unwrap());
          }

          NodeInfo::Ident(symbol) => {
            self.instructions.push(Instruction::Memory{
              u: MemoryMode::Load,
              a: self.rh,
              b: 14,
              offset: symbol.adr as u32
            });
            self.rh += 1;
  
          }

          // TODO(pht) Constant should be allowed to be negative...
          &NodeInfo::Constant(value) => {
            self.instructions.push(Instruction::RegisterIm{
              o: MOV,
              a: self.rh,
              b: 0,
              im: value as i32
            });
            self.rh += 1;
  
          }

          NodeInfo::Assignement => {
            if let Tree::Node(TreeNode{ 
              info: NodeInfo::Ident(symbol),
              child: _child,
              sibling: _sibling
            }) = node.child.as_ref() {

              self.generate_code(&node.sibling);

              // NOTE(pht): it is not absolutely clear if the rh = rh - 1 
              // has to be done before or after the STW ; but it only 
              // makes sense for me to do it before.
              self.rh -= 1;
              self.instructions.push(Instruction::Memory{
                u: MemoryMode::Store,
                a: self.rh,
                b: 14,
                offset: symbol.adr as u32
              });
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
            self.instructions.push(Instruction::Register{
              o: opcode,
              a: self.rh-1,
              b: self.rh-1,
              c: self.rh
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
            self.instructions.push(Instruction::Register{
              o: opcode,
              a: self.rh-1,
              b: self.rh-1,
              c: self.rh
            })
          }
        }
      }
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

  #[test]
  fn generate_no_instruction_for_empty_tree() {
    let mut codegen = Codegen::new();
    let tree = empty();
    codegen.generate_code(&tree,);
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

    assert_eq!(codegen.instructions, vec![
      Instruction::Memory{ u: MemoryMode::Load, a: 0, b: 14, offset: 0}
    ])
  }

  #[test]
  fn generate_load_instruction_for_assignment() {
    let mut scope = Scope::new();
    scope.add("x");
    scope.add("y");
    let mut scanner = Scanner::new("y:=42");
    // Necessary because parse_statement_sequence is not the first thing to compile yet
    parser::scan_next(&mut scanner).unwrap();
    let assignement = parser::parse_statement_sequence(&mut scanner, &mut scope).unwrap();
    
    let mut codegen = Codegen::new();
    codegen.generate_code(&assignement);

    assert_eq!(codegen.instructions, vec![
      Instruction::RegisterIm{ o: MOV, a: 0, b: 0, im: 42}, 
      Instruction::Memory{ u: MemoryMode::Store, a: 0, b: 14, offset: 1}
    ])
  }

  #[test]
  fn generate_load_instruction_for_multiple_assignments() {
    let mut scope = Scope::new();
    scope.add("x");
    scope.add("y");
    let mut scanner = Scanner::new("y:=42;x:=y");
    // Necessary because parse_statement_sequence is not the first thing to compile yet
    parser::scan_next(&mut scanner).unwrap();
    let assignement = parser::parse_statement_sequence(&mut scanner, &mut scope).unwrap();
    
    let mut codegen = Codegen::new();
    codegen.generate_code(&assignement);

    assert_eq!(codegen.instructions, vec![
      Instruction::RegisterIm{ o: MOV, a: 0, b: 0, im: 42}, 
      Instruction::Memory{ u: MemoryMode::Store, a: 0, b: 14, offset: 1},
      Instruction::Memory{ u: MemoryMode::Load, a: 0, b: 14, offset: 1},
      Instruction::Memory{ u: MemoryMode::Store, a: 0, b: 14, offset: 0},
    ])
  }

  #[test]
  fn generate_instructions_for_multiplication() {
    let mut scope = Scope::new();
    scope.add("x");
    scope.add("y");
    let mut scanner = Scanner::new("x*y");
    // Necessary because parse_xxx is not the first thing to compile yet
    parser::scan_next(&mut scanner).unwrap();
    let assignement = parser::parse_term(&mut scanner, &mut scope).unwrap();
    
    let mut codegen = Codegen::new();
    codegen.generate_code(&assignement);

    assert_eq!(codegen.instructions, vec![
      // Load ident X
      Instruction::Memory{ u: MemoryMode::Load, a: 0, b: 14, offset: 0},
      // Load ident Y
      Instruction::Memory{ u: MemoryMode::Load, a: 1, b: 14, offset: 1},
      // Multiply RH,RH,RH+1
      Instruction::Register{o: MUL, a: 0, b: 0, c: 1}
    ])
  }

  #[test]
  fn generate_instructions_for_branching() {
    let mut scope = Scope::new();
    scope.add("x");
    let mut scanner = Scanner::new("IF 0 = 1 THEN x:= 1 END");
    // Necessary because parse_xxx is not the first thing to compile yet
    parser::scan_next(&mut scanner).unwrap();
    parser::scan_next(&mut scanner).unwrap();
    let assignement = parser::parse_if_statement(&mut scanner, &mut scope).unwrap();
    
    let mut codegen = Codegen::new();
    codegen.generate_code(&assignement);
  
    assert_eq!(codegen.instructions, vec![
      // Load 0
      Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 0 },
      // Load 1
      Instruction::RegisterIm { o: MOV, a: 1, b: 0, im: 1 },
      // Compare 0 and 1
      Instruction::Register{o: SUB, a: 0, b: 0, c: 1},
      // Branch if not equals to fixed-up location
      Instruction::BranchOff{cond: BranchCondition::NE, offset: 2, link: false}, 
      // Load 1
      Instruction::RegisterIm { o: MOV, a: 0, b: 0, im: 1 },
      // Assign 1 to x
      Instruction::Memory {
        u: MemoryMode::Store,
        a: 0,
        b: 14,
        offset: 0
      },
    ])
  }
}