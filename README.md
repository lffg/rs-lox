# Crafting Interpreters in Rust – `rs-lox`

My implementation of the first interpreter presented in "[Crafting Interpreters](https://craftinginterpreters.com/)", by Robert Nystrom.

The project structure's TL;DR:

- The scanner is located in `src/parser/scanner.rs`
- The parser is located in `src/parser.rs`
- The interpreter is located in `src/interpreter.rs`

You may compile and run this project using `cargo`. Example:

```
$ cargo run --quiet
Welcome to rs-lox. Enter Ctrl+D or `:exit` to exit.

>>> for (var i = 1; i <= 3; i = i + 1) {
...   print i;
... }
1
2
3
```

---

Things I could do better next:
- Type checking?
- Improve the semantic analysis (the current `resolver` implementation doesn't make me happy).
- Improve the diagnostics presentation.
- Improve the diagnostic API. Currently each interpreter component has its own error implementation details (aka `Error` token from the scanner, `ParseError`, `ResolveError`, `RuntimeError`, etc).
- Handle different files with some kind of unit API and then maybe implement a basic module system. Currently the REPL has a problem related to this due to the loss of a previous "prompt file" string.
- Improve the "pipeline" structure.
- Improve the environment handler. Linked hash map ideas don't look minimally efficient to me.