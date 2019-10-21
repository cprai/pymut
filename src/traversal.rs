use rustpython_parser::{ast, parser, location};

trait ASTNode {
    fn children(&self) -> Vec<Box<dyn ASTNode>>;
}

impl ASTNode for ast::Expression {
    fn children(&self) -> Vec<dyn ASTNode> {
        match &self.node {
            ast::ExpressionType::Identifier {name} => Vec::new(),
            ast::ExpressionType::Number {value} => Vec::new(),
            ast::ExpressionType::BoolOp {op, values} => {
                let ret: Vec<&dyn ASTNode> = Vec::new();
                for expression in values {
                    ret.push(&expression);
                }
                ret
            },
            ast::ExpressionType::Binop {a, op, b} => vec![&*a, &*b],
            ast::ExpressionType::Unop {op, a} => vec![&*a],
            ast::ExpressionType::Compare {vals, ops} => {
                let ret: Vec<&dyn ASTNode> = Vec::new();
                for expression in vals {
                    ret.push(&expression);
                }
                ret
            },
            ast::ExpressionType::True => Vec::new(),
            ast::ExpressionType::False => Vec::new(),
            ast::ExpressionType::None => Vec::new(),

            _ => unreachable!(),
        }
    }
}

impl ASTNode for &ast::Statement {
    fn children(&self) -> Vec<&dyn ASTNode> {
        match &self.node {
            ast::StatementType::Break => Vec::new(),
            ast::StatementType::Continue => Vec::new(),
            ast::StatementType::Pass => Vec::new(),
            ast::StatementType::Assign {targets, value} => {
                let ret: Vec<&dyn ASTNode> = Vec::new();
                for expression in targets {
                    ret.push(&expression);
                }
                ret.push(&value);
                ret
            },
            ast::StatementType::Expression {expression} => vec![&expression],
            //ast::StatementType::If {test, body, orelse} => 
            //ast::StatementType::While {test, body, orelse} => {

            _ => unreachable!(),
        }
    }
}