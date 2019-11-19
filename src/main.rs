use std::fs;
use rustpython_parser::{ast, parser, location};
use rustpython_compiler::{compile};
use rustpython_vm::{
    import, match_class,
    obj,
    print_exception,
    pyobject,
    pyobject::{ItemProtocol, PyResult, PyRef},
    scope::Scope,
    util, PySettings, VirtualMachine,
};

mod traversal;
use crate::traversal::Visitor;

fn mutate(ast: ast::Program, location: location::Location) -> ast::Program {
    return ast;
}

fn take(expr: &mut ast::Expression) {
    println!("expr{}_{}", expr.location.row(), expr.location.column());
}

fn take2(expr: &mut ast::Statement) {
    println!("stmt{}_{}", expr.location.row(), expr.location.column());
}

fn main() {
    let file = fs::read_to_string("django.py").expect("");
    //let program: ast::Program = mutate(parser::parse_program(&file).unwrap(), ast::Location::new(20, 5));
    let mut program: ast::Program = parser::parse_program(&file).unwrap();
    //let program2: ast::Program = program;
    //let program3: ast::Program = program2;

    //program.visit(&ast::Location::new(319, 30));

    //program.visit(&|expr: ast::Expression| println!("{} {}", expr.location.row, expr.location.column));
    program.visit((&take, &take2));


    //match compile::compile_program(program, "".to_string(), 0) {
    //    Ok(code_object) => {
    //        let settings = PySettings::default();
    //        let vm = VirtualMachine::new(settings);
    //        let scope = vm.new_scope_with_builtins();
    //        //let code = obj::objcode::PyCode::new(code_object);
    //        //let codeRef = PyRef{code};
    //        let context = pyobject::PyContext::default();
    //        vm.run_code_obj(context.new_code_object(code_object), scope);
    //    },
    //    Err(error) => unreachable!(),
    //}
}