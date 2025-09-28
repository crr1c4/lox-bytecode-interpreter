use clap::Parser;
use std::path::PathBuf;

use std::fs::read_to_string;
// use std::io::{stdin, stdout, Write};

use rustyline::error::ReadlineError;
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

fn repl(debug: bool) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut vm = VirtualMachine::initialize();

    loop {
        match rl.readline(">> ") {
            Ok(line) => vm.interpret(line.trim(), debug),
            Err(ReadlineError::Interrupted) => {
                println!("Proccess terminated.");
                break;
            }
            Err(err) => {
                eprintln!("Error {err}");
                break;
            }
        };
    }

    Ok(())
}

fn run_file(path: PathBuf, debug: bool) -> Result<()> {
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

    Ok(())
}

// use crate::vm::InterpretResult;
// use crate::vm::VirtualMachine;
fn main() -> Result<()> {
    let args = Args::parse();

    match args.path {
        Some(path) => run_file(path, args.debug),
        None => repl(args.debug),
    };

    Ok(())
}
