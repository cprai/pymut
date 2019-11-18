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

fn id_from_location(prefix: &str, location: &location::Location) -> String {
    format!("_{}_line{}_{}", prefix, location.row(), location.column())
}

fn main() {
    let file = fs::read_to_string("test.py").expect("");
    //let program: ast::Program = mutate(parser::parse_program(&file).unwrap(), ast::Location::new(20, 5));
    let mut program: ast::Program = parser::parse_program(&file).unwrap();
    //let program2: ast::Program = program;
    //let program3: ast::Program = program2;

    program.statements.visit(&ast::Location::new(24, 11));
    match compile::compile_program(program, "".to_string(), 0) {
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