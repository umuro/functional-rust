📖 **[View on hightechmind.io →](https://hightechmind.io/rust/184-free-monad-intro)**

---

# Introduction to Free Monads
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Free monads separate the description of a computation from its execution. A program built from free monad operations is a pure data structure — a tree of instructions. Different interpreters can then execute the same program in different ways: one runs it against a real console, another tests it with mocked input/output, a third logs every operation. This separation enables dependency injection at the computation level, pure testing without mocks, and multiple execution strategies for the same program description.

## Learning Outcomes

- Understand the free monad as a way to separate program description from interpretation
- Learn to build a simple console DSL using the `Console<A>` enum
- See how `Print` and `GetLine` operations form a tree of deferred computations
- Understand the connection between free monads and effect systems

## Rust Application

`Console<A>` has three variants: `Pure(A)` (computation complete), `Print(String, Box<Console<A>>)` (print then continue), and `GetLine(Box<dyn FnOnce(String) -> Console<A>>)` (read then continue). A program like "print greeting, read name, print hello" is built as nested `Print` and `GetLine` values — a pure data structure. The interpreter function `run_console` pattern matches on each variant and executes it, calling the continuation with the result.

## OCaml Approach

OCaml's free monad uses GADTs for type safety:
```ocaml
type _ console =
  | Pure : 'a -> 'a console
  | Print : string * (unit -> 'a console) -> 'a console
  | GetLine : (string -> 'a console) -> 'a console
let ( >>= ) : 'a console -> ('a -> 'b console) -> 'b console = ...
```
OCaml's `let*` syntax makes writing free monad programs feel almost imperative:
```ocaml
let program = let* () = print "Enter name: " in
              let* name = get_line () in
              print ("Hello, " ^ name)
```

## Key Differences

1. **`let*` syntax**: OCaml's monadic bind notation makes free monad programs readable; Rust lacks similar syntax sugar, requiring explicit closures.
2. **FnOnce continuations**: Rust uses `Box<dyn FnOnce(String) -> Console<A>>` for `GetLine`; OCaml uses `string -> 'a console` — structurally identical.
3. **Functor instance**: OCaml's free monad requires a `Functor` instance on the base functor; Rust builds the monad directly as an enum without abstracting over the functor.
4. **Performance**: Free monads build a heap-allocated tree; interpreting large programs is slower than direct execution; algebraic effects (example 189) are more efficient.

## Exercises

1. Add a `ReadFile(path: String, continuation: Box<dyn FnOnce(String) -> Console<A>>)` operation.
2. Write a test interpreter that provides mock input and captures output in a `Vec<String>`.
3. Implement a logging interpreter that wraps every operation with `println!("[LOG] ...")` before executing it.
