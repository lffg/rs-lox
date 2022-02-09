use std::{
    env, fs,
    io::{self, Write},
    path::Path,
};

use vm_lox::interpret;

fn main() -> io::Result<()> {
    match env::args().nth(1) {
        Some(path) => run_file(&path),
        _ => run_repl(),
    }
}

fn run(source: &str) {
    if source.is_empty() {
        return;
    }

    interpret(source).unwrap();
}

fn run_file(path: impl AsRef<Path>) -> io::Result<()> {
    let source = fs::read_to_string(path)?;
    run(&source);
    Ok(())
}

fn run_repl() -> io::Result<()> {
    loop {
        print!(">>> ");
        io::stdout().flush()?;

        let mut source = String::new();
        if io::stdin().read_line(&mut source)? == 0 {
            break;
        }

        run(source.trim());
    }
    Ok(())
}
