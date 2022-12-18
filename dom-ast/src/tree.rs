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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ExpressionOp {
    Eql,
    Neq,
    Lss,
    Leq,
    Gtr,
    Geq,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VarType {
    Integer,
    Array(u32), // NOTE(pht) : this does not allow representing nested arrays, or arrays of record yet.
                // We'll have to store the type definitions somewhere that can be accessible at runtime to allow that :/
}

#[derive(Debug, PartialEq)]
pub enum NodeInfo {
    Module,
    Declarations,
    Declaration,
    Var,
    Type(VarType),
    StatementSequence,
    Assignement,
    Constant(u32),
    Ident(Rc<Symbol>), //
    Term(TermOp),
    SimpleExpression(SimpleExpressionOp),
    Expression(ExpressionOp),
    IfStatement,
    Then,
    Else,
    WhileStatement,
    Do,
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
