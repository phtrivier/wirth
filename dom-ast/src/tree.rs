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

#[derive(Debug, PartialEq)]
pub enum NodeInfo<'a> {
  StatementSequence,
  Assignement,
  Constant(u32),
  Ident(&'a Symbol),
  Term(TermOp),
  SimpleExpression(SimpleExpressionOp)
}

#[derive(Debug)]
pub struct Node<'a> {
  pub info: NodeInfo<'a>,
  pub child: Rc<Tree<'a>>, // NOTE(pht) I wonder if those could be either Boxes. Or, If I don't want to allocate memory, a reference to a vec ?
  pub sibling: Rc<Tree<'a>>,
}

impl Node<'_> {
  pub fn ident<'a>(symbol: &'a Symbol) -> Node<'a> {
    Node {
      info: NodeInfo::Ident(symbol),
      child: Rc::new(Tree::Nil),
      sibling: Rc::new(Tree::Nil),
    }
  }

  pub fn constant<'a>(c: u32) -> Node<'a> {
    Node {
      info: NodeInfo::Constant(c),
      child: Rc::new(Tree::Nil),
      sibling: Rc::new(Tree::Nil),
    }
  }
}

#[derive(Debug)]
pub enum Tree<'a> {
  Node(Node<'a>),
  Nil,
}

impl Tree<'_> {
  // Convenience method to allow exctracting the Node from a tree.
  // I don't know if I should use it except in tests ?
  pub fn get_node<'a>(tree: &'a Rc<Tree<'a>>) -> Option<&'a Node<'a>> {
    match tree.as_ref() {
      Tree::Node(node) => Some(node),
      Tree::Nil => None,
    }
  }

  pub fn get_child<'a>(tree: &'a Rc<Tree<'a>>) -> Option<&'a Rc<Tree<'a>>> {
    match tree.as_ref() {
      Tree::Node(node) => Some(&node.child),
      Tree::Nil => None
    }
  }

  pub fn get_child_node<'a>(tree: &'a Rc<Tree<'a>>) -> Option<&'a Node<'a>> {
    return match tree.as_ref() {
      Tree::Node(node) => {
        Tree::get_node(&node.child)
      }
      Tree::Nil => None,
    }
  }

  pub fn get_sibling<'a>(tree: &'a Rc<Tree<'a>>) -> Option<&'a Rc<Tree<'a>>> {
    match tree.as_ref() {
      Tree::Node(node) => Some(&node.sibling),
      Tree::Nil => None
    }
  }

  pub fn get_sibling_node<'a>(tree: &'a Rc<Tree<'a>>) -> Option<&'a Node<'a>> {
    return match tree.as_ref() {
      Tree::Node(node) => {
        Tree::get_node(&node.sibling)
      }
      Tree::Nil => None,
    }
  }
}
