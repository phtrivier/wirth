use log::debug;
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
            // NOTE(pht) I have no idea what the _size_ of a module should be, or where it should be in memory.
            // Probably it will be the size of the functions, maybe ? In which case you can only add the entry later ?
            let module_size = 1;

            add_symbol(scope, module_ident, module_size, current.context)?;
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
    if current.is_none() {
        return Ok(ast::empty());
    }

    let idents = parse_ident_list(scanner)?;
    debug!("List of idents to declare after first loop {:?}", idents);

    if idents.is_empty() {
        return Ok(ast::empty());
    }

    let mut current = current_token(scanner)?;
    debug!("After var declarations, current ? {:?}", current);

    if let Scan { token: Token::Colon, .. } = current.as_ref() {
        scan_next(scanner)?;
    } else {
        return Err(ParseError::UnexpectedToken(current));
    }

    current = current_token(scanner)?;

    if let Scan {
        token: Token::Array,
        context: _array_ident_context,
    } = current.as_ref()
    {
        debug!("Parsing array type declaration");
        scan_next(scanner)?;
        debug!("Scanned next token");
        current = current_token(scanner)?;
        debug!("Found current token {:?}", current);

        if let Scan {
            token: Token::Int(array_capacity),
            ..
        } = current.as_ref()
        {
            scan_next(scanner)?;

            let mut current = current_token(scanner)?;
            debug!("Found current token {:?}", current);

            if let Scan { token: Token::Of, .. } = current.as_ref() {
                scan_next(scanner)?;
                current = current_token(scanner)?;

                if let Scan {
                    token: Token::Ident(type_ident), ..
                } = current.as_ref()
                {
                    // NOTE(pht) will have to be relaxed to allow nested arrays or
                    // arrays of records
                    if type_ident != "INTEGER" {
                        return Err(ParseError::UndefinedSymbol(String::from(type_ident)));
                    }

                    scan_next(scanner)?;
                    current = current_token(scanner)?;
                    if let Scan { token: Token::Semicolon, .. } = current.as_ref() {
                        scan_next(scanner)?;

                        for (ident, ident_context) in idents.iter() {
                            // TODO(pht) add some capacity info into the identified, otherwise we won't be able to
                            // remember the size ?
                            add_symbol(scope, ident, *array_capacity, *ident_context)?;
                        }

                        return var_declarations(&mut idents.iter(), scope, VarType::Array(*array_capacity), recur_parse_declaration(scanner, scope)?);
                    }
                }
            }
        }
    }

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
                add_symbol(scope, ident, 1, *ident_context)?;
            }

            return var_declarations(&mut idents.iter(), scope, VarType::Integer, recur_parse_declaration(scanner, scope)?);
        }
    }

    Err(ParseError::UnexpectedToken(current))
}

