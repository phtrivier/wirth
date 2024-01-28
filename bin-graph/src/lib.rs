#![allow(dead_code)]

use crate::ast::Ast;
use ::ast::{tree::{NodeInfo, TermOp, Tree}, ast::is_empty};
use ast::ast;
use log::debug;

pub fn to_dot(ast: &Ast) -> String {
    let mut s = String::from("digraph G {\n");

    let mut counter = 0;
    push_strings(&mut s, &ast, &mut counter);

    s.push_str("}\n");

    s.to_string()
}

fn push_strings(s: &mut String, ast: &Ast, counter: &mut u32) -> () {
    match ast.as_ref() {
        Tree::Nil => {}
        Tree::Node(node) => {
            let node_counter : u32 = 0 + *counter;
            s.push_str(format!("node{counter}[label=\"").as_str());
            s.push_str(node_label(&node.info).as_str());
            s.push_str("\"];\n");

            *counter = *counter + 1;

            if !is_empty(&node.child) {
                s.push_str(format!("node{}->node{};\n", node_counter, node_counter+1).as_str());
                push_strings(s, &node.child, counter);
            }

            if !is_empty(&node.sibling) {
                s.push_str(format!("node{}->node{};\n", node_counter, node_counter+2).as_str());
                push_strings(s, &node.sibling, counter);
            }
        }
    }

    s.push_str("")
}

fn node_label(node_info: &NodeInfo) -> String {
    match node_info {
        NodeInfo::Constant(number) => {
            format!("{number}")
        }
        NodeInfo::Term(TermOp::Times) => {
            format!("*")
        }
        _ => {
            format!("todo")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    use ::ast::{tree::{NodeInfo, TermOp, VarType, SimpleExpressionOp, ExpressionOp}, scope::Symbol};
    use test_log::test;

    #[test]
    fn is_empty_for_null() {
        let ast = ast::empty();
        assert_eq!("digraph G {\n}\n", to_dot(&ast))
    }

    #[test]
    fn is_simple_graph() {
        let child = ast::leaf(NodeInfo::Constant(42));
        let sibling = ast::leaf(NodeInfo::Constant(10));
        let node = ast::node(NodeInfo::Term(TermOp::Times), child, sibling);
        assert_eq!(
            concat!("digraph G {\n",
                    "node0[label=\"*\"];\n",
                    "node0->node1;\n",
                    "node1[label=\"42\"];\n",
                    "node0->node2;\n",
                    "node2[label=\"10\"];\n",
                    "}\n"),
            to_dot(&node)
        );
/*
"digraph G {
\nnode0[label=\"*\"];
\nnode0->node1;
\nnode0->node2;
\nnode1[label=\"42\"];
\nnode1->node2;
\nnode1->node3;
\nnode2[label=\"10\"];
\nnode2->node3;
\nnode2->node4;\n}"
*/
    }

    #[test]
    fn can_format_node_infos() {
/*
        assert_eq!("Module", node_label(&NodeInfo::Module));
        assert_eq!("Declarations", node_label(&NodeInfo::Declarations));
        assert_eq!("Declaration", node_label(&NodeInfo::Declaration));
        assert_eq!("Var", node_label(&NodeInfo::Var));

        assert_eq!("Integer", node_label(&NodeInfo::Type(VarType::Integer)));
        assert_eq!("Array[10]", node_label(&NodeInfo::Type(VarType::Array(10))));

        assert_eq!("StatSeq", node_label(&NodeInfo::StatementSequence));

        assert_eq!(":=", node_label(&NodeInfo::Assignement));
        assert_eq!("42", node_label(&NodeInfo::Constant(42)));
        assert_eq!("x", node_label(&NodeInfo::Ident(Rc::new(Symbol{
            name: "x".to_string(),
            adr: 0,
            size: 0
        }))));


        assert_eq!("*", node_label(&NodeInfo::Term(TermOp::Times)));
        assert_eq!("/", node_label(&NodeInfo::Term(TermOp::Div)));

        assert_eq!("+", node_label(&NodeInfo::SimpleExpression(SimpleExpressionOp::Plus)));
        assert_eq!("-", node_label(&NodeInfo::SimpleExpression(SimpleExpressionOp::Minus)));

        assert_eq!("=", node_label(&NodeInfo::Expression(ExpressionOp::Eql)));
        assert_eq!("!=", node_label(&NodeInfo::Expression(ExpressionOp::Neq)));
        assert_eq!("<", node_label(&NodeInfo::Expression(ExpressionOp::Lss)));
        assert_eq!("<=", node_label(&NodeInfo::Expression(ExpressionOp::Leq)));
        assert_eq!(">", node_label(&NodeInfo::Expression(ExpressionOp::Gtr)));
        assert_eq!(">=", node_label(&NodeInfo::Expression(ExpressionOp::Geq)));

        assert_eq!("If", node_label(&NodeInfo::IfStatement));
        assert_eq!("Then", node_label(&NodeInfo::Then));
        assert_eq!("Else", node_label(&NodeInfo::Else));
        assert_eq!("While", node_label(&NodeInfo::WhileStatement));
        assert_eq!("Do", node_label(&NodeInfo::Do));
*/
    }

}
