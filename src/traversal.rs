use rustpython_parser::ast;

pub trait Visitor {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression));
}

impl Visitor for ast::Program {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        self.statements.visit(callback);
    }
}

impl<T: Visitor> Visitor for Option<T> {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        match self {
            Some(visitable) => visitable.visit(callback),
            None => (),
        }
    }
}

impl<T: Visitor> Visitor for Vec<T> {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        for visitable in self {
            visitable.visit(callback);
        }
    }
}

impl<T: Visitor> Visitor for Box<T> {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        let visitable: &mut T = &mut *self;
        visitable.visit(callback);
    }
}

impl<T1: Visitor, T2: Visitor> Visitor for (T1, T2) {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        self.0.visit(callback);
        self.1.visit(callback);
    }
}

impl Visitor for ast::Comprehension {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        self.target.visit(callback);
        self.iter.visit(callback);
        self.ifs.visit(callback);
    }
}

impl Visitor for ast::ComprehensionKind {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        match self {
            ast::ComprehensionKind::GeneratorExpression {element} => element.visit(callback),
            ast::ComprehensionKind::List {element} => element.visit(callback),
            ast::ComprehensionKind::Set {element} => element.visit(callback),
            ast::ComprehensionKind::Dict {key, value} => { key.visit(callback); value.visit(callback); },
        }
    }
}

impl Visitor for ast::Keyword {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        self.value.visit(callback);
    }
}

impl Visitor for ast::WithItem {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        self.context_expr.visit(callback);
        self.optional_vars.visit(callback);
    }
}

impl Visitor for ast::ExceptHandler {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        self.typ.visit(callback);
        self.body.visit(callback);
    }
}

impl Visitor for ast::Varargs {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        match self {
            ast::Varargs::None => (),
            ast::Varargs::Unnamed => (),
            ast::Varargs::Named(paramater) => paramater.visit(callback),
        }
    }
}

impl Visitor for ast::Parameter {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        self.annotation.visit(callback);
    }
}

impl Visitor for ast::Parameters {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        self.args.visit(callback);
        self.kwonlyargs.visit(callback);
        self.vararg.visit(callback);
        self.kwarg.visit(callback);
        self.defaults.visit(callback);
        self.kw_defaults.visit(callback);
    }
}

impl Visitor for ast::Expression {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        callback(self);

        match &mut self.node {
            ast::ExpressionType::BoolOp {op: _, values} => values.visit(callback),
            ast::ExpressionType::Binop {a, op: _, b} => { a.visit(callback); b.visit(callback); },
            ast::ExpressionType::Subscript {a, b} => { a.visit(callback); b.visit(callback); },
            ast::ExpressionType::Unop {op: _, a} => a.visit(callback),
            ast::ExpressionType::Await {value} => value.visit(callback),
            ast::ExpressionType::Yield {value} => value.visit(callback),
            ast::ExpressionType::YieldFrom {value} => value.visit(callback),
            ast::ExpressionType::Compare {vals, ops: _} => vals.visit(callback),
            ast::ExpressionType::Attribute {value, name: _} => value.visit(callback),
            ast::ExpressionType::Call {function, args, keywords} => { function.visit(callback); args.visit(callback); keywords.visit(callback); },
            ast::ExpressionType::Number {value: _} => (),
            ast::ExpressionType::List {elements} => elements.visit(callback),
            ast::ExpressionType::Tuple {elements} => elements.visit(callback),
            ast::ExpressionType::Dict {elements} => elements.visit(callback),
            ast::ExpressionType::Set {elements} => elements.visit(callback),
            ast::ExpressionType::Comprehension {kind, generators} => { kind.visit(callback); generators.visit(callback); },
            ast::ExpressionType::Starred {value} => value.visit(callback),
            ast::ExpressionType::Slice {elements} => elements.visit(callback),
            ast::ExpressionType::String {value: _} => (),
            ast::ExpressionType::Bytes {value: _} => (),
            ast::ExpressionType::Identifier {name: _} => (),
            ast::ExpressionType::Lambda {args, body} => { args.visit(callback); body.visit(callback); },
            ast::ExpressionType::IfExpression {test, body, orelse} => { test.visit(callback); body.visit(callback); orelse.visit(callback); },
            ast::ExpressionType::True {} => (),
            ast::ExpressionType::False {} => (),
            ast::ExpressionType::None {} => (),
            ast::ExpressionType::Ellipsis {} => (),
        }
    }
}

impl Visitor for ast::Statement {
    fn visit(&mut self, callback: &mut dyn FnMut(&mut ast::Expression)) {
        match &mut self.node {
            ast::StatementType::Break => (),
            ast::StatementType::Continue => (),
            ast::StatementType::Return {value} => value.visit(callback),
            ast::StatementType::Import {names: _} => (),
            ast::StatementType::ImportFrom {level: _, module: _, names: _} => (),
            ast::StatementType::Pass => (),
            ast::StatementType::Assert {test, msg} => { test.visit(callback); msg.visit(callback); },
            ast::StatementType::Delete {targets} => targets.visit(callback),
            ast::StatementType::Assign {targets, value} => { targets.visit(callback); value.visit(callback); },
            ast::StatementType::AugAssign {target, op: _, value} => { target.visit(callback); value.visit(callback); },
            ast::StatementType::AnnAssign {target, annotation, value} => { target.visit(callback); annotation.visit(callback); value.visit(callback); },
            ast::StatementType::Expression {expression} => expression.visit(callback),
            ast::StatementType::Global {names: _} => (),
            ast::StatementType::Nonlocal {names: _} => (),
            ast::StatementType::If {test, body, orelse} => { test.visit(callback); body.visit(callback); orelse.visit(callback); },
            ast::StatementType::While {test, body, orelse} => { test.visit(callback); body.visit(callback); orelse.visit(callback); },
            ast::StatementType::With {is_async: _, items, body} => { items.visit(callback); body.visit(callback); },
            ast::StatementType::For {is_async: _, target, iter, body, orelse} => { target.visit(callback); iter.visit(callback); body.visit(callback); orelse.visit(callback); },
            ast::StatementType::Raise {exception, cause} => { exception.visit(callback); cause.visit(callback); },
            ast::StatementType::Try {body, handlers, orelse, finalbody} => { body.visit(callback); handlers.visit(callback); orelse.visit(callback); finalbody.visit(callback); },
            ast::StatementType::ClassDef {name: _, body, bases, keywords, decorator_list} => { body.visit(callback); bases.visit(callback); keywords.visit(callback); decorator_list.visit(callback); },
            ast::StatementType::FunctionDef {is_async: _, name: _, args, body, decorator_list, returns} => { args.visit(callback); body.visit(callback); decorator_list.visit(callback); returns.visit(callback); },
        }
    }
}