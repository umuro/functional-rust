# 120: Fn, FnMut, FnOnce

**Difficulty:** 3  **Level:** Intermediate

Rust's three closure traits encode how a closure uses its captured variables — the compiler picks the right one automatically.

## The Problem This Solves

Every closure captures some variables from its environment. The key question is: what does it *do* with them? Does it just read them? Mutate them? Consume them outright? The answer determines what the closure is allowed to do — and how safely you can call it multiple times.

OCaml has one closure type and no way to express these distinctions in the type system. Rust encodes them as three traits: `Fn`, `FnMut`, and `FnOnce`. A closure that only reads its captures implements all three — it's the most flexible. A closure that mutates its captures implements only `FnMut` and `FnOnce`. A closure that moves a value out of its captures implements only `FnOnce` — you can call it exactly once, because after that call, the captured value is gone.

This matters when writing higher-order functions. If you write `fn apply(f: impl Fn())`, callers know you'll invoke `f` multiple times and it's safe. If you write `fn apply(f: impl FnOnce())`, you're telling callers: "I'll call this exactly once, and it can destroy its captures when it does." The type system makes the contract explicit.

## The Intuition

`FnOnce` = can call once (moves captured values). `FnMut` = can call multiple times with mutation. `Fn` = can call multiple times, read-only. Every closure is at least `FnOnce`; the most permissive (and common) is `Fn`.

## How It Works in Rust

```rust
// Fn — captures by shared reference, callable any number of times
fn make_greeter(prefix: String) -> impl Fn(&str) -> String {
    move |name| format!("{}, {}!", prefix, name)
    // `prefix` is moved in, but the closure only reads it → Fn
}
let greet = make_greeter("Hello".into());
greet("Alice");  // works
greet("Bob");    // works again — Fn means unlimited calls

// FnMut — mutates a captured value, callable multiple times
fn make_counter() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || { count += 1; count }
    // `count` is mutated each call → FnMut (not Fn)
}
let mut next = make_counter();
assert_eq!(next(), 1);
assert_eq!(next(), 2);

// FnOnce — moves a value out of the capture, callable exactly once
fn make_farewell(name: String) -> impl FnOnce() -> String {
    move || format!("Goodbye, {}!", name)
    // `name` is moved into the return value → FnOnce
}
let farewell = make_farewell("World".into());
let msg = farewell();  // name is consumed here
// farewell();         // ERROR: use of moved value

// Every Fn is also FnMut and FnOnce — the hierarchy is Fn ⊂ FnMut ⊂ FnOnce
fn takes_fnonce(f: impl FnOnce() -> i32) -> i32 { f() }
let f = || 42;         // implements Fn
takes_fnonce(f);       // works — Fn satisfies FnOnce too
```

## What This Unlocks

- **Correct API contracts** — `impl Fn` in a parameter tells callers you'll invoke the closure multiple times; `impl FnOnce` tells them it's single-use.
- **Stateful iterators and generators** — `FnMut` closures are the building block for counters, accumulators, and lazy state machines.
- **One-shot callbacks** — `FnOnce` callbacks can consume resources (close a socket, send a message) and the type system ensures they're not called twice.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Closure traits | One closure type | Three: `Fn`, `FnMut`, `FnOnce` |
| Mutating captured state | Via `ref` cell inside the closure | `FnMut` — mutable borrow of captured binding |
| Consuming a captured value | Not expressible | `FnOnce` — moves out of captured binding |
| Callability guarantees | None in the type | `Fn` = unlimited, `FnOnce` = exactly once |
