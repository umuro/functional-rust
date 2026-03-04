# 065: Church Numerals

**Difficulty:** 3  **Level:** Advanced

Numbers built entirely from functions — no integers, no bits, just closures.

## The Problem This Solves

You've been writing Rust for a while. Numbers are `u64`, booleans are `bool`, and that's just... the way things are. But have you ever asked: *what actually IS a number?* Not "what does it represent" — but what IS it, at the most fundamental level?

This sounds philosophical, but it has real engineering consequences. Every programming language runtime needs to answer this question. The Church numeral answers it in the most minimal way possible: a number is a *behaviour*. Specifically, the number N is the behaviour of "apply some function N times."

This is the foundation of lambda calculus — the mathematical model that ALL functional programming languages (and Rust's type system) are built on. Understanding it makes you a better Rust developer by revealing what closures really are and what the ownership system is actually protecting.

This example exists to show you the deepest roots of what functions can express — and to reveal why Rust's ownership model creates interesting friction at those roots.

## The Intuition

Think of a number as an action, not a value.

- **Zero**: "Do nothing." Apply `f` to `x` zero times → return `x` unchanged.
- **One**: "Do it once." Apply `f` to `x` once → return `f(x)`.
- **Two**: "Do it twice." Apply `f` to `x` twice → return `f(f(x))`.
- **Three**: "Do it three times." → `f(f(f(x)))`.

To "read" the number, you plug in `f = |x| x + 1` and `x = 0`. The number three applied to those arguments gives you `3`. But the number ISN'T the integer 3 — it IS the function that applies its argument three times.

```rust
// "Three" in Church encoding — it IS a function
let three = |f: fn(i64) -> i64, x: i64| f(f(f(x)));

// To extract the integer: apply "add 1" starting from 0
let result = three(|x| x + 1, 0);
// f applied 3 times: 0 → 1 → 2 → 3
assert_eq!(result, 3);
```

Addition then falls out naturally: `m + n` means "apply f m times, then n more times."

## How It Works in Rust

The core challenge: in lambda calculus, functions are perfectly flexible — you can pass any function anywhere. In Rust, every closure has a unique unnameable type, so we need `Box<dyn Fn>` to erase those types.

```rust
// A Church numeral: takes a function, returns a function
// (apply-me-n-times-to-whatever-you-give-me)
type Church = Box<dyn Fn(Box<dyn Fn(i64) -> i64>) -> Box<dyn Fn(i64) -> i64>>;

// Zero: apply f zero times = just return x
fn zero() -> Church {
    Box::new(|_f| Box::new(|x| x))
}

// One: apply f exactly once
fn one() -> Church {
    Box::new(|f| Box::new(move |x| f(x)))
}

// Successor: take n, return n+1 (apply f one extra time)
fn succ(n: Church) -> Church {
    Box::new(move |f: Box<dyn Fn(i64) -> i64>| {
        use std::rc::Rc;
        // Problem: we need to give f to BOTH n and the extra application
        // Rust won't let us move f twice — so we use Rc for shared ownership
        let f = Rc::new(f);
        let f2 = f.clone();
        let inner = n(Box::new(move |x| f2(x)));  // n's applications
        let f3 = f.clone();
        Box::new(move |x| f3(inner(x)))            // one more application
    })
}

// To read the number: apply "add 1" to 0
fn to_int(n: Church) -> i64 {
    n(Box::new(|x| x + 1))(0)
}
```

**Why `Rc` appears**: In OCaml, closures automatically share the captured value. In Rust, the borrow checker requires explicit shared ownership. This `Rc` cost is exactly why idiomatic Rust uses a struct instead:

```rust
// Practical version: same semantics, Rust-friendly
struct ChurchNum(usize);

impl ChurchNum {
    fn apply<T>(&self, f: impl Fn(T) -> T, x: T) -> T {
        (0..self.0).fold(x, |acc, _| f(acc))
    }
    fn to_int(&self) -> usize { self.apply(|x: usize| x + 1, 0) }
    fn add(self, other: Self) -> Self { ChurchNum(self.0 + other.0) }
    fn mul(self, other: Self) -> Self { ChurchNum(self.0 * other.0) }
}
```

## What This Unlocks

- **Lambda calculus literacy**: You can now read papers and languages that talk about Church encoding — this is used in Haskell, Coq, proof assistants, and compiler theory.
- **Understanding trait objects**: The `Box<dyn Fn>` gymnastics here reveal exactly WHY Rust needs trait objects and what heap allocation costs at the closure level.
- **Deriving data from functions**: Church encoding proves you can build lists, booleans, pairs — all data — from just functions. Used in minimalist language interpreters and formal verification.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Church numeral type | `('a -> 'a) -> 'a -> 'a` (polymorphic) | `Box<dyn Fn(Box<dyn Fn(i64)->i64>)->Box<dyn Fn(i64)->i64>>` |
| Sharing a closure | Automatic (GC) | Requires `Rc` or explicit cloning |
| Successor function | One line, elegant | Needs `Rc` for shared `f` |
| Practical use | Possible as-is | Wrap in a struct instead |
| Each closure allocation | Stack (usually) | Heap (`Box`) |
