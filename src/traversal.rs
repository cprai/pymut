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
            ast::ExpressionType::Binop {a, op: _, b} => { a.visit(location); b.visit(location); },
            //ast::ExpressionType::Subscript {} => ,
            ast::ExpressionType::Unop {op: _, a} => a.visit(location),
            ast::ExpressionType::Await {value} => value.visit(location),
            ast::ExpressionType::Yield {value} => value.visit(location),
            ast::ExpressionType::YieldFrom {value} => value.visit(location),
            ast::ExpressionType::Compare {vals, ops: _} => vals.visit(location),
            //ast::ExpressionType::Attribute {} => ,
            //ast::ExpressionType::Call {} => ,
            //ast::ExpressionType::Number {} => ,
            ast::ExpressionType::List {elements} => elements.visit(location),
            ast::ExpressionType::Tuple {elements} => elements.visit(location),
            //ast::ExpressionType::Dict {} => ,
            ast::ExpressionType::Set {elements} => elements.visit(location),
            //ast::ExpressionType::Comprehension {} => ,
            ast::ExpressionType::Starred {value} => value.visit(location),
            ast::ExpressionType::Slice {elements} => elements.visit(location),
            //ast::ExpressionType::String {} => ,
            //ast::ExpressionType::Bytes {} => ,
            //ast::ExpressionType::Identifier {} => ,
            //ast::ExpressionType::Lambda {} => ,
            ast::ExpressionType::IfExpression {test, body, orelse} => { test.visit(location); body.visit(location); orelse.visit(location); },
            ast::ExpressionType::True {} => (),
            ast::ExpressionType::False {} => (),
            ast::ExpressionType::None {} => (),
            ast::ExpressionType::Ellipsis {} => (),

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