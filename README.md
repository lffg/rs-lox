# Crafting Interpreters in Rust – `rs-lox`

My implementation of the first interpreter presented in "[Crafting Interpreters](https://craftinginterpreters.com/)", by Robert Nystrom.

This repository holds two implementations for the same language. The first one, `tree-lox` implements a tree-based interpreter and the other, `vm-lox` uses a bytecode virtual machine to interpret the code.

### Quick start

This project can by compiled using Cargo. Use the `--project` flag to choose the implementation.

Run an existing file:

```terminal
$ cargo r -p tree-lox -- script-name.lox
```

Open the REPL:

```
$ cargo r -p tree-lox
```

### Project overview

todo

### License

Code licensed under the MIT license.