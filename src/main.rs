// TODO: Write tests.
// TODO: Refactor code.
// TODO: Add thiserror crate.
// TODO: Add docs.

mod chunk;
mod cli;
mod compiler;
mod error;
mod scanner;
mod value;
mod vm;

use crate::vm::InterpretResult;
use crate::vm::VirtualMachine;

use clap::Parser;
use std::path::PathBuf;

use std::fs::read_to_string;
use std::io::{stdin, stdout, Write};

use rustyline::{DefaultEditor, Result};

use std::process::exit;

pub struct CLI {}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    path: Option<PathBuf>,
    #[arg(short, long)]
    debug: bool,
}

fn repl(debug: bool) {
    let mut line = String::new();
    let mut vm = VirtualMachine::initialize();

    loop {
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut line).expect("Failed to read line");

        if line.trim().is_empty() {
            break;
        }

        vm.interpret(line.trim(), debug);
        line.clear();
    }
}

fn run_file(path: PathBuf, debug: bool) {
    let Ok(source) = read_to_string(&path) else {
        eprintln!("Could not open file {}.", path.display());
        exit(74);
    };

    let mut vm = VirtualMachine::initialize();
    match vm.interpret(&source, debug) {
        InterpretResult::CompileError => exit(65),
        InterpretResult::RuntimeError => exit(70),
        InterpretResult::Ok => (),
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.path {
        Some(path) => run_file(path, args.debug),
        None => repl(args.debug),
    }

    Ok(())
}
