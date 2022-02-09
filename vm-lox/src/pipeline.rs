use crate::scanner::Scanner;

/// Represents an error within the Lox interpretation pipeline.
// TODO: impl Error + Display
#[derive(Debug)]
pub enum Error {
    CompileError(String),
    RuntimeError(String),
}

/// A specialized Result type for the Lox interpretation pipeline.
pub type Result<T> = std::result::Result<T, Error>;

/// Runs the Lox interpretation pipeline (scanning, parsing, compiling and interpretation).
pub fn interpret(source: &str) -> Result<()> {
    let scanner = Scanner::new(source);

    for token in scanner {
        println!("{:?}", token);
    }

    Ok(())
}
