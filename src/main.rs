use std::{
    env, fs,
    io::{stdin, stdout, Write},
    path::Path,
};

use anyhow::Result;
use lox::{ast::dbg::TreePrinter, interpreter::Interpreter, parser::Parser};

fn main() -> Result<()> {
    if let Some(script_file_name) = env::args().nth(1) {
        run_file(script_file_name)?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run(src: &str, show_tree: bool) -> Result<()> {
    let mut parser = Parser::new(src);
    parser.options.repl_mode = true;
    let (stmts, errors) = parser.parse();

    if !errors.is_empty() {
        assert!(stmts.is_empty());
        for error in errors {
            eprintln!("{}", error);
        }
    } else {
        let mut interpreter = Interpreter;
        if show_tree {
            for stmt in &stmts {
                println!("┌─");
                TreePrinter::new("│ ").print_stmt(stmt);
                println!("└─")
            }
        }
        if let Err(error) = interpreter.interpret(&stmts) {
            eprintln!("{}", error);
        }
    }
    Ok(())
}

fn run_file(file: impl AsRef<Path>) -> Result<()> {
    let source = fs::read_to_string(file)?;
    run(&source, false)
}

fn run_prompt() -> Result<()> {
    eprintln!("Welcome to rs-lox. Enter Ctrl+D or `:exit` to exit.\n");

    let mut show_tree = false;

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
                "tree" => {
                    show_tree = !show_tree;
                    let status = if show_tree { "ON" } else { "OFF" };
                    println!("Toggled `show_tree` option to {}.", status);
                }
                "help" => eprintln!(":exit | :eval a b ... | :tree | :help"),
                invalid => eprintln!(
                    "The command `{}` is not valid. Type `:help` for guidance.",
                    invalid
                ),
            }
            continue;
        }

        run(source, show_tree).unwrap_or_else(|err| {
            eprintln!("{:?}", err);
        });
    }
}
