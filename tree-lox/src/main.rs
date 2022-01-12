use std::{env, io};

use tree_lox::user;

fn main() -> io::Result<()> {
    match env::args().nth(1) {
        Some(path) => user::run_file(path, None).map(drop),
        _ => user::repl::Repl::run(),
    }
}
