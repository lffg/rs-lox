use std::{
    env, fs,
    io::{stdin, stdout, Write},
    path::Path,
};

use anyhow::Result;
use lox::{interpreter::Interpreter, parser::Parser};

fn main() -> Result<()> {
    if let Some(script_file_name) = env::args().nth(1) {
        run_file(script_file_name)?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run(src: &str) -> Result<()> {
    let (stmts, errors) = Parser::new(src).parse();

    if !errors.is_empty() {
        assert!(stmts.is_empty());
        for error in errors {
            eprintln!("{}", error);
        }
    } else {
        let mut interpreter = Interpreter;
        if let Err(error) = interpreter.interpret(&stmts) {
            eprintln!("{}", error);
        }
    }
    Ok(())
}

fn run_file(file: impl AsRef<Path>) -> Result<()> {
    let source = fs::read_to_string(file)?;
    run(&source)
}

fn run_prompt() -> Result<()> {
    eprintln!("Welcome to rs-lox. Enter Ctrl+D or `:exit` to exit.\n");
    loop {
        print!("> ");
        stdout().flush()?;

        let mut source = String::new();
        stdin().read_line(&mut source)?;

        if source.is_empty() {
            return Ok(());
        }
        let source = source.trim();

        if let Some(tail) = source.strip_prefix(':') {
            let cmd: Vec<_> = tail
                .split_ascii_whitespace()
                .filter(|s| !s.is_empty())
                .collect();
            match *cmd.first().unwrap_or(&"<empty>") {
                "exit" => return Ok(()),
                "eval" => {
                    for file in &cmd[1..] {
                        eprintln!("Evaluating `{}`...", file);
                        if let Err(err) = run_file(file) {
                            eprintln!("  error: {}", err);
                        }
                    }
                }
                "help" => eprintln!(":exit | :eval a b ... | :help"),
                invalid => eprintln!(
                    "The command `{}` is not valid. Type `:help` for guidance.",
                    invalid
                ),
            }
            continue;
        }

        run(source).unwrap_or_else(|err| {
            eprintln!("{:?}", err);
        });
    }
}
