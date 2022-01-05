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
) {
    let writer = &mut io::stderr();

    // parser
    if !errors.is_empty() {
        for error in errors {
            eprintln!("{}\n", error);
            print_span_window(writer, src, error.primary_span());
        }
        return;
    }

    // resolver
    let resolver = Resolver::new(interpreter);
    let (ok, errors) = resolver.resolve(stmts);
    if !ok {
        for error in errors {
            eprintln!("{}; at position {}\n", error.message, error.span);
            print_span_window(writer, src, error.span);
        }
        return;
    }

    // interpreter
    if let Err(error) = interpreter.interpret(stmts) {
        eprintln!("{}\n", error);
        print_span_window(writer, src, error.primary_span());
    }
}

pub fn run_file(file: impl AsRef<Path>) -> io::Result<()> {
    let src = &fs::read_to_string(file)?;
    let outcome = Parser::new(src).parse();
    handle_parser_outcome(src, &outcome, &mut Interpreter::new());
    Ok(())
}
