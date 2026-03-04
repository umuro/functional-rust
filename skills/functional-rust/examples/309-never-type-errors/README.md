# 309: The Never Type (!)

**Difficulty:** 4  **Level:** Expert

`!` is the bottom type — a computation that never produces a value — and it fits anywhere in the type system.

## The Problem This Solves

Every expression in Rust has a type. But some expressions never finish: they panic, loop forever, or call `std::process::exit()`. What type should those have? If `panic!()` had type `()`, it couldn't appear in match arms that expect `i32`. If it had type `i32`, it couldn't appear where a `String` is needed.

The answer is `!` — the never type, the bottom of the type lattice. A value of type `!` can never exist, which means it can pretend to be any type. This isn't a hack; it's mathematically sound. From a false premise, you can prove anything (ex falso quodlibet). If `!` had a value, you could use it as an `i32`, a `String`, or anything else — but since it never does, the coercion is always vacuously safe.

This matters practically in error handling: `Infallible` (a zero-variant enum, equivalent to `!`) is the error type for conversions that literally cannot fail, and `Result<T, !>` can only ever be `Ok(T)`.

## The Intuition

Think of `!` as a mathematical guarantee: "this code path is unreachable." When you write a function returning `!`, you're promising the compiler that the function never returns normally. The compiler trusts this and allows `!` expressions to unify with any other type — like a wildcard in pattern matching.

`Infallible` is `!` made stable and nameable. It's an enum with zero variants — you cannot construct a value of it, ever. So `Result<T, Infallible>` is effectively `T` wrapped in a `Result`, and you can `.unwrap()` it knowing it will never panic.

## How It Works in Rust

```rust
// Functions returning ! never return normally
fn crash(msg: &str) -> ! {
    panic!("{}", msg)
}

// ! coerces to any type — valid in match arms
fn parse_or_die(s: &str) -> i32 {
    s.parse().unwrap_or_else(|_| crash("parse failed"))
    //                              ^^^^^^^^^^^^^^^^^ : ! coerces to i32
}

// Infallible: can never be constructed
use std::convert::Infallible;
let r: Result<i32, Infallible> = Ok(42);
let val = r.unwrap(); // safe — Err variant is impossible

// Result<T, Infallible> can match with only the Ok arm
let val = match r {
    Ok(v) => v,
    // Err arm omitted — compiler knows Infallible has no values
};

// From<!> for T is implemented for all T
// From<Infallible> for T is implemented via the same logic
```

## What This Unlocks

- **Type-safe divergence** — express "this branch cannot be reached" at the type level, not just in comments
- **API design** — `Result<T, Infallible>` lets you satisfy a fallible API contract while documenting that your implementation is actually infallible
- **Exhaustive match simplification** — matching on `Infallible` requires zero arms; the compiler knows it's impossible

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Never type | `'a` (universal polymorphism) | `!` (explicit bottom type) |
| Diverging coercion | Implicit (any diverging expr) | `!` coerces to any type |
| Infallible error | Phantom type / `exn` workaround | `std::convert::Infallible` |
| Match on empty type | Impossible pattern | `match inf {}` (zero arms) |
| Process exit | `exit` returns `unit` | `std::process::exit() -> !` |
