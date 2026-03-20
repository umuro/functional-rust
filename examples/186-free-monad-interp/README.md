📖 **[View on hightechmind.io →](https://hightechmind.io/rust/186-free-monad-interp)**

---

# Free Monad Interpreters

## Problem Statement

The power of free monads is multiple interpreters for the same program. A console DSL program can be interpreted by a "real" interpreter that reads from stdin and writes to stdout, a "test" interpreter that uses mock I/O with predetermined inputs, and a "logging" interpreter that records every operation for debugging. The program description is shared; only the interpreter changes. This is the essence of dependency injection at the computation level.

## Learning Outcomes

- Implement multiple interpreters for the same free monad DSL
- Understand how different interpreters provide testability, logging, and simulation
- Learn the interpreter pattern: fold the free monad tree with different algebras
- See the analogy to dependency injection: swapping interpreters changes the computation's context

## Rust Application

The real interpreter `run_io(program: Console<A>)` matches `Pure(a) => a`, `Print(msg, k) => { println!("{}", msg); run_io(k()) }`, `ReadLine(k) => { let line = read_from_stdin(); run_io(k(line)) }`. The test interpreter `run_test(program, inputs: &mut Vec<String>, outputs: &mut Vec<String>)` pops from `inputs` for `ReadLine` and pushes to `outputs` for `Print`. Both interpreters share no code — they are independent traversals of the same program tree.

## OCaml Approach

OCaml's interpreter pattern is identical in structure. The `run_io` and `run_test` functions use `let rec` and pattern matching. OCaml's effect handlers (OCaml 5) provide a more efficient alternative to free monads for the same separation: effects are declared, then handled by different effect handlers — the operational semantics is similar to free monad interpreters but implemented at the runtime level.

## Key Differences

1. **Effect handlers vs. free monads**: OCaml 5's effect handlers are a native runtime mechanism; free monads are a library pattern — effect handlers are faster and more composable.
2. **Mutual recursion**: OCaml's `run_io` and `run_test` can be `let rec`; Rust's interpreter functions are plain functions called recursively.
3. **State threading**: The test interpreter threads mutable state (`inputs`, `outputs`) through the traversal; in OCaml this is often done with a state monad or `ref` cells.
4. **Stack depth**: Both interpreters recurse on the continuation tree — deep programs risk stack overflow; trampolining (example 197) resolves this.

## Exercises

1. Implement a "counting" interpreter that returns the number of I/O operations performed.
2. Write a "recording" interpreter that captures the full execution trace as a `Vec<(Operation, Value)>`.
3. Add a timeout interpreter: if the program performs more than N `ReadLine` operations, terminate with an error.
