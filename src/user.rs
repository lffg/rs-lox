use std::{
    fs,
    io::{self, Write},
    path::Path,
};

use crate::{
    ast::{self, stmt::Stmt},
    interpreter::Interpreter,
    parser::{error::ParseError, Parser},
};

fn maybe_print_parse_errors(errors: &[ParseError]) {
    if !errors.is_empty() {
        for error in errors {
            eprintln!("{}", error);
        }
    }
}

pub fn run_file(file: impl AsRef<Path>) -> io::Result<()> {
    let src = fs::read_to_string(file)?;
    let (stmts, errors, _) = Parser::new(&src).parse();
    maybe_print_parse_errors(&errors);
    if errors.is_empty() {
        if let Err(error) = Interpreter::new().interpret(&stmts) {
            eprintln!("{}", error);
        }
    }
    Ok(())
}

pub fn run_repl() -> io::Result<()> {
    Repl::run()
}

struct Repl {
    current_src: Option<String>,
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
            current_src: None,
            show_lex: false,
            show_ast: false,
            done: false,
        }
    }

    fn start(mut self) -> io::Result<()> {
        eprintln!("Welcome to rs-lox. Enter Ctrl+D or `:exit` to exit.\n");

        while !self.done {
            let (line, is_eof) = self.read_line()?;

            if is_eof {
                eprintln!();
            }

            if let Some(raw_cmd) = line.trim().strip_prefix(':') {
                self.handle_command(raw_cmd);
                continue;
            }

            match self.current_src.as_mut() {
                Some(string) => string.push_str(&line),
                None => self.current_src = Some(line),
            }

            let mut parser = Parser::new(self.current_src.as_ref().unwrap());
            parser.options.repl_mode = true;
            let (stmts, errors, allow_continuation) = parser.parse();

            if self.show_ast && !stmts.is_empty() {
                ast::dbg::print_program_tree(&stmts);
            }
            if !errors.is_empty() {
                if !allow_continuation || is_eof {
                    maybe_print_parse_errors(&errors);
                    self.current_src = None;
                }
                continue;
            }
            self.interpret(&stmts)
        }
        Ok(())
    }

    fn interpret(&mut self, stmts: &[Stmt]) {
        if let Err(error) = self.interpreter.interpret(stmts) {
            eprintln!("{}", error);
        }
        self.current_src = None;
    }

    fn read_line(&mut self) -> io::Result<(String, bool)> {
        let prompt = if self.current_src.is_none() {
            ">>>"
        } else {
            "..."
        };
        print!("{} ", prompt);
        io::stdout().flush()?;

        let mut line = String::new();
        let is_eof = io::stdin().read_line(&mut line)? == 0;
        self.done = is_eof && self.current_src.is_none();
        Ok((line, is_eof))
    }

    fn handle_command(&mut self, raw_cmd: &str) {
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
}

macro_rules! handle_bool_opt {
    ($self:ident . $option:ident) => {{
        $self.$option = !$self.$option;
        let status = if $self.$option { "ON" } else { "OFF" };
        println!("Toggled `{}` option {}.", stringify!($option), status);
    }};
}
use handle_bool_opt;
