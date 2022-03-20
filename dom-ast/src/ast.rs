use std::rc::Rc;
use std::collections::VecDeque;

use crate::tree::Tree;
use crate::tree::TreeNode;
use crate::tree::NodeInfo;

pub type Ast = Rc<Tree>;

pub enum PathDir {
  Child,
  Sibling
}

pub struct Path {
  dirs: VecDeque<PathDir>
}

impl Path {
  pub fn root() -> Path {
    Path {
      dirs: VecDeque::new()
    }
  }

  pub fn child(&mut self) -> &mut Path {
    self.dirs.push_back(PathDir::Child);
    self
  }

  pub fn sibling(&mut self) -> &mut Path {
    self.dirs.push_back(PathDir::Sibling);
    self
  }

  pub fn follow<'a>(&'a mut self, ast: &'a Ast) -> Option<&'a NodeInfo> {
    match self.dirs.pop_front() {
      None => info(ast),
      Some(PathDir::Child) => 
        match child(ast) {
          Some(child) => self.follow(child),
          None => None
        }
      Some(PathDir::Sibling) => 
        match sibling(ast) {
          Some(sibling) => self.follow(sibling),
          None => None
        }
    }
  }
}

pub fn empty() -> Ast {
  Rc::new(Tree::Nil)
}

pub fn info(ast: &Ast) -> Option<&NodeInfo> {
  match ast.as_ref() {
    Tree::Node(TreeNode{
      info,
      ..
    }) => Some(info),
    Tree::Nil => None
  }
}

pub fn sibling(ast: &Ast) -> Option<&Ast> {
  match ast.as_ref() {
    Tree::Node(TreeNode{
      sibling,
      ..
    }) => Some(sibling),
    Tree::Nil => None
  }
}


pub fn child(ast: &Ast) -> Option<&Ast> {
  match ast.as_ref() {
    Tree::Node(TreeNode{
      child,
      ..
    }) => Some(child),
    Tree::Nil => None
  }
}


pub fn leaf(node_info: NodeInfo) -> Ast {
  Rc::new(Tree::Node(TreeNode{
    info: node_info,
    child: empty(),
    sibling: empty()
  }))
}

pub fn node(node_info: NodeInfo, child: Ast, sibling: Ast) -> Ast {
  Rc::new(Tree::Node(TreeNode{
    info: node_info,
    child,
    sibling
  }))
}

pub fn is_empty(ast: &Ast) -> bool {
  matches!(ast.as_ref(), Tree::Nil)
}

pub fn print(ast: &Ast) {
  print_indentation("", ast);
  println!();
}



fn print_indentation(prefix: &str, ast: &Ast) {
  print!("{}", prefix);
  match ast.as_ref() {
    Tree::Nil => {
      print!("Nil");
    }
    Tree::Node(node) => {
      print!("{:?}", node.info);
      println!();
      print_indentation((String::from(prefix) + " ").as_str(), &node.child);
      println!();
      print_indentation((String::from(prefix)).as_str(), &node.sibling);
    }
  }

}