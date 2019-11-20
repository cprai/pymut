use std::fs;
use rustpython_parser::{ast, parser};
use rustpython_compiler::{compile};
use rustpython_vm::{
    //import, match_class,
    //obj,
    //print_exception,
    pyobject,
    //pyobject::{ItemProtocol, PyResult, PyRef},
    //scope::Scope,
    PySettings,
    VirtualMachine,
};

mod traversal;
mod mutation;
mod util;
use crate::traversal::Visitor;
use crate::mutation::Mutation;

fn take(expr: &mut ast::Expression) {
    if expr.location != ast::Location::new(24, 11) {
        return;
    }

    println!("expr{}_{}: {}", expr.location.row(), expr.location.column(), util::stringify_expression(&expr.node));

    expr.mutate(mutation::MutationType::BinaryOperatorReplacement{new_operator: ast::Operator::Sub});
}

fn take2(stmt: &mut ast::Statement) {
    //println!("stmt{}_{}: {}", stmt.location.row(), stmt.location.column(), util::stringify_statement(&stmt.node));
}

fn run(ast: ast::Program) {
    match compile::compile_program(ast, "".to_string(), 0) {
        Ok(code_object) => {
            let settings = PySettings::default();
            let vm = VirtualMachine::new(settings);
            let scope = vm.new_scope_with_builtins();
            //let code = obj::objcode::PyCode::new(code_object);
            //let codeRef = PyRef{code};
            let context = pyobject::PyContext::default();
            vm.run_code_obj(context.new_code_object(code_object), scope);
        },
        Err(error) => unreachable!(),
    }
}

fn main() {
    let file = fs::read_to_string("test.py").expect("");
    let program: ast::Program = parser::parse_program(&file).unwrap();

    let mut mutated_program = program.clone();

    mutated_program.visit((&take, &take2));

    run(program);
    run(mutated_program);
}