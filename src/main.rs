pub mod chunk;
pub mod compiler;
pub mod scanner;
pub mod value;
pub mod vm;

use crate::vm::InterpretResult;
use crate::vm::VirtualMachine;

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
            eprintln!("Usage: lox [path]");
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

        if line.trim().is_empty() {
            break;
        }

        VirtualMachine::interpret(line.trim());
        line.clear();
    }
}

fn run_file(path: &String) {
    let Ok(source) = read_to_string(path) else {
        eprintln!("Could not open file {}.", path);
        exit(74);
    };

    match VirtualMachine::interpret(&source) {
        InterpretResult::CompileError => exit(65),
        InterpretResult::RuntimeError => exit(70),
        InterpretResult::Ok => (),
    };
}
