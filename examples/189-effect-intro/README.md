📖 **[View on hightechmind.io →](https://hightechmind.io/rust/189-effect-intro)**

---

# Introduction to Algebraic Effects

## Problem Statement

Algebraic effects are a modern alternative to monads for structuring side effects. Unlike free monads (which build heap-allocated trees), algebraic effects are handled at the runtime level — the effect is "thrown" and a handler "catches" it, potentially resuming the computation. OCaml 5 introduced native effect handlers. Rust simulates them using closures and callbacks. Effects are used in async runtimes (every `await` is an effect), error handling, non-determinism, and resource management.

## Learning Outcomes

- Understand algebraic effects as a mechanism for suspending and resuming computations
- Learn how effects differ from free monads: effects are operational, free monads are denotational
- See a simple simulation using Rust's closures and callback-based resumption
- Understand why algebraic effects require native language support for full efficiency

## Rust Application

Without native effects, Rust simulates them with callback-based inversion of control. An "effect" is a value thrown upward; a "handler" is a function that handles it and optionally resumes the computation via a callback: `fn with_reader<A>(env: E, program: impl FnOnce(&E) -> A) -> A`. More complex effects use `Result`-based suspension: the program returns `Suspend(effect, continuation)` when it performs an effect, and the handler resumes via the continuation.

## OCaml Approach

OCaml 5's native effect handlers:
```ocaml
effect Read : string
let with_reader env program =
  match_with program ()
    { retc = (fun v -> v)
    ; exnc = raise
    ; effc = fun (type a) (eff : a eff) ->
        match eff with
        | Read -> Some (fun (k : (a, _) continuation) ->
            continue k env)
        | _ -> None }
```
The handler intercepts `Read` effects, provides a value, and resumes the computation via `continue k env`. This is purely functional with no heap-allocated trees — the runtime manages the stack frames.

## Key Differences

1. **Native vs. simulated**: OCaml 5 has native effect handlers with efficient stack manipulation; Rust requires explicit continuation-passing simulation.
2. **Stack vs. heap**: OCaml's effect continuations are captured stack frames — efficient; Rust's simulation uses heap-allocated closures.
3. **Multi-shot continuations**: OCaml's `continue k` can call `k` multiple times (non-determinism); Rust's `FnOnce` callbacks can only be called once.
4. **Composability**: OCaml effects compose naturally via nested handlers; Rust simulations require manual composition.

## Exercises

1. Implement a `with_logger` handler that intercepts `Log(message)` effects and stores them in a `Vec<String>`.
2. Simulate non-determinism: an `Amb(choices: Vec<A>)` effect that a handler interprets by trying all choices.
3. Compare the performance of a free monad interpreter vs. the effect simulation for a program with 10,000 operations.
