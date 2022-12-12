use crate::scope::*;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum TermOp {
    Times,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum SimpleExpressionOp {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
pub enum ExpressionOp {
    Eql,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Type {
    Integer,
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
    Expression(ExpressionOp),
    IfStatement,
    Then,
    Else
}

#[derive(Debug)]
pub struct TreeNode {
    pub info: NodeInfo,
    pub child: Rc<Tree>, // NOTE(pht) I wonder if those could be either Boxes. Or, If I don't want to allocate memory, a reference to a vec ?
    pub sibling: Rc<Tree>,
}

#[derive(Debug)]
pub enum Tree {
    Node(TreeNode),
    Nil,
}
