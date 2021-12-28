# `rs-lox`

My implementation of the first interpreter presented in "[Crafting Interpreters](https://craftinginterpreters.com/)", by Robert Nystrom.

The project structure's TL;DR:

- The scanner is located in `src/parser/scanner.rs`
- The parser is located in `src/parser.rs`
- The interpreter is located in `src/interpreter.rs`

You may compile and run this project using `cargo`. Example:

```
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/lox`
Welcome to rs-lox. Enter Ctrl+D or `:exit` to exit.

>>> for (var i = 0; i < 3; i = i + 1) { print "result: " + show (1 + i * 3); }
result: 1
result: 4
result: 7
```

---

# Notes

#### TODO: 
- allow incomplete groups in the repl (i.e. start `(` or `{` in one line and finish with `)` or `}` in another one).
- maybe refactor parser grouping logic with some kind of closure to automatically open, close and register the nesting level (this "nesting level" might be an implementation detail of the above item).
- ast children api
    - question: what each ast children call should return? `vec`? `iterator`? references or boxed values?
    - the proper api? trait? function?
    - what each "node" is exactly?
    - refactor `ast::dbg::TreePrinter` to use the new ast children api