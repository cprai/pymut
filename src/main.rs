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
    import, match_class,
    PySettings,
    VirtualMachine,
};

mod traversal;
mod mutation;
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

//fn run(ast: ast::Program) -> RunResult {
//    let settings = PySettings::default();
//    let vm = VirtualMachine::new_with_callback(settings, &|ast, src| {
//        return ast;
//    });
//    let context = pyobject::PyContext::default();
//    let scope = vm.new_scope_with_builtins();
//
//    match compile::compile_program(ast, "".to_string(), 0) {
//        Ok(code_object) => {
//            match vm.run_code_obj(context.new_code_object(code_object), scope) {
//                Ok(..) => RunResult::Sucess,
//                Err(..) => RunResult::RuntimeError,
//            }
//        },
//        Err(..) => RunResult::CompileError,
//    }
//}

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

        let callback = Box::new(move |ast: ast::Program, src: &str| -> ast::Program {
            let target_file_hash = mutation_entry.file_sha1.clone();
            let file_hash = hex::encode(Sha1::digest(src.as_bytes()).as_slice());
            println!("{}", file_hash);

            if file_hash == target_file_hash {
                println!("{}", src);
            }

            return ast;
        });

        //let vm = VirtualMachine::new_with_callback(PySettings::default(), callback);
        //let vm = VirtualMachine::new(PySettings::default());
        let vm: VirtualMachine = Default::default();
        let context = pyobject::PyContext::default();
        let scope = vm.new_scope_with_builtins();
        import::init_importlib(&vm, cfg!(not(target_os = "wasi")));

        let r = run_script(&vm, scope, &command_line_options.file);
        println!("{}", r.is_err());

        //let result = match compile::compile(&file, compile::Mode::Exec, command_line_options.file.clone(), vm.settings.optimize) {
        //    Ok(code_object) => {
        //        match vm.run_code_obj(context.new_code_object(code_object), scope) {
        //            Ok(..) => RunResult::Sucess,
        //            Err(..) => RunResult::RuntimeError,
        //        }
        //    },
        //    Err(..) => RunResult::CompileError,
        //};

        /*

        let result = match vm.compile(&file, compile::Mode::Exec, command_line_options.file.clone()) {
            Ok(code_object) => {
                vm.
                match vm.run_code_obj(context.new_code_object((*code_object).code.clone() ), scope) {
                    Ok(..) => RunResult::Sucess,
                    Err(..) => RunResult::RuntimeError,
                }
            },
            Err(..) => RunResult::CompileError,
        };

        */
    }
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

use std::env;
use std::path::PathBuf;
use std::process;
use rustpython_vm::{
    util,
    obj::{objint::PyInt, objtuple::PyTuple, objtype},
    print_exception,
    pyobject::{ItemProtocol, PyResult},
    scope::Scope,
};

fn _run_string(vm: &VirtualMachine, scope: Scope, source: &str, source_path: String) -> PyResult {
    let code_obj = vm
        .compile(source, compile::Mode::Exec, source_path.clone())
        .map_err(|err| vm.new_syntax_error(&err))?;
    // trace!("Code object: {:?}", code_obj.borrow());
    scope
        .globals
        .set_item("__file__", vm.new_str(source_path), vm)?;
    vm.run_code_obj(code_obj, scope)
}

fn run_command(vm: &VirtualMachine, scope: Scope, source: String) -> PyResult<()> {
    _run_string(vm, scope, &source, "<stdin>".to_string())?;
    Ok(())
}

fn run_module(vm: &VirtualMachine, module: &str) -> PyResult<()> {
    let runpy = vm.import("runpy", &[], 0)?;
    let run_module_as_main = vm.get_attribute(runpy, "_run_module_as_main")?;
    vm.invoke(&run_module_as_main, vec![vm.new_str(module.to_owned())])?;
    Ok(())
}

fn run_script(vm: &VirtualMachine, scope: Scope, script_file: &str) -> PyResult<()> {
    // Parse an ast from it:
    let file_path = PathBuf::from(script_file);
    let file_path = if file_path.is_file() {
        file_path
    } else if file_path.is_dir() {
        let main_file_path = file_path.join("__main__.py");
        if main_file_path.is_file() {
            main_file_path
        } else {
            process::exit(1);
        }
    } else {
        process::exit(1);
    };

    let dir = file_path.parent().unwrap().to_str().unwrap().to_string();
    let sys_path = vm.get_attribute(vm.sys_module.clone(), "path").unwrap();
    vm.call_method(&sys_path, "insert", vec![vm.new_int(0), vm.new_str(dir)])?;

    match util::read_file(&file_path) {
        Ok(source) => {
            _run_string(vm, scope, &source, file_path.to_str().unwrap().to_string())?;
        }
        Err(err) => {
            process::exit(1);
        }
    }
    Ok(())
}