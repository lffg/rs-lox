use std::{fs, io, path::Path};

use crate::{
    interpreter::Interpreter,
    parser::{Parser, ParserOutcome},
    resolver::Resolver,
    user::diagnostic_printer::print_span_window,
};

pub mod diagnostic_printer;
pub mod repl;

fn handle_parser_outcome(
    src: &str,
    (stmts, errors): &ParserOutcome,
    interpreter: &mut Interpreter,
) -> bool {
    let writer = &mut io::stderr();

    // parser
    if !errors.is_empty() {
        for error in errors {
            eprintln!("{}\n", error);
            print_span_window(writer, src, error.primary_span());
        }
        return false;
    }

    // resolver
    let resolver = Resolver::new(interpreter);
    let (ok, errors) = resolver.resolve(stmts);
    if !ok {
        for error in errors {
            eprintln!("{}; at position {}\n", error.message, error.span);
            print_span_window(writer, src, error.span);
        }
        return false;
    }

    // interpreter
    if let Err(error) = interpreter.interpret(stmts) {
        eprintln!("{}\n", error);
        print_span_window(writer, src, error.primary_span());
        return false;
    }
    true
}

pub fn run_file(file: impl AsRef<Path>, interpreter: Option<&mut Interpreter>) -> io::Result<bool> {
    let src = &fs::read_to_string(file)?;
    let outcome = Parser::new(src).parse();
    let status = handle_parser_outcome(
        src,
        &outcome,
        interpreter.unwrap_or(&mut Interpreter::new()),
    );
    Ok(status)
}