fn parse_ident_list(scanner: &mut Scanner) -> Result<IdentList, ParseError> {
    let mut idents: IdentList = vec![];

    let mut current;
    loop {
        current = current_token(scanner)?;
        debug!("Scanning var declarations, current ? {:?}", current);
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

pub fn var_declarations(idents: &mut dyn Iterator<Item = &(String, ScanContext)>, scope: &Scope, node_type: crate::tree::VarType, final_sibling: Ast) -> ParseResult {
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
    debug!("parse_statement_sequence {:?}", current_token(scanner));

    let first_statement = parse_statement(scanner, scope)?;
    let current = current_token_or_none(scanner);
    debug!("parse_statement current ? {:?}", current);

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
    debug!("parse_statement {:?}", current_token(scanner));
    let mut current = current_token(scanner)?;

    if let Scan { token: Token::Ident(ident), .. } = current.as_ref() {
        let subject = parse_ident_with_selector(scanner, scope, ident)?;

        current = current_token(scanner)?;
        if current.as_ref().token == Token::Becomes {
            scan_next(scanner)?;
            return parse_assignment(subject, scanner, scope);
        }
        return Err(ParseError::UnexpectedToken(current));
    }

    if let Scan { token: Token::If, .. } = current.as_ref() {
        scan_next(scanner)?;
        return parse_if_statement(scanner, scope);
    }

    if let Scan { token: Token::While, .. } = current.as_ref() {
        scan_next(scanner)?;
        return parse_while_statement(scanner, scope);
    }

    Err(ParseError::UnexpectedToken(current))
}

fn parse_assignment(subject: Rc<Tree>, scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    debug!("parse_assignment {:?}", current_token(scanner));

    let object = parse_expression(scanner, scope)?;

    Ok(Rc::new(Tree::Node(TreeNode {
        info: NodeInfo::Assignement,
        child: subject,
        sibling: object,
    })))
}

pub fn parse_if_statement(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    debug!("parse_if_statement {:?}", current_token(scanner));
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
            else_statement_sequence = ast::node(NodeInfo::Else, parse_statement_sequence(scanner, scope)?, ast::empty());
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

pub fn parse_while_statement(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    debug!("parse_while_statement {:?}", current_token(scanner));
    let test_expression = parse_expression(scanner, scope)?;

    let do_statement_sequence;
    let current = current_token(scanner)?;
    let current = match current.as_ref() {
        Scan { token: Token::Do, .. } => {
            scan_next(scanner)?;
            do_statement_sequence = parse_statement_sequence(scanner, scope)?;
            current_token(scanner)?
        }
        _ => {
            return Err(ParseError::UnexpectedToken(current));
        }
    };

    if let Scan { token: Token::End, .. } = current.as_ref() {
        scan_next(scanner)?;
        return Ok(ast::node(
            NodeInfo::WhileStatement,
            test_expression,
            ast::node(NodeInfo::Do, do_statement_sequence, ast::empty()),
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
            Token::Eql => parse_expression_relation(scanner, scope, first_expression, ExpressionOp::Eql),
            Token::Neq => parse_expression_relation(scanner, scope, first_expression, ExpressionOp::Neq),
            Token::Lss => parse_expression_relation(scanner, scope, first_expression, ExpressionOp::Lss),
            Token::Leq => parse_expression_relation(scanner, scope, first_expression, ExpressionOp::Leq),
            Token::Gtr => parse_expression_relation(scanner, scope, first_expression, ExpressionOp::Gtr),
            Token::Geq => parse_expression_relation(scanner, scope, first_expression, ExpressionOp::Geq),
            _ => Ok(first_expression),
        },
    }
}

pub fn parse_expression_relation(scanner: &mut Scanner, scope: &Scope, first_expression: Rc<Tree>, expression_op: ExpressionOp) -> ParseResult {
    scan_next(scanner)?;
    let second_expression = parse_simple_expression(scanner, scope)?;
    Ok(ast::node(NodeInfo::Expression(expression_op), first_expression, second_expression))
}

pub fn parse_simple_expression(scanner: &mut Scanner, scope: &Scope) -> ParseResult {
    let mut tree = parse_term(scanner, scope)?;
    debug!("parse_simple_expression ; parsed term {:?}", tree);

    loop {
        let current = current_token_or_none(scanner);
        debug!("parse_simple_expression in loop, current ? {:?}", current);

        if let Some(scan) = current.as_ref() {
            let operator: Option<SimpleExpressionOp> = match scan.as_ref() {
                Scan { token: Token::Plus, .. } => Some(SimpleExpressionOp::Plus),
                Scan { token: Token::Minus, .. } => Some(SimpleExpressionOp::Minus),
                _ => None,
            };

            match operator {
                Some(operator) => {
                    debug!("parse_simple_expression in loop, + found");
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
        debug!("parse_term loop, current ? {:?}", current);

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
                        debug!("parse_simple_expression in loop, + found");
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
        return parse_ident_with_selector(scanner, scope, ident);
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

pub fn parse_ident_with_selector(scanner: &mut Scanner, scope: &Scope, ident: &str) -> ParseResult {
    let symbol = lookup(scope, ident)?;

    scan_next(scanner)?;

    let maybe_selector_start = current_token_or_none(scanner);
    debug!("maybe selector start {:?}", maybe_selector_start);

    match maybe_selector_start {
        None => Ok(ast::leaf(NodeInfo::Ident(symbol))),
        Some(scan) => {
            if let Scan { token: Token::Lbrak, .. } = scan.as_ref() {
                scan_next(scanner)?;
                let current = current_token(scanner)?;

                // NOTE(pht) this only allows constant and ident access at the moment
                if let Scan {
                    token: Token::Int(constant_value),
                    ..
                } = current.as_ref()
                {
                    scan_next(scanner)?;
                    let current = current_token(scanner)?;

                    if let Scan { token: Token::Rbrak, .. } = current.as_ref() {
                        scan_next(scanner)?;
                        let child = ast::leaf(NodeInfo::Constant(*constant_value));
                        return Ok(ast::node(NodeInfo::Ident(symbol), child, ast::empty()));
                    }
                    return Err(ParseError::UnexpectedToken(current));
                }

                if let Scan {
                    token: Token::Ident(index_ident), ..
                } = current.as_ref()
                {
                    scan_next(scanner)?;
                    let current = current_token(scanner)?;

                    if let Scan { token: Token::Rbrak, .. } = current.as_ref() {
                        scan_next(scanner)?;

                        let index_symbol = lookup(scope, index_ident)?;
                        let child = ast::leaf(NodeInfo::Ident(index_symbol));
                        return Ok(ast::node(NodeInfo::Ident(symbol), child, ast::empty()));
                    }
                    return Err(ParseError::UnexpectedToken(current));
                }

                Err(ParseError::UnexpectedToken(current))
            } else {
                Ok(ast::leaf(NodeInfo::Ident(symbol)))
            }
        }
    }
}

pub fn scan_next(scanner: &mut Scanner) -> Result<(), ParseError> {
    debug!("Advancing scanner from token {:?}", scanner.current());
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

fn add_symbol(scope: &Scope, ident: &str, size: u32, context: ScanContext) -> Result<Rc<Symbol>, ParseError> {
    match scope.lookup(ident) {
        None => {
            scope.add_with_size(ident, size as usize);
            lookup(scope, ident)
        }
        Some(_symbol) => Err(ParseError::SymbolAlreadyDeclared(String::from(ident), context)),
    }
}

fn lookup(scope: &Scope, ident: &str) -> Result<Rc<Symbol>, ParseError> {
    scope.lookup(ident).ok_or_else(|| ParseError::UndefinedSymbol(String::from(ident)))
}
