use std::rc::Rc;
use std::collections::VecDeque;

use crate::tree::Tree;
use crate::tree::TreeNode;
use crate::tree::NodeInfo;

pub type Ast<'a> = Rc<Tree<'a>>;

pub enum PathDir {
  Child,
  Sibling
}

pub struct Path {
  dirs: VecDeque<PathDir>
}

impl Path {
  pub fn root() -> Path {
    return Path {
      dirs: VecDeque::new()
    };
  }

  pub fn child(&mut self) -> &mut Path {
    self.dirs.push_back(PathDir::Child);
    return self;
  }

  pub fn sibling(&mut self) -> &mut Path {
    self.dirs.push_back(PathDir::Sibling);
    return self;
  }

  pub fn follow<'a>(&'a mut self, ast: &'a Ast<'a>) -> Option<&'a NodeInfo> {
    match self.dirs.pop_front() {
      None => return info(&ast),
      Some(PathDir::Child) => 
        match child(ast) {
          Some(child) => {
            return self.follow(child);
          }
          None => {
            return None;
          }
        }
      Some(PathDir::Sibling) => 
        match sibling(ast) {
          Some(sibling) => {
            return self.follow(sibling);
          }
          None => {
            return None
          }
        }
    }
  }
}

pub fn empty<'a>() -> Ast<'a> {
  return Rc::new(Tree::Nil);
}

pub fn info<'a>(ast: &'a Ast<'a>) -> Option<&'a NodeInfo> {
  match ast.as_ref() {
    Tree::Node(TreeNode{
      info,
      ..
    }) => Some(info),
    Tree::Nil => None
  }
}

pub fn sibling<'a>(ast: &'a Ast<'a>) -> Option<&'a Ast<'a>> {
  match ast.as_ref() {
    Tree::Node(TreeNode{
      sibling,
      ..
    }) => Some(sibling),
    Tree::Nil => None
  }
}


pub fn child<'a>(ast: &'a Ast<'a>) -> Option<&'a Ast<'a>> {
  match ast.as_ref() {
    Tree::Node(TreeNode{
      child,
      ..
    }) => Some(child),
    Tree::Nil => None
  }
}


pub fn leaf<'a>(node_info: NodeInfo<'a>) -> Ast<'a> {
  return Rc::new(Tree::Node(TreeNode{
    info: node_info,
    child: empty(),
    sibling: empty()
  }));
}

pub fn node<'a>(node_info: NodeInfo<'a>, child: Ast<'a>, sibling: Ast<'a>) -> Ast<'a> {
  return Rc::new(Tree::Node(TreeNode{
    info: node_info,
    child: child,
    sibling: sibling
  }));
}
