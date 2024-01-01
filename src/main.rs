//! Entry point of the loxrs language implementation.
use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{stdin, Read};
use std::process::exit;

use loxrs::errors::LoxError;

fn main() -> Result<(), LoxError> {
    let args: Vec<String> = env::args().collect();

    let _ = handle_argments(args);

    Ok(())
}

fn handle_argments(args: Vec<String>) -> Result<(), LoxError> {
    // First value in argument list is `target/debug/loxrs`
    let file_path = &args[1];
    let args_query = 2;

    match args.len().cmp(&args_query) {
        Ordering::Greater => {
            eprintln!("Usage: loxrs [script]");
            exit(64);
        }
        Ordering::Equal => {
            run_files(file_path)?;
        }
        Ordering::Less => run_prompt()?,
    }

    Ok(())
}

fn run_files(file_path: &str) -> Result<(), LoxError> {
    let mut f = File::open(file_path)?;
    let mut data = vec![];

    f.read_to_end(&mut data)?;
    dbg!(data);

    Ok(())
}

fn run_prompt() -> Result<(), LoxError> {
    let mut input = String::new();

    match stdin().read_line(&mut input) {
        Ok(_) => {
            let _ = run(input);
        }
        Err(error) => println!("error: {error}"),
    }

    Ok(())
}

fn run(source: String) -> Result<(), LoxError> {
    println!("source: {:?}", source);

    Ok(())
}
