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
use crate::traversal::Visitor;
use crate::mutation::{Mutation, MutationType, Mutate};

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

    let mut mutations: Vec<Mutation> = Vec::new();
    {
        let mut i: u64 = 0;
        program.visit(&mut |expr| {
            i += 1;

            match expr.node {
                ast::ExpressionType::Binop {..} => {
                    {
                        let mutation = MutationType::BinaryOperatorReplacement{new_operator: ast::Operator::Mult};
                        mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                    }
                    {
                        let mutation = MutationType::BinaryOperatorReplacement{new_operator: ast::Operator::Sub};
                        mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                    }
                    {
                        let mutation = MutationType::BinaryOperatorReplacement{new_operator: ast::Operator::Pow};
                        mutations.push(Mutation{traversal_location: i, mutation_type: mutation});
                    }
                },
                _ => (),
            }
        });
    }
    let serialized = serde_json::to_string(&mutations).unwrap();
    println!("serialized = {}", serialized);

    for mutation in mutations {
        let mut mutated_program = program.clone();

        let mut i: u64 = 0;
        mutated_program.visit(&mut |expr| {
            i += 1;

            if i == mutation.traversal_location {
                expr.mutate(mutation.mutation_type.clone());
            }
        });

        run(mutated_program);
    }
}