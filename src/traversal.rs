use rustpython_parser::{ast, parser, location};

enum ASTNode {
    Some(ast::Expression),
    Some(ast::Statement),
    Some(ast::Suite),
}

fn expression_children(expression: ast::Expression) -> Vec<ASTNode> {
    match expression.node {
        ast::ExpressionType::Identifier {name} => Vec::new(),
        ast::ExpressionType::Number {value} => Vec::new(),
        ast::ExpressionType::BoolOp {op, values} => values,
        ast::ExpressionType::Binop {a, op, b} => vec![*a, *b],
        ast::ExpressionType::Unop {op, a} => vec![*a],
        ast::ExpressionType::Compare {vals, ops} => vals,
        ast::ExpressionType::True => Vec::new(),
        ast::ExpressionType::False => Vec::new(),
        ast::ExpressionType::None => Vec::new(),

        _ => unreachable!(),
    }
}

fn statement_children(statement: ast::Statement) -> Vec<ASTNode> {
    match statement.node {
        ast::StatementType::Break => Vec::new(),
        ast::StatementType::Continue => Vec::new(),
        ast::StatementType::Pass => Vec::new(),
        ast::StatementType::Assign {targets, value} => {targets.push(value); targets},
        ast::StatementType::Expression {expression} => vec![expression],
        //ast::StatementType::If {test, body, orelse} => 
        //ast::StatementType::While {test, body, orelse} => {

        _ => unreachable!(),
    }
}

fn suite_children(suite: ast::Suite) -> Vec<ASTNode> {
    return suite;
}