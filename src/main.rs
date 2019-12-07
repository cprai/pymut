use std::fs;
use std::env;
use parse_display::{Display, FromStr};
use rusqlite::{Connection, Result, NO_PARAMS};
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

#[macro_use]
extern crate clap;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "Chuck Rai")]
struct CommandLineOptions {
    #[clap(short = "m", long = "mode")]
    mode: Mode,

    #[clap(short = "d", long = "database")]
    database: String,

    #[clap(short = "f", long = "file")]
    file: String,
}

#[derive(Clap, FromStr, Display)]
enum Mode {
    Explore,
    Execute,
}

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

fn explore(command_line_options: CommandLineOptions) {
    let conn = Connection::open(command_line_options.database).unwrap();

    conn.execute(
        "create table if not exists mutations (
            name text,
            mutation text
        )",
        NO_PARAMS,
    ).unwrap();

    //conn.execute(
    //    "create table if not exists ? (
    //        mutation text primary key
    //    )",
    //    &[&command_line_options.file],
    //).unwrap();

    let file = fs::read_to_string(&command_line_options.file).expect("");
    let mut program: ast::Program = parser::parse_program(&file).unwrap();

    let mutations: Vec<Mutation> = explore_mutations(&mut program);

    for mutation in mutations {
        let serialized = serde_json::to_string(&mutation).unwrap();

        conn.execute(
            "insert into mutations (
                name,
                mutation
            ) values (
                ?1,
                ?2
            )",
            &[&command_line_options.file, &serialized],
        ).unwrap();
    }
}

fn main() {
    let command_line_options = CommandLineOptions::parse();

    match command_line_options.mode {
        Mode::Execute => (),
        Mode::Explore => explore(command_line_options),
    }
}


//        let mut mutated_program = program.clone();
//        apply_mutation(&mut mutated_program, mutation);
//
//        if mutated_program == program {
//            continue;
//        }
//
//        match run(mutated_program) {
//            RunResult::Sucess => println!("-- Uncaught mutation!"),
//            _ => println!("Caught mutation"),
//        }