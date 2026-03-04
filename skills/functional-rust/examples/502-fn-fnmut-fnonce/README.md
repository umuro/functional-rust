# 502: Fn, FnMut, FnOnce Hierarchy

**Difficulty:** 3  **Level:** Intermediate

The three closure traits tell you *how many times* a closure can be called and *what it does* to its captures.

## The Problem This Solves

You write a function that accepts a closure and hit: `the trait bound FnMut is not satisfied`, or you try to call a closure twice and get `use of moved value`. The root cause is Rust's three-trait hierarchy for closures — and not understanding it means writing overly restrictive or overly permissive function signatures.

If you always write `F: Fn(...)`, you reject valid closures that mutate state (counters, accumulators). If you write `F: FnOnce(...)`, you allow callers to pass a one-shot closure into a loop. Getting the bounds right matters for correctness, usability, and compiler error quality.

Without this understanding, you'll either make every function too strict (rejecting useful closures) or introduce subtle bugs by calling a closure that should only run once multiple times.

## The Intuition

Think of the three traits as permission levels:

- **`FnOnce`** — "call me once." The closure consumes (moves out of) at least one captured value when called. Like a JavaScript `.then()` callback that uses a `Promise` — once the value is moved, it's gone.
- **`FnMut`** — "call me multiple times, I'll update my internal state." The closure mutates its captures. Like a Python generator's `__next__` — each call advances state.
- **`Fn`** — "call me as many times as you want, nothing changes." Pure read-only access to captures. Like a Python lambda or JavaScript arrow function that only reads its closure variables.

The hierarchy: **`Fn ⊆ FnMut ⊆ FnOnce`**. Every `Fn` is also `FnMut` and `FnOnce`. But not every `FnOnce` is `FnMut`.

## How It Works in Rust

```rust
// FnOnce: consumes a captured value — can only be called once
fn make_greeting(name: String) -> impl FnOnce() -> String {
    move || format!("Hello, {}!", name)  // name is moved OUT on call
}
let greet = make_greeting("Alice".to_string());
println!("{}", greet());  // ✓ first call
// println!("{}", greet()); // ✗ ERROR: value used after move

// FnMut: mutates captures — can be called many times
fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || { count += 1; count }   // mutates captured count
}
let mut counter = make_counter();
println!("{}", counter()); // 1
println!("{}", counter()); // 2

// Fn: read-only captures — freely shareable
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor    // only reads factor
}
let double = make_multiplier(2);
println!("{}", double(5)); // 10
println!("{}", double(5)); // 10 — still works, nothing consumed

// Function signature best practice: use the least restrictive bound
fn call_once<F: FnOnce() -> String>(f: F) -> String { f() }
fn call_many<F: FnMut() -> i32>(mut f: F) -> Vec<i32> { vec![f(), f(), f()] }
fn apply_pure<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(f(x)) }  // calls twice
```

## What This Unlocks

- **Generic higher-order functions** — write the right bound (`Fn`/`FnMut`/`FnOnce`) to accept the widest useful set of closures.
- **One-shot callbacks** — `FnOnce` models tasks, futures, and destructuring callbacks that must only fire once.
- **Stateful iterators** — `FnMut` is the trait behind `Iterator::map`, enabling counters and accumulators inside iterator chains.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Single-call function | `unit -> 'a` (no enforcement) | `FnOnce` — compiler enforced |
| Stateful closure | `unit -> 'a` with `ref` inside | `FnMut` — requires `&mut self` on call |
| Pure closure | `'a -> 'b` (no restriction) | `Fn` — only reads captures |
| Hierarchy | No subtyping between function types | `Fn ⊆ FnMut ⊆ FnOnce` |
| Type constraint | Structural typing | Explicit trait bounds |
