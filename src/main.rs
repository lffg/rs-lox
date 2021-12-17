use std::{
    env, fs,
    io::{stdin, stdout, Write},
    path::Path,
};

use anyhow::Result;

fn main() -> Result<()> {
    if let Some(script_file_name) = env::args().nth(1) {
        run_file(script_file_name)?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run(source: &str) -> Result<()> {
    println!("you entered `{:?}`", source);
    Ok(())
}

fn run_file(file: impl AsRef<Path>) -> Result<()> {
    let source = fs::read_to_string(file)?;
    run(&source)
}

fn run_prompt() -> Result<()> {
    println!("Welcome to rs-lox. Enter Ctrl+D to exit.\n");
    loop {
        print!("> ");
        stdout().flush()?;

        let mut source = String::new();
        stdin().read_line(&mut source)?;

        if source.is_empty() {
            println!("\nbye");
            return Ok(());
        }

        run(&source).unwrap_or_else(|err| {
            eprintln!("{:?}", err);
        });
    }
}
