use std::fs;
use sha1::{Sha1, Digest};
extern crate hex;
use parse_display::{Display, FromStr};
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
extern crate diesel;

use diesel::sqlite::Sqlite;
use diesel::insert_into;
use diesel::prelude::*;

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
    let vm = VirtualMachine::new_with_callback(settings, &|ast, src| {
        return ast;
    });
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

mod schema {
    table! {
        mutations (file_sha1, location, mutation) {
            file_sha1 -> Text,
            location -> Integer,
            mutation -> Text,
        }
    }
}

use schema::mutations;

#[derive(Insertable, Queryable, PartialEq)]
#[table_name = "mutations"]
struct MutationEntry {
    file_sha1: String,
    location: i32,
    mutation: String,
}

struct Helper {
    callback: Box<dyn Fn(ast::Program, &str) -> ast::Program>,
}

fn helper(filename: &str, callback: Box<dyn Fn(ast::Program, &str) -> ast::Program>) {
    let h = Helper{callback: callback};
    let file = fs::read_to_string(filename).expect("");
    let program: ast::Program = parser::parse_program(&file).unwrap();
    (*h.callback)(program, &file);
}

fn execute(command_line_options: CommandLineOptions) {
    let conn = SqliteConnection::establish(&command_line_options.database).unwrap();

    conn.execute(
        "create table if not exists results (
            file_sha1 text,
            location integer,
            mutation text,

            executer_sha1 text,
            result text,
            primary key (file_sha1, location, mutation, executer_sha1, result)
        )"
    ).unwrap();

    let file = fs::read_to_string(&command_line_options.file).expect("");

    use schema::mutations::dsl::*;
    let mutation_entries = mutations.load::<MutationEntry>(&conn).unwrap();

    for mutation_entry in mutation_entries {

        helper(&command_line_options.file, Box::new(move |ast: ast::Program, src: &str| -> ast::Program {
            let target_file_hash = mutation_entry.file_sha1.clone();
            println!("{}", mutation_entry.mutation);
            return ast;
        }));

        //let callback = |ast: ast::Program, src: &str| -> ast::Program {
        //    let newast = ast.clone();
        //    let target_file_hash = mutation_entry.file_sha1.clone();
        //    let file_hash = hex::encode(Sha1::digest(src.as_bytes()).as_slice());
        //    if 2==3 {
        //        return newast;
        //    }

        //    return newast;
        //};

        //unsafe {
        //    let vm = VirtualMachine::new_with_callback(PySettings::default(), &callback);
        //}

    }

    //let vm = VirtualMachine::new_with_callback(PySettings::default(), &|ast, src| {
    //    return ast;
    //});
}

fn explore(command_line_options: CommandLineOptions) {
    let conn = SqliteConnection::establish(&command_line_options.database).unwrap();

    conn.execute(
        "create table if not exists mutations (
            file_sha1 text,
            location integer,
            mutation text,
            primary key (file_sha1, location, mutation)
        )"
    ).unwrap();

    let file = fs::read_to_string(&command_line_options.file).expect("");
    let mut program: ast::Program = parser::parse_program(&file).unwrap();

    let found_mutations: Vec<Mutation> = explore_mutations(&mut program);

    for found_mutation in found_mutations {
        use schema::mutations::dsl::*;

        let entry = MutationEntry {
            file_sha1: hex::encode(Sha1::digest(file.as_bytes()).as_slice()),
            location: found_mutation.traversal_location as i32,
            mutation: serde_json::to_string(&found_mutation.mutation_type).unwrap(),
        };

        insert_into(mutations).values(entry).execute(&conn);
    }
}

fn main() {
    let command_line_options = CommandLineOptions::parse();

    match command_line_options.mode {
        Mode::Execute => execute(command_line_options),
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