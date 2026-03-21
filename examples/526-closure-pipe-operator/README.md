📖 **[View on hightechmind.io →](https://hightechmind.io/rust/526-closure-pipe-operator)**

---

# Pipe Operator Simulation
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

OCaml's `|>` (pipe) operator, F#'s `|>`, and Elixir's `|>` all solve the same readability problem: deeply nested function calls read inside-out, but data transformations are conceptually left-to-right. `f(g(h(x)))` is hard to read; `x |> h |> g |> f` reads as a pipeline. Rust does not have a native pipe operator, but the pattern can be simulated with an extension trait `Pipe` that adds `.pipe(f)` to every type. This lets Rust code express transformation pipelines in the same left-to-right style as functional languages.

## Learning Outcomes

- How to implement `|>` semantics using an extension trait with a blanket impl
- How `pipe`, `pipe_ref`, and `pipe_mut` handle different ownership scenarios
- How function composition with `compose` and `compose_n` relates to piping
- Where the pipe pattern appears: data transformation, validation chains, error propagation
- Why Rust does not have a native pipe operator and what RFC proposals exist

## Rust Application

The `Pipe` trait provides three methods: `pipe<B, F: FnOnce(Self) -> B>(self, f: F) -> B` (consuming), `pipe_ref<B, F: FnOnce(&Self) -> B>(&self, f: F) -> B` (borrowing), and `pipe_mut<B, F: FnOnce(&mut Self) -> B>(&mut self, f: F) -> B` (mutable). A blanket `impl<T> Pipe for T {}` adds these to every type. `compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C` creates a combined function. Helper functions `double`, `add1`, `square`, `to_string`, and `prefix` demonstrate chaining: `5.pipe(double).pipe(add1).pipe(square).pipe(to_string).pipe(prefix)`.

Key patterns:
- `fn pipe<B, F: FnOnce(Self) -> B>(self, f: F) -> B { f(self) }` — trivial but powerful
- Blanket `impl<T> Pipe for T {}` — zero-cost, no allocation, inlined by compiler
- `compose` for creating reusable combined transformations

## OCaml Approach

OCaml has `|>` as a built-in operator in the standard library since OCaml 4.01. It is simply defined as `let (|>) x f = f x`. No extension traits or special syntax are needed — it is universally available and composes with every function.

```ocaml
5 |> double |> add1 |> square |> string_of_int |> (fun s -> "Result: " ^ s)
```

## Key Differences

1. **Language support**: OCaml has `|>` as a stdlib operator available everywhere; Rust requires either an extension trait (library-level) or the nightly `|>` RFC (not yet stabilized).
2. **Ownership variants**: Rust needs three pipe variants (`pipe`, `pipe_ref`, `pipe_mut`) for owned, borrowed, and mutable cases; OCaml has one `|>` since all values are GC-managed.
3. **Composition**: Rust's `compose` returns `impl Fn` — an anonymous type; OCaml function composition with `>>` or `@@` returns a plain function value visible to the type system.
4. **Inline cost**: Rust's blanket `impl<T> Pipe for T` is always inlined at zero cost; OCaml's `|>` is a regular function call, optimized away by the compiler in most cases.

## Exercises

1. **Pipe with error**: Implement `pipe_result<T, U, E, F: FnOnce(T) -> Result<U, E>>(self: Result<T, E>, f: F) -> Result<U, E>` as a method on `Result` to chain fallible transformations.
2. **Three-stage pipeline**: Write a validation pipeline using `.pipe` that parses a string to integer, multiplies by 2, and formats as `"value: N"` — all expressed as a left-to-right chain.
3. **Compose chain**: Use `compose` to build a single `fn(i32) -> String` that triples, negates, adds 100, and converts to string, then benchmark it against the equivalent direct call.
