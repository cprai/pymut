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
mod serde_compatibility;
use crate::mutation::{Mutation, explore_mutations, apply_mutation};

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
    let mut program: ast::Program = parser::parse_program(&file).unwrap();

    let mutations: Vec<Mutation> = explore_mutations(&mut program);

    for mutation in mutations {
        let serialized = serde_json::to_string(&mutation).unwrap();
        println!("serialized = {}", serialized);

        let mut mutated_program = program.clone();
        apply_mutation(&mut mutated_program, mutation);

        run(mutated_program);
    }
}