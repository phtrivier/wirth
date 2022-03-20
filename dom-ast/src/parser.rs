use crate::ast;
use crate::ast::Ast;
use crate::scanner::*;
use crate::scope::*;
use crate::token::*;
use crate::tree::*;

use std::rc::Rc;

#[derive(Debug)]
pub enum ParseError {
    ScanError(crate::token::ScanError),
    UndefinedSymbol(String),
    PrematureEof,
    UnexpectedToken(Rc<Scan>),
    SymbolAlreadyDeclared(String, ScanContext),
    UnexpectedBlockEnding { expected: String, found: String },
    Todo,
}

pub type ParseResult = Result<Rc<Tree>, ParseError>;

type IdentList = Vec<(String, ScanContext)>;

pub fn parse_module(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let current = current_token(scanner)?;

    let current = match current.as_ref() {
        Scan { token: Token::Module, .. } => {
            scan_next(scanner)?;
            current_token(scanner)?
        }
        _ => {
            return Err(ParseError::UnexpectedToken(current));
        }
    };

    let module_ident;
    let current = match current.as_ref() {
        Scan { token: Token::Ident(ident), .. } => {
            scan_next(scanner)?;
            module_ident = ident;
            current_token(scanner)?
        }
        _ => {
            return Err(ParseError::UnexpectedToken(current));
        }
    };

    let symbol: Rc<Symbol>;
    let child: Ast;
    let sibling: Ast;
    let current = match current.as_ref() {
        Scan { token: Token::Semicolon, .. } => {
            add_symbol(scope, module_ident, current.context)?;
            symbol = lookup(scope, module_ident)?;

            child = ast::leaf(NodeInfo::Ident(symbol));
            scan_next(scanner)?;
            sibling = parse_declarations(scanner, scope, &mut parse_begin_end)?;

            current_token(scanner)?
        }
        _ => {
            return Err(ParseError::UnexpectedToken(current));
        }
    };

    let current = match current.as_ref() {
        Scan { token: Token::End, .. } => {
            scan_next(scanner)?;
            current_token(scanner)?
        }
        _ => {
            return Err(ParseError::UnexpectedToken(current));
        }
    };

    let current = match current.as_ref() {
        Scan {
            token: Token::Ident(ending_ident),
            ..
        } => {
            if ending_ident != module_ident {
                return Err(ParseError::UnexpectedBlockEnding {
                    expected: String::from(module_ident),
                    found: String::from(ending_ident),
                });
            }
            scan_next(scanner)?;
            current_token(scanner)?
        }
        _ => {
            return Err(ParseError::UnexpectedToken(current));
        }
    };

    let current = match current.as_ref() {
        Scan { token: Token::Period, .. } => {
            scan_next(scanner)?;
            current_token_or_none(scanner)
        }
        _ => {
            return Err(ParseError::UnexpectedToken(current));
        }
    };

    match current {
        None => Ok(ast::node(NodeInfo::Module, child, sibling)),
        Some(scan) => Err(ParseError::UnexpectedToken(scan)),
    }
}

pub fn parse_begin_end(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    match current_token(scanner)?.as_ref() {
        Scan { token: Token::Begin, .. } => {
            scan_next(scanner)?;
            parse_statement_sequence(scanner, scope)
        }
        _ => Ok(ast::empty()),
    }
}

pub fn parse_declarations(scanner: &mut Scanner, scope: &Scope, and_then: &mut dyn FnMut(&mut Scanner, &Scope) -> ParseResult) -> ParseResult {
    // It should actually just be "parse_const_declarations".
    // Which should, in the end, try to parse the "var_declarations"
    // ...
    let mut declarations = ast::node(NodeInfo::Declarations, ast::empty(), ast::empty());

    let current = current_token_or_none(scanner);
    if let Some(scan) = current.as_ref() {
        if let Scan { token: Token::Var, .. } = scan.as_ref() {
            scan_next(scanner)?;
            // NOTE(pht) and_then is passed to parse_declaration, but in the end
            // it should be passed to the last function that parses declaration
            declarations = parse_var_declarations(scanner, scope, and_then)?;
        }
    }
    Ok(declarations)
}

pub fn parse_var_declarations(scanner: &mut Scanner, scope: &Scope, and_then: &mut dyn FnMut(&mut Scanner, &Scope) -> ParseResult) -> ParseResult {
    let declarations = recur_parse_declaration(scanner, scope)?;

    Ok(ast::node(NodeInfo::Declarations, declarations, and_then(scanner, scope)?))
}

