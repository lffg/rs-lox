use std::io::{self, Write};

use crate::{
    ast,
    interpreter::Interpreter,
    parser::{error::ParseError, Parser},
    user::{handle_parser_outcome, run_file},
};

pub struct Repl {
    interpreter: Interpreter,
    current_src: String,
    show_lex: bool,
    show_ast: bool,
    done: bool,
}

impl Repl {
    pub fn run() -> io::Result<()> {
        Self::new().start()
    }

    fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            current_src: "".into(),
            show_lex: false,
            show_ast: false,
            done: false,
        }
    }

    fn start(mut self) -> io::Result<()> {
        eprintln!("Welcome to rs-lox. Enter Ctrl+D or `:exit` to exit.\n");

        while !self.done {
            let (line, is_eof) = self.read_line()?;

            // If previous line started with `:`, interpret it as a command and
            // skip this iteration entirely, handling the command.
            if let Some(raw_cmd) = line.trim().strip_prefix(':') {
                self.handle_command(raw_cmd);
                continue;
            }

            // Accumulate the input.
            self.current_src += &line;

            let mut parser = Parser::new(&self.current_src);
            parser.options.repl_mode = true;
            let outcome @ (stmts, errors) = &parser.parse();

            // If the parser produced an error, but the error allows REPL continuation then we
            // just continue to ask for successive user inputs.
            if !errors.is_empty() && Self::should_continue_repl(errors) && !is_eof {
                continue;
            }

            // If the user asks so, show them some debug information before any code is interpreted
            // or errors are emitted.
            if self.show_lex && !self.current_src.trim().is_empty() {
                ast::dbg::print_scanned_tokens(&self.current_src);
            }
            if self.show_ast && !stmts.is_empty() {
                ast::dbg::print_program_tree(stmts);
            }

            handle_parser_outcome(&self.current_src, outcome, &mut self.interpreter);
            self.current_src = "".into();
        }
        Ok(())
    }

    fn read_line(&mut self) -> io::Result<(String, bool)> {
        let prompt = if self.current_src.is_empty() {
            ">>>"
        } else {
            "..."
        };
        print!("{} ", prompt);
        io::stdout().flush()?;

        let mut line = String::new();
        let is_eof = io::stdin().read_line(&mut line)? == 0;
        self.done = is_eof && self.current_src.is_empty();

        if is_eof {
            println!();
        }

        Ok((line, is_eof))
    }

    fn should_continue_repl(errors: &[ParseError]) -> bool {
        !errors.is_empty() && errors.iter().all(ParseError::allows_continuation)
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
            "load" => match run_file(cmd[1], Some(&mut self.interpreter)) {
                Ok(status) => {
                    if status {
                        println!("ok");
                    }
                }
                Err(error) => eprintln!("{}", error),
            },

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
