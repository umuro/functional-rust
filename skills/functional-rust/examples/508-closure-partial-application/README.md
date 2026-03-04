# 508: Partial Application with Closures

**Difficulty:** 2  **Level:** Beginner-Intermediate

Fix some arguments of a function, producing a specialized version with fewer parameters.

## The Problem This Solves

You have a general function `clamp(lo, hi, x)` and you need to call it hundreds of times always with `lo=0, hi=100`. Every call site repeats `clamp(0, 100, value)`. If the constants change, you update dozens of places.

Or you're building a pipeline: `items.iter().filter(|x| in_range(0, 100, x))` — the predicate logic is there but the bounds are scattered. What you really want is a pre-configured `in_range_0_100` you can pass around cleanly.

This is the problem partial application solves: fix the "configuration" arguments, get back a function that only takes the "input" arguments. It's a core technique in functional programming that dramatically reduces repetition and makes code read at the right level of abstraction.

## The Intuition

Partial application is like filling out a form ahead of time. `send_email(from, to, subject, body)` — you fill in `from` and `subject` once at the top of your module, and pass around `send_with_context(to, body)` to everyone who needs to send emails.

In Python: `from functools import partial; add5 = partial(add, 5)`. In JavaScript: `const add5 = (x) => add(5, x)` or `add.bind(null, 5)`. Rust uses the same technique as JavaScript: a closure that captures the fixed arguments via `move` and forwards them to the original function.

Unlike OCaml or Haskell where functions are curried by default (applying fewer args than expected automatically produces a partial application), Rust requires you to write the closure explicitly — but it's one line.

## How It Works in Rust

```rust
fn clamp(lo: i32, hi: i32, x: i32) -> i32 { x.max(lo).min(hi) }
fn between(lo: i32, hi: i32, x: i32) -> bool { x >= lo && x <= hi }

// Manual partial application: capture fixed args via move closure
let clamp_0_100 = |x| clamp(0, 100, x);   // lo and hi are fixed
let in_teens    = |x| between(13, 19, x); // lo and hi are fixed

println!("{}", clamp_0_100(150));  // 100
println!("{}", in_teens(15));      // true

// Generic partial helper: fix the first argument of any 2-arg function
fn partial<A: Copy, B, C, F>(f: F, a: A) -> impl Fn(B) -> C
where F: Fn(A, B) -> C {
    move |b| f(a, b)   // a is captured by value (Copy), b comes from caller
}

let starts_with_hello = partial(|prefix: &str, s: &str| s.starts_with(prefix), "hello");
println!("{}", starts_with_hello("hello world")); // true
println!("{}", starts_with_hello("hi there"));    // false

// In a pipeline: partially applied functions slot right into iterators
let add5 = |x: &i32| x + 5;       // fixes the +5 part
let double = |x: i32| x * 2;      // captures nothing; still "partially applies" the *2

let result: Vec<i32> = [1, 2, 3, 4, 5].iter()
    .map(add5)
    .map(double)
    .filter(|&x| between(13, 19, x))  // or .filter(in_teens.clone())
    .collect();
```

## What This Unlocks

- **Cleaner pipelines** — pre-configure predicates and transformers at the top of a function; pass the specialized versions into `map`/`filter`/`sort_by`.
- **Reusable configuration** — share pre-configured functions across a module without passing redundant parameters everywhere.
- **Factory functions** — `make_validator(min, max)` returns a closure; hand validators to form fields, API endpoints, and CLI parsers uniformly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Partial application | `let add5 = (+) 5` — automatic | `let add5 = \|x\| add(5, x)` — explicit closure |
| Curried by default | Yes — `f a b = (f a) b` | No — must wrap in closure |
| Fix first arg | `let f = g arg1` | `move \|rest\| g(arg1, rest)` |
| Generic helper | `let partial f a b = f a b` | `fn partial<A,B,C,F>(f: F, a: A) -> impl Fn(B)->C` |
| Iterator integration | Natural with curried predicates | Closures plug directly into `map`/`filter` |
