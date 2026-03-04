# 191: Effect Handlers for Exceptions

**Difficulty:** ⭐⭐⭐⭐⭐  **Level:** Expert

Model exceptions as a special case of algebraic effects — an effect that the handler catches *without resuming*, equivalent to Rust's `Result<T, E>` but demonstrating the connection between exception handling and the effect system.

## The Problem This Solves

Exceptions and algebraic effects are usually taught separately: exceptions are a control flow primitive built into the runtime, while effects are a functional abstraction. But exceptions are actually a restricted form of algebraic effects — specifically, effects whose handler does *not* resume the computation. Understanding this connection deepens your mental model of both.

In OCaml 5, you can model `try/catch` entirely with effects: `effect Exn : string -> 'a` raises an exception-like signal, and a handler catches it with `| effect (Exn msg) _ -> handler msg` — the `_` discards the continuation, making it non-resumable, just like a real exception. This shows that exceptions are not fundamental — they're a pattern within the effect system.

In Rust, we already have `Result<T, E>` which is algebraically equivalent. `Result` is the "exception effect" specialized for Rust's ownership model. This example shows both approaches: idiomatic `Result`-based error handling, and a free-monad simulation of the `Exn` effect, demonstrating that they're the same pattern at different levels of abstraction.

## The Intuition

Normal algebraic effects are like phone calls: the caller performs an effect (dials), the handler picks up and helps, then *resumes* the caller (hangs up and the caller continues their day). Exceptions are like fire alarms: someone raises an exception, the building empties, and there's no "resuming where you left off." The continuation is discarded.

`Result<T, E>` is Rust's fire alarm system. `Err(e)` propagates up through `?` operators, handlers match on it, and there's no going back to the point where the error occurred. The free-monad `Comp<A>` in this example makes that propagation explicit: `Raise` short-circuits every `bind` downstream, bubbling up exactly like an exception, until a `run_comp` handler catches it.

## How It Works in Rust

```rust
// Approach 1: Result<T, E> — idiomatic Rust, equivalent to an Exn effect

fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err("division by zero".to_string()) }
    else { Ok(a / b) }
}

// try_with mirrors OCaml's effect handler: f() runs; if Err, handler catches it
fn try_with<A, F: FnOnce() -> Result<A, String>, H: FnOnce(String) -> A>(f: F, h: H) -> A {
    match f() { Ok(v) => v, Err(msg) => h(msg) }
}

let result = try_with(|| safe_div(10, 0), |msg| { println!("caught: {}", msg); -1 });

// Approach 2: Free-monad simulation — models the Exn effect explicitly

enum Comp<A> {
    Done(A),
    Raise(String),   // the exception — no continuation stored
    Step(Box<dyn FnOnce() -> Comp<A>>),
}

// bind propagates Raise automatically — just like ? operator
fn comp_bind<A: 'static, B: 'static, F: FnOnce(A) -> Comp<B> + 'static>(
    comp: Comp<A>, f: F,
) -> Comp<B> {
    match comp {
        Comp::Done(x) => f(x),
        Comp::Raise(e) => Comp::Raise(e),  // short-circuit: f never called
        Comp::Step(thunk) => Comp::Step(Box::new(move || comp_bind(thunk(), f))),
    }
}

fn run_comp<A, H: FnOnce(String) -> A>(comp: Comp<A>, handler: H) -> A {
    let mut current = comp;
    loop {
        match current {
            Comp::Done(x) => return x,
            Comp::Raise(msg) => return handler(msg),  // handler catches, no resume
            Comp::Step(thunk) => current = thunk(),
        }
    }
}

// Key behavior: Raise in first step skips ALL subsequent steps
let chained = comp_bind(comp_safe_div(100, 0), |x| comp_safe_div(x, 4));
// 100/0 raises → comp_safe_div(x, 4) is NEVER called
let result = run_comp(chained, |msg| { println!("caught: {}", msg); -1 });
// result = -1

// Annotated handler: wraps the error with context, then re-raises
fn annotated_div(a: i32, b: i32) -> Result<i32, String> {
    safe_div(a, b).map_err(|e| format!("annotated_div({}, {}): {}", a, b, e))
}
```

The crucial difference from resumable effects: when `Comp::Raise` matches in `bind`, `f` is discarded. There's no continuation to resume. This is exactly how OCaml's `effect Exn _ -> handler msg` works — the `_` drops the continuation.

## What This Unlocks

- **Understanding `?` operator** — the `?` operator is syntactic sugar for the Exn effect: `Raise` propagates, `Done` continues. Seeing it as an effect handler makes the semantics crystal clear.
- **Custom error propagation strategies** — the `Comp` approach lets you write interpreters that log every error, transform errors as they propagate, or implement retry logic at the effect level.
- **Layered error handling** — annotated handlers add context at each layer (`annotated_div` wraps the inner error), demonstrating how nested effect handlers compose — exactly how `anyhow::Context` works.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Raising an exception | `perform (Exn "msg")` — effect with no useful return type | `Err("msg")` with `?`, or `comp_raise("msg")` in free-monad style |
| Non-resumable handler | `\| effect (Exn msg) _ -> handler msg` — `_` discards continuation | `Comp::Raise(e)` in `bind` discards `f`; `run_comp` calls handler without continuation |
| Automatic propagation | `perform` bubbles through all intermediate frames until a handler | `?` operator on `Result`; `Comp::Raise` propagates through every `comp_bind` |
| Handler re-raise | New `perform (Exn ...)` inside handler — starts new effect | `Err(transformed_msg)` or `comp_raise(new_msg)` in handler |
| Relationship to effects | Exceptions are effects where handler discards `k` | `Result` is the idiomatic equivalent; `Comp<A>` makes the free-monad structure explicit |
