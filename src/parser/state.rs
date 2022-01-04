#[derive(Debug, Default)]
pub struct ParserOptions {
    pub repl_mode: bool,
}

#[derive(Debug, Default)]
pub struct ParserContext {
    pub within_fn: bool,
}
