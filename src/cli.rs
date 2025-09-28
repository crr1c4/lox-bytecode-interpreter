use crate::error::LexicalError;
use crate::lox;
use crate::lox::scanner::Scanner;
use crate::lox::token::Token;
use anyhow::Result;
use clap::Parser;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::collections::VecDeque;
// use std::fmt::Result;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::exit;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(short, long)]
    pub path: Option<PathBuf>,
    // #[arg(short, long)]
    // pub debug: bool,
}

pub fn run_prompt() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let possible_error = match rl.readline(">> ") {
            Ok(line) => run(line),
            Err(ReadlineError::Interrupted) => {
                println!("Proccess terminated.");
                break;
            }
            Err(err) => {
                eprintln!("Error {err}");
                break;
            }
        };

        if possible_error.is_err() {
            eprintln!("{:?}", possible_error.unwrap_err());
        }
    }

    Ok(())
}

pub fn run_file(path: PathBuf) -> Result<()> {
    let Ok(source) = read_to_string(&path) else {
        eprintln!("Could not open file {}.", path.display());
        exit(74);
    };

    run(source)?;

    Ok(())
}

fn run(source: String) -> anyhow::Result<()> {
    let scanner = Scanner::new(source);
    let tokens: VecDeque<Token> = scanner.collect::<Result<_, _>>()?;
    for token in tokens {
        println!("{token:?}")
    }

    Ok(())
}
