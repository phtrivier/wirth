#![allow(dead_code)]

use crate::ast::Ast;
use ::ast::{
    ast::is_empty,
    tree::{ExpressionOp, NodeInfo, TermOp, Tree, VarType},
};
use ast::ast;
use log::debug;

pub fn to_dot(ast: &Ast) -> String {
    let mut s = String::from("digraph G {\n");

    let mut grow = vec![]; // Yes, this is stupid
    push_strings(&mut s, ast, &mut grow);

    s.push_str("}\n");

    debug!("{:?}", grow);

    s.to_string()
}

fn push_strings(s: &mut String, ast: &Ast, grow: &mut Vec<u32>) {
    match ast.as_ref() {
        Tree::Nil => {}
        Tree::Node(node) => {
            let node_id = format!("{}", grow.len());
            s.push_str(format!("node{node_id}[label=\"").as_str());
            s.push_str(node_label(&node.info).as_str());
            s.push_str("\"];\n");

            if !is_empty(&node.child) {
                let l = grow.len() as u32;
                grow.push(l);
                push_strings(s, &node.child, grow);
                s.push_str(format!("node{}->node{};\n", node_id, l + 1).as_str());
            }

            if !is_empty(&node.sibling) {
                let l = grow.len() as u32;
                grow.push(grow.len() as u32);

                push_strings(s, &node.sibling, grow);
                s.push_str(format!("node{}->node{};\n", node_id, l + 1).as_str());
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
            "*".to_string()
        }
        NodeInfo::Term(TermOp::Div) => {
            "/".to_string()
        }
        NodeInfo::SimpleExpression(::ast::tree::SimpleExpressionOp::Plus) => {
            "+".to_string()
        }
        NodeInfo::SimpleExpression(::ast::tree::SimpleExpressionOp::Minus) => {
            "-".to_string()
        }

        NodeInfo::Module => {
            "Module".to_string()
        }
        NodeInfo::Declaration => {
            "Declaration".to_string()
        }
        NodeInfo::Declarations => {
            "Declarations".to_string()
        }
        NodeInfo::Var => {
            "Var".to_string()
        }
        NodeInfo::IfStatement => {
            "If".to_string()
        }

        NodeInfo::Then => {
            "Then".to_string()
        }

        NodeInfo::Else => {
            "Else".to_string()
        }
        NodeInfo::WhileStatement => {
            "While".to_string()
        }
        NodeInfo::Do => {
            "Do".to_string()
        }
        NodeInfo::StatementSequence => {
            "StatSeq".to_string()
        }
        NodeInfo::Type(VarType::Integer) => {
            "Integer".to_string()
        }
        NodeInfo::Type(VarType::Array(n)) => {
            format!("Array[{n}]")
        }
        NodeInfo::Assignement => {
            ":=".to_string()
        }

        NodeInfo::Expression(ExpressionOp::Eql) => {
            "=".to_string()
        }
        NodeInfo::Expression(ExpressionOp::Neq) => {
            "!=".to_string()
        }
        NodeInfo::Expression(ExpressionOp::Lss) => {
            "<".to_string()
        }
        NodeInfo::Expression(ExpressionOp::Leq) => {
            "<=".to_string()
        }
        NodeInfo::Expression(ExpressionOp::Gtr) => {
            ">".to_string()
        }
        NodeInfo::Expression(ExpressionOp::Geq) => {
            ">=".to_string()
        }

        NodeInfo::Ident(ident) => {
            format!("{}", ident.name)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    use ::ast::{
        scope::Symbol,
        tree::{ExpressionOp, NodeInfo, SimpleExpressionOp, TermOp, VarType},
    };
    use test_log::test;

    #[test]
    fn is_empty_for_null() {
        let ast = ast::empty();
        assert_eq!("digraph G {\n}\n", to_dot(&ast))
    }

    #[test]
    fn is_simple_graph() {
        // 2 * (42 * 10)
        let x = ast::leaf(NodeInfo::Constant(42));
        let y = ast::leaf(NodeInfo::Constant(10));
        let xy = ast::node(NodeInfo::SimpleExpression(SimpleExpressionOp::Plus), x, y);
        let z = ast::leaf(NodeInfo::Constant(2));
        let zxy = ast::node(NodeInfo::Term(TermOp::Times), z, xy);

        println!("{}", to_dot(&zxy));

        assert_eq!(
            "digraph G {
node0[label=\"*\"];
node1[label=\"2\"];
node0->node1;
node2[label=\"+\"];
node3[label=\"42\"];
node2->node3;
node4[label=\"10\"];
node2->node4;
node0->node2;
}
",
            to_dot(&zxy)
        );
    }

    #[test]
    fn can_format_node_infos() {
        assert_eq!("Module", node_label(&NodeInfo::Module));

        assert_eq!("Declarations", node_label(&NodeInfo::Declarations));
        assert_eq!("Declaration", node_label(&NodeInfo::Declaration));
        assert_eq!("Var", node_label(&NodeInfo::Var));

        assert_eq!("*", node_label(&NodeInfo::Term(TermOp::Times)));
        assert_eq!("/", node_label(&NodeInfo::Term(TermOp::Div)));

        assert_eq!("+", node_label(&NodeInfo::SimpleExpression(SimpleExpressionOp::Plus)));
        assert_eq!("-", node_label(&NodeInfo::SimpleExpression(SimpleExpressionOp::Minus)));
        assert_eq!("If", node_label(&NodeInfo::IfStatement));
        assert_eq!("Then", node_label(&NodeInfo::Then));
        assert_eq!("Else", node_label(&NodeInfo::Else));
        assert_eq!("While", node_label(&NodeInfo::WhileStatement));
        assert_eq!("Do", node_label(&NodeInfo::Do));

        assert_eq!("Integer", node_label(&NodeInfo::Type(VarType::Integer)));
        assert_eq!("Array[10]", node_label(&NodeInfo::Type(VarType::Array(10))));

        assert_eq!("StatSeq", node_label(&NodeInfo::StatementSequence));

        assert_eq!(":=", node_label(&NodeInfo::Assignement));

        assert_eq!("=", node_label(&NodeInfo::Expression(ExpressionOp::Eql)));
        assert_eq!("!=", node_label(&NodeInfo::Expression(ExpressionOp::Neq)));
        assert_eq!("<", node_label(&NodeInfo::Expression(ExpressionOp::Lss)));
        assert_eq!("<=", node_label(&NodeInfo::Expression(ExpressionOp::Leq)));
        assert_eq!(">", node_label(&NodeInfo::Expression(ExpressionOp::Gtr)));
        assert_eq!(">=", node_label(&NodeInfo::Expression(ExpressionOp::Geq)));

        assert_eq!("42", node_label(&NodeInfo::Constant(42)));

        assert_eq!(
            "x",
            node_label(&NodeInfo::Ident(Rc::new(Symbol {
                name: "x".to_string(),
                adr: 0,
                size: 0
            })))
        );
    }
}
