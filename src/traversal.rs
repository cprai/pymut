use rustpython_parser::{ast, parser, location};

pub trait Visitor {
    fn visit(&mut self, location: &location::Location);
}

impl<T: Visitor> Visitor for Option<T> {
    fn visit(&mut self, location: &location::Location) {
        match self {
            Some(thing) => thing.visit(location),
            None => (),
        }
    }
}

impl<T: Visitor> Visitor for Vec<T> {
    fn visit(&mut self, location: &location::Location) {
        for visitable in self {
            visitable.visit(location);
        }
    }
}

impl<T: Visitor> Visitor for Box<T> {
    fn visit(&mut self, location: &location::Location) {
        let visitable: &mut T = &mut *self;
        visitable.visit(location);
    }
}

impl Visitor for ast::Expression {
    fn visit(&mut self, location: &location::Location) {
        if &mut self.location == location {
            println!("whoopie!");
            match &mut self.node {
                ast::ExpressionType::Binop {a, op, b} => *op = ast::Operator::Mult,

                _ => (),
            }
        }

        // recurse
        match &mut self.node {
            ast::ExpressionType::BoolOp {op: _, values} => values.visit(location),
            ast::ExpressionType::Binop {a, op: _, b} => {a.visit(location); b.visit(location);},//println!("{} {}", self.location.row(), self.location.column())},
            ast::ExpressionType::Unop {op: _, a} => a.visit(location),
            ast::ExpressionType::Compare {vals, ops: _} => vals.visit(location),

            _ => (),
        }
    }
}

impl Visitor for ast::Statement {
    fn visit(&mut self, location: &location::Location) {
        match &mut self.node {
            ast::StatementType::Break => (),
            ast::StatementType::Continue => (),
            ast::StatementType::Return {value} => value.visit(location),
            // Import
            // ImportFrom
            ast::StatementType::Pass => (),
            ast::StatementType::Assert {test, msg} => { test.visit(location); msg.visit(location); },
            ast::StatementType::Delete {targets} => targets.visit(location),
            ast::StatementType::Assign {targets, value} => { targets.visit(location); value.visit(location); },
            // AugAssign
            // AnnAssign
            ast::StatementType::Expression {expression} => expression.visit(location),
            // Global
            // Nonlocal
            ast::StatementType::If {test, body, orelse} => { test.visit(location); body.visit(location); orelse.visit(location); },
            ast::StatementType::While {test, body, orelse} => { test.visit(location); body.visit(location); orelse.visit(location); },
            // With
            // For
            ast::StatementType::Raise {exception, cause} => { exception.visit(location); cause.visit(location); },
            // Try
            // ClassDef
            // FunctionDef

            _ => unreachable!(),
        }
    }
}