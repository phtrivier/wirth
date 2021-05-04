use risc::instructions::*;
use risc::instructions::OpCode::*;
use ast::tree::*;
use std::rc::Rc;

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

  pub fn generate_code(&mut self, tree: &Rc<Tree>) -> () {
    match tree.as_ref() {
      Tree::Nil => {},
      Tree::Node(node) => {
        match &node.info {
          NodeInfo::Ident(symbol) => {
            self.instructions.push(Instruction::Memory{
              u: MemoryMode::Load,
              a: self.rh,
              b: 14,
              offset: symbol.adr as u32
            });
            self.rh = self.rh + 1;
  
          }

          // TODO(pht) Constant should be allowed to be negative...
          &NodeInfo::Constant(value) => {
            self.instructions.push(Instruction::RegisterIm{
              o: MOV,
              a: self.rh,
              b: 0,
              im: value as i32
            });
            self.rh = self.rh + 1;
  
          }

          NodeInfo::Assignement => {
            if let Tree::Node(Node{ 
              info: NodeInfo::Ident(symbol),
              child: _child,
              sibling: _sibling
            }) = node.child.as_ref() {

              self.generate_code(&node.sibling);

              // NOTE(pht): it is not absolutely clear if the rh = rh - 1 
              // has to be done before or after the STW ; but it only 
              // makes sense for me to do it before.
              self.rh = self.rh - 1;
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

        }
      }
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;
  use ast::parser::*;
  use ast::scanner::*;
  use ast::scope::Scope;  

  #[test]
  fn generate_no_instruction_for_empty_tree() {
    let mut codegen = Codegen::new();
    let tree = Rc::new(Tree::Nil);
    codegen.generate_code(&tree,);
    assert_eq!(codegen.instructions, vec![]);
  }

  #[test]
  fn generate_load_instruction_for_single_ident() {
    let mut codegen = Codegen::new();
    let mut scope = Scope::new();
    scope.add("x");
    let symbol = scope.lookup("x").unwrap();

    let tree = Rc::new(Tree::Node(Node::ident(&symbol)));
    
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
    let p = Parser::new();
    let assignement = p.parse_statement_sequence(&mut scanner, &mut scope).unwrap();
    
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
    let p = Parser::new();
    let assignement = p.parse_statement_sequence(&mut scanner, &mut scope).unwrap();
    
    let mut codegen = Codegen::new();
    codegen.generate_code(&assignement);

    assert_eq!(codegen.instructions, vec![
      Instruction::RegisterIm{ o: MOV, a: 0, b: 0, im: 42}, 
      Instruction::Memory{ u: MemoryMode::Store, a: 0, b: 14, offset: 1},
      Instruction::Memory{ u: MemoryMode::Load, a: 0, b: 14, offset: 1},
      Instruction::Memory{ u: MemoryMode::Store, a: 0, b: 14, offset: 0},
    ])
  }

}