fn recur_parse_declaration(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let current = current_token_or_none(scanner);
    if current == None {
        return Ok(ast::empty());
    }

    let idents = parse_ident_list(scanner)?;
    println!("List of idents to declare after first loop {:?}", idents);

    if idents.is_empty() {
        return Ok(ast::empty());
    }

    let mut current = current_token(scanner)?;
    println!("After var declarations, current ? {:?}", current);

    if let Scan { token: Token::Colon, .. } = current.as_ref() {
        scan_next(scanner)?;
    } else {
        return Err(ParseError::UnexpectedToken(current));
    }

    current = current_token(scanner)?;
    if let Scan {
        token: Token::Ident(type_ident),
        context: _type_ident_context,
    } = current.as_ref()
    {
        if type_ident != "INTEGER" {
            return Err(ParseError::UndefinedSymbol(String::from(type_ident)));
        }

        scan_next(scanner)?;
        current = current_token(scanner)?;
        if let Scan { token: Token::Semicolon, .. } = current.as_ref() {
            scan_next(scanner)?;

            for (ident, ident_context) in idents.iter() {
                add_symbol(scope, ident, *ident_context)?;
            }

            return var_declarations(&mut idents.iter(), scope, Type::Integer, recur_parse_declaration(scanner, scope)?);
        }
    }

    Err(ParseError::UnexpectedToken(current))
}

fn parse_ident_list(scanner: &mut Scanner) -> Result<IdentList, ParseError> {
    let mut idents: IdentList = vec![];

    let mut current;
    loop {
        current = current_token(scanner)?;
        println!("Scanning var declarations, current ? {:?}", current);
        if let Scan {
            token: Token::Ident(ident),
            context: ident_context,
        } = current.as_ref()
        {
            // NOTE(pht) Ideally, I would like not to have to clone the identifier, but since the token can fall
            // out of scope, I don't see a way to do that.
            idents.push((String::from(ident), *ident_context));
            scan_next(scanner)?;
            current = current_token(scanner)?;
            if let Scan { token: Token::Comma, .. } = current.as_ref() {
                scan_next(scanner)?;
                continue;
            } else {
                break;
            }
        }
        break;
    }
    Ok(idents)
}

pub fn var_declarations(idents: &mut dyn Iterator<Item = &(String, ScanContext)>, scope: &Scope, node_type: crate::tree::Type, final_sibling: Ast) -> ParseResult {
    match idents.next() {
        None => Ok(final_sibling),
        Some((ident, _ident_context)) => {
            let symbol = lookup(scope, ident)?;
            let child = ast::leaf(NodeInfo::Ident(symbol));
            let sibling = ast::leaf(NodeInfo::Type(node_type));
            let var = ast::node(NodeInfo::Var, child, sibling);

            Ok(ast::node(NodeInfo::Declaration, var, var_declarations(idents, scope, node_type, final_sibling)?))
        }
    }
}

pub fn parse_statement_sequence(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    println!("parse_statement_sequence {:?}", current_token(scanner));

    let first_statement = parse_statement(scanner, scope)?;
    let current = current_token_or_none(scanner);
    println!("parse_statement current ? {:?}", current);

    let next_statement = match current {
        Some(scan) => {
            if let Scan { token: Token::Semicolon, .. } = scan.as_ref() {
                scan_next(scanner)?;
                parse_statement_sequence(scanner, scope)?
            } else {
                ast::empty()
            }
        }
        _ => ast::empty(),
    };

    Ok(ast::node(NodeInfo::StatementSequence, first_statement, next_statement))
}

pub fn parse_statement(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    println!("parse_statement {:?}", current_token(scanner));
    let current = current_token(scanner)?;

    if let Scan { token: Token::Ident(ident), .. } = current.as_ref() {
        let ident_symbol = lookup(scope, ident)?;
        let subject = ast::leaf(NodeInfo::Ident(ident_symbol));

        println!("Current before calling scan_next {:?}", current_token(scanner));
        scan_next(scanner)?;
        println!("Current after calling scan_next {:?}", current_token(scanner));

        let what = current_token(scanner)?;

        println!("What ? {:?}", what);
        if what.as_ref().token == Token::Becomes {
            scan_next(scanner)?;
            return parse_assignment(subject, scanner, scope);
        }
        return Err(ParseError::UnexpectedToken(what));
    }

    if let Scan { token: Token::If, .. } = current.as_ref() {
        scan_next(scanner)?;
        return parse_if_statement(scanner, scope);
    }

    Err(ParseError::UnexpectedToken(current))
}

fn parse_assignment(subject: Rc<Tree>, scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    println!("parse_assignment {:?}", current_token(scanner));

    let object = parse_expression(scanner, scope)?;

    Ok(Rc::new(Tree::Node(TreeNode {
        info: NodeInfo::Assignement,
        child: subject,
        sibling: object,
    })))
}

pub fn parse_if_statement(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    println!("parse_if_statement {:?}", current_token(scanner));
    let test_expression = parse_expression(scanner, scope)?;

    let then_statement_sequence;
    let current = current_token(scanner)?;
    let current = match current.as_ref() {
        Scan { token: Token::Then, .. } => {
            scan_next(scanner)?;
            then_statement_sequence = parse_statement_sequence(scanner, scope)?;
            current_token(scanner)?
        }
        _ => {
            return Err(ParseError::UnexpectedToken(current));
        }
    };

    let else_statement_sequence;
    let current = match current.as_ref() {
        Scan { token: Token::Else, .. } => {
            scan_next(scanner)?;
            else_statement_sequence = parse_statement_sequence(scanner, scope)?;
            current_token(scanner)?
        }
        _ => {
            else_statement_sequence = ast::empty();
            current
        }
    };

    if let Scan { token: Token::End, .. } = current.as_ref() {
        scan_next(scanner)?;
        return Ok(ast::node(
            NodeInfo::IfStatement,
            test_expression,
            ast::node(NodeInfo::Then, then_statement_sequence, else_statement_sequence),
        ));
    }

    Err(ParseError::UnexpectedToken(current))
}

