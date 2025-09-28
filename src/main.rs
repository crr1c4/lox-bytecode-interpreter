mod lox;
mod cli;
mod error;

use clap::Parser;
use anyhow::Result;
use cli::*;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.path {
        Some(path) => run_file(path),
        None => run_prompt(),
    };


    Ok(())
}

