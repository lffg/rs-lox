use std::{
    env, fs,
    io::{stdin, stdout, Write},
    path::Path,
};

use anyhow::Result;
use lox::scanner::Scanner;

fn main() -> Result<()> {
    if let Some(script_file_name) = env::args().nth(1) {
        run_file(script_file_name)?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run(source: &str) -> Result<()> {
    let mut scanner = Scanner::new(source.trim());
    for token in scanner.scan_tokens() {
        println!("token: {:?}", token);
    }
    Ok(())
}

fn run_file(file: impl AsRef<Path>) -> Result<()> {
    let source = fs::read_to_string(file)?;
    run(&source)?;
    Ok(())
}

fn run_prompt() -> Result<()> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut source = String::new();
        stdin().read_line(&mut source)?;

        if source.is_empty() {
            println!("\nbye");
            return Ok(());
        }

        run(&source)?;
    }
}
