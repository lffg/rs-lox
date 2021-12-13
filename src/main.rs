use std::{
    env, fs,
    io::{stdin, stdout, Write},
    path::Path,
};

use anyhow::Result;
use lox::{
    parser::Parser,
    scanner::Scanner,
    token::{Token, TokenKind},
};

fn main() -> Result<()> {
    if let Some(script_file_name) = env::args().nth(1) {
        run_file(script_file_name)?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run(source: &str) -> Result<()> {
    let tokens = Scanner::new(source.trim())
        .scan_tokens()
        .filter(|token| {
            !matches!(
                token,
                Ok(Token {
                    kind: TokenKind::Whitespace(_),
                    ..
                }),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let ast = Parser::new(tokens).parse()?;
    println!("{:#?}", ast);
    Ok(())
}

fn run_file(file: impl AsRef<Path>) -> Result<()> {
    let source = fs::read_to_string(file)?;
    run(&source)?;
    Ok(())
}

fn run_prompt() -> Result<()> {
    println!("Welcome to rs-lox. Type Ctrl+D to exit.\n");
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
