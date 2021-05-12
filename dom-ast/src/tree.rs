use crate::scope::*;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum TermOp {
  Times,
  Div
}

#[derive(Debug, PartialEq)]
pub enum SimpleExpressionOp{
  Plus,
  Minus
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
  Integer
}

#[derive(Debug, PartialEq)]
pub enum NodeInfo {
  Module,
  Declarations,
  Declaration,
  Var,
  Type(Type),
  StatementSequence,
  Assignement,
  Constant(u32),
  Ident(Rc<Symbol>),
  Term(TermOp),
  SimpleExpression(SimpleExpressionOp),
}

#[derive(Debug)]
pub struct TreeNode {
  pub info: NodeInfo,
  pub child: Rc<Tree>, // NOTE(pht) I wonder if those could be either Boxes. Or, If I don't want to allocate memory, a reference to a vec ?
  pub sibling: Rc<Tree>,
}

impl TreeNode {
  pub fn ident(symbol: Rc<Symbol>) -> TreeNode {
    TreeNode {
      info: NodeInfo::Ident(symbol),
      child: Rc::new(Tree::Nil),
      sibling: Rc::new(Tree::Nil),
    }
  }

  pub fn constant(c: u32) -> TreeNode {
    TreeNode {
      info: NodeInfo::Constant(c),
      child: Rc::new(Tree::Nil),
      sibling: Rc::new(Tree::Nil),
    }
  }
}

#[derive(Debug)]
pub enum Tree {
  Node(TreeNode),
  Nil,
}