use std::{
    env, fs,
    io::{stdin, stdout, Write},
    path::Path,
};

use anyhow::Result;
use lox::{
    ast::dbg::TreePrinter,
    interpreter::Interpreter,
    parser::{scanner::Scanner, Parser},
};

#[derive(Debug, Default, Clone)]
struct ReplOptions {
    pub show_lex: bool,
    pub show_tree: bool,
}

macro_rules! handle_bool_opt {
    ($struct:ident . $option:ident) => {{
        $struct.$option = !$struct.$option;
        let status = if $struct.$option { "ON" } else { "OFF" };
        println!("Toggled `{}` option {}.", stringify!($option), status);
    }};
}

fn main() -> Result<()> {
    if let Some(script_file_name) = env::args().nth(1) {
        run_file(script_file_name, &ReplOptions::default())?;
    } else {
        run_prompt()?;
    }
    Ok(())
}

fn run(src: &str, interpreter: &mut Interpreter, options: &ReplOptions) {
    if options.show_lex {
        let scanner = Scanner::new(src);
        println!(/*                */ "┌─");
        scanner.for_each(|t| println!("│ {:?}", t));
        println!(/*                */ "└─");
    }

    let mut parser = Parser::new(src);
    parser.options.repl_mode = true;
    let (stmts, errors) = parser.parse();

    if !stmts.is_empty() && options.show_tree {
        println!(/*   */ "┌─");
        TreePrinter::new("│ ").print_stmts(&stmts);
        println!(/*   */ "└─");
    }

    if !errors.is_empty() {
        for error in errors {
            eprintln!("{}", error);
        }
        return;
    }

    if let Err(error) = interpreter.interpret(&stmts) {
        eprintln!("{}", error);
    }
}

fn run_file(file: impl AsRef<Path>, options: &ReplOptions) -> Result<()> {
    let source = fs::read_to_string(file)?;
    let mut interpreter = Interpreter::new();
    run(&source, &mut interpreter, options);
    Ok(())
}

fn run_prompt() -> Result<()> {
    eprintln!("Welcome to rs-lox. Enter Ctrl+D or `:exit` to exit.\n");

    let mut options = ReplOptions::default();
    let mut interpreter = Interpreter::new();

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
                        if let Err(err) = run_file(file, &options) {
                            eprintln!("  error: {}", err);
                        }
                    }
                }
                "lex" => handle_bool_opt!(options.show_lex),
                "tree" => handle_bool_opt!(options.show_tree),
                "help" => eprintln!(":exit | :eval a b ... | :tree | :lex | :help"),
                invalid => eprintln!(
                    "The command `{}` is not valid. Type `:help` for guidance.",
                    invalid
                ),
            }
            continue;
        }

        run(source, &mut interpreter, &options);
    }
}
