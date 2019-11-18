use rustpython_parser::{ast, location};

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

impl<T1: Visitor, T2: Visitor> Visitor for (T1, T2) {
    fn visit(&mut self, location: &location::Location) {
        self.0.visit(location);
        self.1.visit(location);
    }
}

impl Visitor for ast::Comprehension {
    fn visit(&mut self, location: &location::Location) {
        self.target.visit(location);
        self.iter.visit(location);
        self.ifs.visit(location);
    }
}

impl Visitor for ast::ComprehensionKind {
    fn visit(&mut self, location: &location::Location) {
        match self {
            ast::ComprehensionKind::GeneratorExpression {element} => element.visit(location),
            ast::ComprehensionKind::List {element} => element.visit(location),
            ast::ComprehensionKind::Set {element} => element.visit(location),
            ast::ComprehensionKind::Dict {key, value} => { key.visit(location); value.visit(location); },
        }
    }
}

impl Visitor for ast::Keyword {
    fn visit(&mut self, location: &location::Location) {
        self.value.visit(location);
    }
}

impl Visitor for ast::WithItem {
    fn visit(&mut self, location: &location::Location) {
        self.context_expr.visit(location);
        self.optional_vars.visit(location);
    }
}

impl Visitor for ast::ExceptHandler {
    fn visit(&mut self, location: &location::Location) {
        self.typ.visit(location);
        self.body.visit(location);
    }
}

impl Visitor for ast::Varargs {
    fn visit(&mut self, location: &location::Location) {
        match self {
            ast::Varargs::None => (),
            ast::Varargs::Unnamed => (),
            ast::Varargs::Named(paramater) => paramater.visit(location),
        }
    }
}

impl Visitor for ast::Parameter {
    fn visit(&mut self, location: &location::Location) {
        self.annotation.visit(location);
    }
}

impl Visitor for ast::Parameters {
    fn visit(&mut self, location: &location::Location) {
        self.args.visit(location);
        self.kwonlyargs.visit(location);
        self.vararg.visit(location);
        self.kwarg.visit(location);
        self.defaults.visit(location);
        self.kw_defaults.visit(location);
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
            ast::ExpressionType::Subscript {a, b} => { a.visit(location); b.visit(location); },
            ast::ExpressionType::Unop {op: _, a} => a.visit(location),
            ast::ExpressionType::Await {value} => value.visit(location),
            ast::ExpressionType::Yield {value} => value.visit(location),
            ast::ExpressionType::YieldFrom {value} => value.visit(location),
            ast::ExpressionType::Compare {vals, ops: _} => vals.visit(location),
            ast::ExpressionType::Attribute {value, name: _} => value.visit(location),
            ast::ExpressionType::Call {function, args, keywords} => { function.visit(location); args.visit(location); keywords.visit(location); },
            ast::ExpressionType::Number {value: _} => (),
            ast::ExpressionType::List {elements} => elements.visit(location),
            ast::ExpressionType::Tuple {elements} => elements.visit(location),
            ast::ExpressionType::Dict {elements} => elements.visit(location),
            ast::ExpressionType::Set {elements} => elements.visit(location),
            ast::ExpressionType::Comprehension {kind, generators} => { kind.visit(location); generators.visit(location); },
            ast::ExpressionType::Starred {value} => value.visit(location),
            ast::ExpressionType::Slice {elements} => elements.visit(location),
            ast::ExpressionType::String {value: _} => (),
            ast::ExpressionType::Bytes {value: _} => (),
            ast::ExpressionType::Identifier {name: _} => (),
            ast::ExpressionType::Lambda {args, body} => { args.visit(location); body.visit(location); },
            ast::ExpressionType::IfExpression {test, body, orelse} => { test.visit(location); body.visit(location); orelse.visit(location); },
            ast::ExpressionType::True {} => (),
            ast::ExpressionType::False {} => (),
            ast::ExpressionType::None {} => (),
            ast::ExpressionType::Ellipsis {} => (),
        }
    }
}

impl Visitor for ast::Statement {
    fn visit(&mut self, location: &location::Location) {
        match &mut self.node {
            ast::StatementType::Break => (),
            ast::StatementType::Continue => (),
            ast::StatementType::Return {value} => value.visit(location),
            ast::StatementType::Import {names: _} => (),
            ast::StatementType::ImportFrom {level: _, module: _, names: _} => (),
            ast::StatementType::Pass => (),
            ast::StatementType::Assert {test, msg} => { test.visit(location); msg.visit(location); },
            ast::StatementType::Delete {targets} => targets.visit(location),
            ast::StatementType::Assign {targets, value} => { targets.visit(location); value.visit(location); },
            ast::StatementType::AugAssign {target, op: _, value} => { target.visit(location); value.visit(location); },
            ast::StatementType::AnnAssign {target, annotation, value} => { target.visit(location); annotation.visit(location); value.visit(location); },
            ast::StatementType::Expression {expression} => expression.visit(location),
            ast::StatementType::Global {names: _} => (),
            ast::StatementType::Nonlocal {names: _} => (),
            ast::StatementType::If {test, body, orelse} => { test.visit(location); body.visit(location); orelse.visit(location); },
            ast::StatementType::While {test, body, orelse} => { test.visit(location); body.visit(location); orelse.visit(location); },
            ast::StatementType::With {is_async: _, items, body} => { items.visit(location); body.visit(location); },
            ast::StatementType::For {is_async: _, target, iter, body, orelse} => { target.visit(location); iter.visit(location); body.visit(location); orelse.visit(location); },
            ast::StatementType::Raise {exception, cause} => { exception.visit(location); cause.visit(location); },
            ast::StatementType::Try {body, handlers, orelse, finalbody} => { body.visit(location); handlers.visit(location); orelse.visit(location); finalbody.visit(location); },
            ast::StatementType::ClassDef {name: _, body, bases, keywords, decorator_list} => { body.visit(location); bases.visit(location); keywords.visit(location); decorator_list.visit(location); },
            ast::StatementType::FunctionDef {is_async: _, name: _, args, body, decorator_list, returns} => { args.visit(location); body.visit(location); decorator_list.visit(location); returns.visit(location); },
        }
    }
}