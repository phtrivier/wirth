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

  pub fn follow<'a>(&'a mut self, ast: &'a Ast) -> Option<&'a NodeInfo> {
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

pub fn empty<'a>() -> Ast {
  return Rc::new(Tree::Nil);
}

pub fn info<'a>(ast: &'a Ast) -> Option<&'a NodeInfo> {
  match ast.as_ref() {
    Tree::Node(TreeNode{
      info,
      ..
    }) => Some(info),
    Tree::Nil => None
  }
}

pub fn sibling<'a>(ast: &'a Ast) -> Option<&'a Ast> {
  match ast.as_ref() {
    Tree::Node(TreeNode{
      sibling,
      ..
    }) => Some(sibling),
    Tree::Nil => None
  }
}


pub fn child<'a>(ast: &'a Ast) -> Option<&'a Ast> {
  match ast.as_ref() {
    Tree::Node(TreeNode{
      child,
      ..
    }) => Some(child),
    Tree::Nil => None
  }
}


pub fn leaf<'a>(node_info: NodeInfo) -> Ast {
  return Rc::new(Tree::Node(TreeNode{
    info: node_info,
    child: empty(),
    sibling: empty()
  }));
}

pub fn node<'a>(node_info: NodeInfo, child: Ast, sibling: Ast) -> Ast {
  return Rc::new(Tree::Node(TreeNode{
    info: node_info,
    child: child,
    sibling: sibling
  }));
}

pub fn is_empty(ast: &Ast) -> bool {
  match ast.as_ref() {
    Tree::Nil => true,
    _ => false
  }
}

pub fn print(ast: &Ast) -> () {
  print_indentation("", ast);
  print!("\n");
}

fn print_indentation(prefix: &str, ast: &Ast) -> () {
  print!("{}", prefix);
  match ast.as_ref() {
    Tree::Nil => {
      print!("Nil");
    }
    Tree::Node(node) => {
      print!("{:?}", node.info);
      print!("\n");
      print_indentation((String::from(prefix) + " ").as_str(), &node.child);
      print!("\n");
      print_indentation((String::from(prefix)).as_str(), &node.sibling);
    }
  }

}