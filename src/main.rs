// TODO: Add tests
// TODO: Finish scanner (check book because I need to refactor the code).

pub mod chunk;
pub mod compiler;
pub mod debug;
pub mod scanner;
pub mod value;
pub mod vm;

// use crate::chunk::*;
// use crate::debug::disassemble_chunk;
use crate::vm::InterpretResult;
// use crate::vm::VirtualMachine;
use crate::compiler::compile;

use std::env::args;
use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            eprintln!("Usage: rlox [path]");
            exit(64);
        }
    };
}

fn repl() {
    let mut line = String::new();

    loop {
        print!("> ");
        stdout().flush().unwrap();

        stdin().read_line(&mut line).expect("Failed to read line");

        if line.trim().eq("") {
            break;
        }

        interpret(&line);
        line.clear();
    }
}

fn run_file(path: &String) {
    let Ok(source) = read_to_string(path) else {
        eprintln!("Could not open file {}.", path);
        exit(74);
    };

    let result = interpret(&source);
    if result == InterpretResult::CompileError {
        exit(65);
    }
    if result == InterpretResult::RuntimeError {
        exit(70);
    }
}

fn interpret(source: &String) -> InterpretResult {
    compile(source);
    InterpretResult::Ok
}
