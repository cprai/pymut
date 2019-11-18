use rustpython_parser::{ast, parser, location};

pub trait Visitor {
    fn visit(&mut self, location: &location::Location);
}

impl<T: Visitor> Visitor for Option<T> {
    fn visit(&self, location: &location::Location) {
        match self {
            Some(thing) => thing.visit(location),
            None => (),
        }
    }
}

impl Visitor for ast::Expression {
    fn visit(&self, location: &location::Location) {
        if &self.location == location {
            println!("whoopie!");
            match &self.node {
                ast::ExpressionType::Binop {a, op, b} => op = &ast::Operator::Mult,

                _ => (),
            }
        }

        // recurse
        match &self.node {
            ast::ExpressionType::BoolOp {op, values} => values.iter().for_each(|expression| expression.visit(location)),
            ast::ExpressionType::Binop {a, op, b} => {(*a).visit(location); (*b).visit(location);},
            ast::ExpressionType::Unop {op, a} => (*a).visit(location),
            ast::ExpressionType::Compare {vals, ops} => vals.iter().for_each(|expression| expression.visit(location)),

            _ => (),
        }
    }
}

impl Visitor for ast::Statement {
    fn visit(&self, location: &location::Location) {
        match &self.node {
            ast::StatementType::Break => (),
            ast::StatementType::Continue => (),
            ast::StatementType::Return {value} => {
                value.visit(location);
            },
            ast::StatementType::Pass => (),
            ast::StatementType::Assert {test, msg} => {
                test.visit(location);
                msg.visit(location);
            }
            ast::StatementType::Assign {targets, value} => {
                for expression in targets {
                    expression.visit(location);
                }
                value.visit(location);
            },
            ast::StatementType::Expression {expression} => expression.visit(location),
            ast::StatementType::If {test, body, orelse} => {
                test.visit(location);
                body.visit(location);
                orelse.visit(location);
            },
            ast::StatementType::While {test, body, orelse} => {
                test.visit(location);
                body.visit(location);
                orelse.visit(location);
            },

            _ => unreachable!(),
        }
    }
}

impl Visitor for ast::Suite {
    fn visit(&mut self, location: &location::Location) {
        for statement in self {
            statement.visit(location);
        }
    }
}