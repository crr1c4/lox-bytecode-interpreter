mod cli;
mod error;
mod lox;

use anyhow::Result;
use clap::Parser;
use cli::*;

fn main() -> Result<()> {
    let args = Args::parse();

    match args.path {
        Some(path) => run_file(path),
        None => run_prompt(),
    };

    Ok(())
}
