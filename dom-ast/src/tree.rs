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
  StatementSequence,
  Assignement,
  Constant(u32),
  Ident(Rc<Symbol>),
  Term(TermOp),
  SimpleExpression(SimpleExpressionOp),
  // NOTE(pht) I really must stop caring avout the order in which declarations
  // are done, except maybe for the ProcedureDefinitions that must be in the end ?
  Declarations,
  Declaration,
  Var,
  Type(Type)
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

/* TODO(pht) make sure no ones need this
impl Tree {
  // Convenience method to allow exctracting the Node from a tree.
  // I don't know if I should use it except in tests ?
  pub fn get_node<'a>(tree: Rc<Tree>) -> Option<&'a TreeNode> {
    match tree.as_ref() {
      Tree::Node(node) => Some(node),
      Tree::Nil => None,
    }
  }

  pub fn get_child<'a>(tree: &'a Rc<Tree>) -> Option<&'a Rc<Tree>> {
    match tree.as_ref() {
      Tree::Node(node) => Some(&node.child),
      Tree::Nil => None
    }
  }

  pub fn get_child_node<'a>(tree: &'a Rc<Tree>) -> Option<&'a TreeNode> {
    return match tree.as_ref() {
      Tree::Node(node) => {
        Tree::get_node(node.child)
      }
      Tree::Nil => None,
    }
  }

  pub fn get_sibling<'a>(tree: &'a Rc<Tree>) -> Option<&'a Rc<Tree>> {
    match tree.as_ref() {
      Tree::Node(node) => Some(&node.sibling),
      Tree::Nil => None
    }
  }

  pub fn get_sibling_node<'a>(tree: &'a Rc<Tree>) -> Option<&'a TreeNode> {
    return match tree.as_ref() {
      Tree::Node(node) => {
        Tree::get_node(node.sibling)
      }
      Tree::Nil => None,
    }
  }
}
*/