pub fn parse_expression(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let first_expression = parse_simple_expression(scanner, scope)?;

    let current = current_token_or_none(scanner);
    match current {
        None => Ok(first_expression),
        Some(scan) => match scan.token {
            Token::Eql => {
                scan_next(scanner)?;

                let second_expression = parse_simple_expression(scanner, scope)?;

                Ok(ast::node(NodeInfo::Expression(ExpressionOp::Eql), first_expression, second_expression))
            }
            _ => Ok(first_expression),
        },
    }
}

pub fn parse_simple_expression(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let mut tree = parse_term(scanner, scope)?;
    println!("parse_simple_expression ; parsed term {:?}", tree);

    loop {
        let current = current_token_or_none(scanner);
        println!("parse_simple_expression in loop, current ? {:?}", current);

        if let Some(scan) = current.as_ref() {
            let operator: Option<SimpleExpressionOp> = match scan.as_ref() {
                Scan { token: Token::Plus, .. } => Some(SimpleExpressionOp::Plus),
                Scan { token: Token::Minus, .. } => Some(SimpleExpressionOp::Minus),
                _ => None,
            };

            match operator {
                Some(operator) => {
                    println!("parse_simple_expression in loop, + found");
                    scan_next(scanner)?;
                    let sibling = parse_term(scanner, scope)?;
                    let node = TreeNode {
                        info: NodeInfo::SimpleExpression(operator),
                        child: tree,
                        sibling,
                    };
                    tree = Rc::new(Tree::Node(node));
                    continue;
                }
                None => {
                    break;
                }
            }
        }
        break;
    }
    Ok(tree)
}

pub fn parse_term(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let mut tree = parse_factor(scanner, scope)?;
    loop {
        let current = current_token_or_none(scanner);
        println!("parse_term loop, current ? {:?}", current);

        match current {
            None => {
                break;
            }
            Some(scan) => {
                let operator: Option<TermOp> = match scan.as_ref() {
                    Scan { token: Token::Times, .. } => Some(TermOp::Times),
                    Scan { token: Token::Div, .. } => Some(TermOp::Div),
                    _ => None,
                };

                match operator {
                    Some(operator) => {
                        println!("parse_simple_expression in loop, + found");
                        scan_next(scanner)?;
                        let sibling = parse_factor(scanner, scope)?;
                        let node = TreeNode {
                            info: NodeInfo::Term(operator),
                            child: tree,
                            sibling,
                        };
                        tree = Rc::new(Tree::Node(node));
                        continue;
                    }
                    None => {
                        break;
                    }
                }
            }
        }
    }
    Ok(tree)
}

pub fn parse_factor(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let mut current = current_token(scanner)?;

    if let Scan {
        token: Token::Int(constant_value),
        ..
    } = current.as_ref()
    {
        scan_next(scanner)?;
        return Ok(ast::leaf(NodeInfo::Constant(*constant_value)));
    }

    if let Scan { token: Token::Ident(ident), .. } = current.as_ref() {
        let symbol = lookup(scope, ident)?;

        scan_next(scanner)?;
        return Ok(ast::leaf(NodeInfo::Ident(symbol)));
    }

    if let Scan { token: Token::Lparen, .. } = current.as_ref() {
        scan_next(scanner)?;
        let expression = parse_expression(scanner, scope);

        current = current_token(scanner)?;
        if let Scan { token: Token::Rparen, .. } = current.as_ref() {
            scan_next(scanner)?;
            return expression;
        }
    }

    Err(ParseError::UnexpectedToken(current))
}

pub fn scan_next(scanner: &mut Scanner) -> Result<(), ParseError> {
    println!("Advancing scanner from token {:?}", scanner.current());
    match scanner.next() {
        None => Ok(()),
        Some(scan_result) => match scan_result {
            Ok(_scan) => Ok(()),
            Err(scan_error) => Err(ParseError::ScanError(scan_error)),
        },
    }
}

fn current_token(scanner: &mut Scanner) -> Result<Rc<Scan>, ParseError> {
    let current = scanner.current();
    match current {
        Some(scan) => Ok(scan),
        None => Err(ParseError::PrematureEof),
    }
}

fn current_token_or_none(scanner: &mut Scanner) -> Option<Rc<Scan>> {
    scanner.current()
}

fn add_symbol(scope: &Scope, ident: &str, context: ScanContext) -> Result<Rc<Symbol>, ParseError> {
    match scope.lookup(ident) {
        None => {
            scope.add(ident);
            lookup(scope, ident)
        }
        Some(_symbol) => Err(ParseError::SymbolAlreadyDeclared(String::from(ident), context)),
    }
}

fn lookup(scope: &Scope, ident: &str) -> Result<Rc<Symbol>, ParseError> {
    scope.lookup(ident).ok_or_else(|| ParseError::UndefinedSymbol(String::from(ident)))
}
