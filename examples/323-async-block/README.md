📖 **[View on hightechmind.io →](https://hightechmind.io/rust/323-async-block)**

---

# 323: async blocks and Lazy Evaluation
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

`async fn` creates a function that returns a `Future`. `async { }` blocks create anonymous futures inline. The key property of both is laziness: the code inside does not execute until the future is `.await`ed or driven by an executor. This is fundamentally different from eager evaluation — a value computed immediately. Understanding this laziness is essential for avoiding bugs where code runs at the wrong time or doesn't run at all.

## Learning Outcomes

- Understand that `async { }` blocks create futures that are lazy — code doesn't run until `.await`ed
- Recognize the analogy between closures (lazy functions) and async blocks (lazy computations)
- Use `async move` to capture values by ownership into an async block
- Understand why forgetting to `.await` a future causes silent bugs (the computation never runs)

## Rust Application

The example uses synchronous closures as an analogy for async blocks' lazy evaluation:

```rust
// Lazy computation: created but not executed
pub fn lazy_comp<'a, F, T>(label: &'a str, f: F) -> impl FnOnce() -> T + 'a {
    println!("Creating: {}", label);  // runs at creation
    move || {
        println!("Executing: {}", label);  // runs only when called
        f()
    }
}

// Async equivalent: async { work } creates a future but doesn't run work
// async move { owned_var } captures owned_var by value
```

In real async code: `let fut = fetch_data();` creates the future; `fut.await` runs it.

## OCaml Approach

OCaml's `Lwt` promises are also lazy in a sense — a `Lwt.t` value represents a pending computation, and only resolving it (via `Lwt.bind` or `>>=`) triggers continuation:

```ocaml
(* Thunk: lazy value in OCaml *)
let lazy_comp label f =
  Printf.printf "Creating: %s\n" label;
  fun () ->
    Printf.printf "Executing: %s\n" label;
    f ()
```

## Key Differences

1. **Explicit laziness**: Rust async blocks are explicitly lazy — nothing runs until `.await`; OCaml's `Lwt.t` values trigger computation when bound.
2. **Move semantics**: `async move { }` captures environment by value; regular `async { }` may borrow — ownership rules apply to async blocks.
3. **Forgotten futures**: In Rust, not `.await`-ing a future silently discards it; in OCaml, not binding a `Lwt.t` has similar silent effects.
4. **Type**: An `async { }` block has type `impl Future<Output = T>` where `T` is the last expression's type.

## Exercises

1. Create two async blocks (simulated as closures) and demonstrate that their creation and execution are separate events.
2. Implement `run_if(cond: bool, thunk: impl FnOnce() -> T) -> Option<T>` and show its analogy to `if cond { Some(fut.await) } else { None }`.
3. Show with a test that a `lazy_comp` closure that captures a `String` by move can only be called once (FnOnce semantics).
