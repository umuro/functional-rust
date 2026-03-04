# 052: Functor Laws

**Difficulty:** ⭐⭐ Intermediate  **Level:** Intermediate

Why `map` must follow two specific rules — and what breaks when it doesn't.

## The Problem This Solves

Suppose someone writes a "custom map" for their type. It looks right: it takes a function and returns a transformed container. You start using it in your code. Then things get weird.

```rust
let x = MyBox::new(42);
let y = x.map(|v| v);  // identity — should be a no-op
assert_eq!(x, y);      // FAILS. Why?!
```

Or even stranger: you chain two maps, then try to replace them with a single composed map (a common optimization), and the results differ. Your code is full of "equivalent" expressions that aren't actually equivalent. Debugging becomes a nightmare because you can't trust what `map` does.

This isn't hypothetical. Without laws, every type's `map` is a snowflake with hidden behavior. Code that looks like a refactoring is actually a behavior change. Generic algorithms that assume `map` is predictable silently produce wrong results.

The Functor laws are two simple rules that tell you exactly what `map` is allowed to do — and what it isn't. When every Functor follows these rules, you can reason about code, refactor safely, and trust that `map` never has secret side effects. This concept exists to solve exactly that pain.

## The Intuition

Think of `map` like a vending machine that takes your snack request (a function) and applies it to what's inside without touching the machine's mechanics.

**Law 1 — Identity:** If you ask the machine to "give me exactly what's already there" (`|x| x`), you should get the exact same thing back. The machine should not have secretly counted your request, logged it, or changed the snack in any way.

**Law 2 — Composition:** If you first ask for "salted" then "crushed" (two separate passes), you should get the same result as if you had said "salted and crushed" in one request. Two maps in sequence should always equal one map with the composed function.

```
map(id) == identity              ← Law 1: doing nothing does nothing
map(f).map(g) == map(g∘f)        ← Law 2: two passes == one composed pass
```

These laws mean `map` is **structure-preserving**: it can change the values inside, but it cannot add elements, remove elements, reorder them, or count how many times it was called.

## How It Works in Rust

**Law 1 — Identity:**

```rust
fn verify_identity_law(x: Maybe<i32>) -> bool {
    x.clone().map(|v| v) == x  // map with identity function = original
}

assert!(verify_identity_law(Maybe::Just(42)));  // true for any value
assert!(verify_identity_law(Maybe::Nothing));   // true for empty too
```

**Law 2 — Composition:**

```rust
fn verify_composition_law(f: fn(i32)->i32, g: fn(i32)->i32, x: Maybe<i32>) -> bool {
    let composed = x.clone().map(|v| f(g(v)));  // one map, composed function
    let chained  = x.map(g).map(f);             // two maps in sequence
    composed == chained
}
```

If both sides are equal for all functions `f`, `g` and all values `x`, your Functor is law-abiding.

**A law-breaking Functor — the bad example:**

```rust
struct BadFunctor<T> {
    value: T,
    map_count: usize,  // tracks how many times map was called
}

impl<T> BadFunctor<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> BadFunctor<U> {
        BadFunctor {
            value: f(self.value),
            map_count: self.map_count + 1,  // ← breaks Law 1!
        }
    }
}
```

Why does this break Law 1? Because `bad.map(|x| x)` is *not* the same as `bad` — the `map_count` changed from 0 to 1. The identity law is violated. It also breaks Law 2: `bad.map(g).map(f)` has `map_count = 2`, while `bad.map(|x| f(g(x)))` has `map_count = 1`. Two "equivalent" expressions give different results.

**Testing laws with standard Rust types:**

```rust
// Vec follows both laws — you can verify:
let xs = vec![1, 2, 3];
let after_id: Vec<i32> = xs.iter().copied().map(|x| x).collect();
assert_eq!(after_id, xs);  // Law 1: holds

let f = |x: i32| x * 2;
let g = |x: i32| x + 3;
let composed: Vec<i32> = xs.iter().copied().map(|x| f(g(x))).collect();
let chained:  Vec<i32> = xs.iter().copied().map(g).map(f).collect();
assert_eq!(composed, chained);  // Law 2: holds
```

`Option`, `Vec`, `Result` in Rust's standard library all satisfy both laws. That's why you can chain `.map()` calls freely and trust they compose.

## What This Unlocks

- **Safe refactoring.** You can replace `x.map(f).map(g)` with `x.map(|v| g(f(v)))` with confidence — they're provably equal for any law-abiding Functor.
- **Trustworthy generic code.** Functions that accept any Functor can assume predictable behavior. Property-based testing tools (like `proptest`) can automatically verify these laws for your types.
- **Catching subtle bugs early.** A custom container with a broken `map` is a time bomb. Testing identity and composition laws catches it immediately, before it causes mysterious failures downstream.

Real codebases where Functor laws matter: `serde` (serialization must be structure-preserving), `rayon` (parallel map must compose like sequential map), iterator adapters in `std` (the entire adapter chain relies on composition law).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Equality check | Structural `=` by default — works on any type | Requires `#[derive(PartialEq)]` explicitly |
| Cloning for law tests | Not needed — immutable values can be shared/reused | Must call `.clone()` before testing to preserve the original |
| Law enforcement | Not enforced by compiler — convention and tests only | Same — no compile-time enforcement; use `#[test]` to verify |
| Identity function | `Fun.id` from stdlib, or `fun x -> x` | Written inline as `\|x\| x` (no stdlib `id` function) |
| Composition | `let compose f g x = f (g x)` — explicit helper | Closure composition: `\|x\| f(g(x))` inline |
| Bad functor detection | Runtime assertion: `assert (mapped <> original)` | `assert_ne!(mapped, original)` in `#[test]` |
