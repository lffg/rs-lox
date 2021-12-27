use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use crate::{
    ast,
    interpreter::Interpreter,
    parser::{Parser, ParserOutcome},
};

pub fn handle_parser_outcome((stmts, errors): ParserOutcome, interpreter: &mut Interpreter) {
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

pub fn run_file(file: impl AsRef<Path>) -> io::Result<()> {
    let src = fs::read_to_string(file)?;
    let parsed = Parser::new(&src).parse();
    handle_parser_outcome(parsed, &mut Interpreter::new());
    Ok(())
}

pub fn run_repl() -> io::Result<()> {
    Repl::run()
}

struct Repl {
    interpreter: Interpreter,
    show_lex: bool,
    show_ast: bool,
    done: bool,
}

impl Repl {
    fn run() -> io::Result<()> {
        Self::new().start()
    }

    fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            show_lex: false,
            show_ast: false,
            done: false,
        }
    }

    fn start(mut self) -> io::Result<()> {
        while !self.done {
            let line = self.read_line()?;

            if let Some(raw_cmd) = line.trim().strip_prefix(':') {
                self.handle_command(raw_cmd);
                continue;
            }

            let mut parser = Parser::new(&line);
            parser.options.repl_mode = true;
            let (stmts, errors) = parser.parse();

            if self.show_ast && !stmts.is_empty() {
                ast::dbg::print_program_tree(&stmts);
            }
            handle_parser_outcome((stmts, errors), &mut self.interpreter);
        }
        Ok(())
    }

    pub fn handle_command(&mut self, raw_cmd: &str) {
        let cmd: Vec<_> = raw_cmd
            .split_ascii_whitespace()
            .filter(|s| !s.is_empty())
            .collect();
        match *cmd.first().unwrap_or(&"") {
            "exit" => self.done = true,
            "ast" | "tree" => handle_bool_opt!(self.show_ast),
            "lex" => handle_bool_opt!(self.show_lex),
            "help" => eprintln!(":exit | :lex | :ast | :help"),
            _ => eprintln!("Invalid command. Type `:help` for guidance."),
        }
    }

    pub fn read_line(&mut self) -> io::Result<String> {
        print!(">>> ");
        io::stdout().flush()?;

        let mut line = String::new();
        // If `Ctrl+D` (user's Eof), `read_line` returns `0` (bytes read).
        // In such case, `self.done` must be set to `true`.
        self.done = io::stdin().read_line(&mut line)? == 0;
        Ok(line)
    }
}

macro_rules! handle_bool_opt {
    ($self:ident . $option:ident) => {{
        $self.$option = !$self.$option;
        let status = if $self.$option { "ON" } else { "OFF" };
        println!("Toggled `{}` option {}.", stringify!($option), status);
    }};
}
use handle_bool_opt;
