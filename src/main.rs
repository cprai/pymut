use std::fs;
use std::env;
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

enum RunResult {
    Sucess,
    CompileError,
    RuntimeError,
    Timeout,
}

fn run(ast: ast::Program) -> RunResult {
    let settings = PySettings::default();
    let vm = VirtualMachine::new(settings);
    let context = pyobject::PyContext::default();
    let scope = vm.new_scope_with_builtins();

    match compile::compile_program(ast, "".to_string(), 0) {
        Ok(code_object) => {
            match vm.run_code_obj(context.new_code_object(code_object), scope) {
                Ok(..) => RunResult::Sucess,
                Err(..) => RunResult::RuntimeError,
            }
        },
        Err(..) => RunResult::CompileError,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("");
    let mut program: ast::Program = parser::parse_program(&file).unwrap();

    let mutations: Vec<Mutation> = explore_mutations(&mut program);

    for mutation in mutations {
        //let serialized = serde_json::to_string(&mutation).unwrap();
        //println!("serialized = {}", serialized);

        let mut mutated_program = program.clone();
        apply_mutation(&mut mutated_program, mutation);

        if mutated_program == program {
            continue;
        }

        match run(mutated_program) {
            RunResult::Sucess => println!("-- Uncaught mutation!"),
            _ => println!("Caught mutation"),
        }
    }
}