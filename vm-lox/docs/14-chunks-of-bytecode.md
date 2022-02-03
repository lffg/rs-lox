# Chunks of Bytecode

## Why use bytecode?

In the first implementation, `tree-lox`, the strategy used to interpret the user code was to user a tree-walk interpreter. Though simple to implement, this approach has some drawbacks, such—and most notably—as the poor performance.

Interpreting the code by walking the AST is slow because such data structure is not memory-efficient to that use case. Each syntax node of the language is represented by a single AST node, which may reference other AST nodes. Since they may be located in different memory locations, they don't have an ideal cache locality.

By using bytecode, which is a _super condensed_ representation of the source code, most of those problems are attenuated or resolved. The main cost is a slight more complex interpreter implementation. Since the language execution _must_ have a good performance that cost is definitely worth it.

Bytecode has some similarities with machine code. The main differences are speed (being machine code the fastest), portability and ease of implementation. Compiling to a simple bytecode, designed for a specific language, is tremendously straightforward than compiling to machine code, which of course varies between different architectures.

## Starting out

The original C implementation implements a bytecode chunk using a `Chunk` structure, with a dynamic array of bytes, which would represent the bytecode instructions and instruction arguments.

In this Rust implementation, a bytecode chunk is implemented using a `Chunk` structure, with a vector of `Ins` (an enum, named after the abbreviation of "instruction"). Of course the variants of such enum should not get too big. This size shall be carefully observed. Using an enum in this case is less memory efficient; however, due to the ease of implementation and its idiomatic characteristic in Rust, such approach was preferred.

A disassembler is implemented in order to debug the bytecode. The `std::fmt::Debug` crate was used, both in the `Chunk` structure and in the `Ins` enum.

## Bytecode constants

In the C implementation, constants were stored in a separate dynamic array under the `Chunk` struct. The constant instruction is then followed by a index to such array, for example:

```
...
OP_CONSTANT 5
...
```

Being `5` the index in the `constants` array.

In this implementation, since a Rust enum is being used, the `Ins` can directly carry the `Value` (which is also an enum). This makes the implementation a little easier and less error prone. Of course, though, the `Value` enum must now be quite small in memory size.

The same approach was used to keep track of line numbers, a separate vector in the `Chunk` struct. Direct correspondence with the `code` vector. In the future I might revisit this approach to be more memory efficient. (TODO: lffg)

