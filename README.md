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

# Notes

#### TODO: 

- Better diagnostics (presentation).
- `self.maybe_trailing_semicolon();`