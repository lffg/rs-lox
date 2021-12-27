use std::{env, io};

use lox::user;

fn main() -> io::Result<()> {
    match env::args().nth(1) {
        Some(path) => user::run_file(path),
        _ => user::run_repl(),
    }
}
