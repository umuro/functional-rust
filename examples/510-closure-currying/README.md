📖 **[View on hightechmind.io →](https://hightechmind.io/rust/510-closure-currying)**

---

# 510: Currying Pattern in Rust

**Difficulty:** 3  **Level:** Intermediate

Transform multi-argument functions into chains of single-argument functions: `add(3, 4)` becomes `add(3)(4)`.

## The Problem This Solves

You want to use `map` with a two-argument function, but `map` only passes one argument per call. In OCaml or Haskell, this is trivial — all functions are curried by default, so `map (add 5) [1;2;3]` just works. In Rust, you reach for a closure: `map(|x| add(5, x), ...)`.

The deeper problem appears when you need to compose or partially apply many multi-argument functions. The manual closure wrapping accumulates, making code verbose. Currying solves this structurally: a curried function *is* its own partial application mechanism.

Understanding currying also demystifies function type signatures in type theory, makes closures-as-return-values feel natural, and helps you recognize patterns in Rust APIs that implicitly curry (like `sort_by_key`).

## The Intuition

Currying is named after Haskell Curry (the logician, not the food). The idea: instead of `f(a, b) = result`, you have `f(a) = g` where `g(b) = result`. You transform a function of N arguments into N nested single-argument functions.

In OCaml, `let add a b = a + b` is automatically `add : int -> int -> int` — a function that takes an `int` and returns a function `int -> int`. You get partial application for free: `let add5 = add 5`.

In Rust, you make this explicit with nested closures. Each closure captures the previous arguments via `move` and returns another closure. It's more verbose than OCaml but the mechanics are identical.

## How It Works in Rust

```rust
// Curried add: add(x)(y) instead of add(x, y)
fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y    // x captured by move
}

let add5 = add(5);      // add5: impl Fn(i32) -> i32
println!("{}", add5(3));  // 8
println!("{}", add(3)(4)); // 7 — chained call

// Three-argument curried function: clamp(lo)(hi)(x)
// impl Fn -> impl Fn doesn't compile, so use Box<dyn Fn>
fn clamp(lo: i32) -> Box<dyn Fn(i32) -> Box<dyn Fn(i32) -> i32>> {
    Box::new(move |hi| Box::new(move |x| x.max(lo).min(hi)))
}

let clamp_0_100 = clamp(0)(100);  // partially applied — lo and hi fixed
println!("{}", clamp_0_100(150));  // 100 — clamped
println!("{}", clamp_0_100(42));   //  42 — unchanged

// Convert uncurried to curried generically
fn curry<A: Copy + 'static, B: Copy + 'static, C: 'static, F>(
    f: F
) -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> C>>
where F: Fn(A, B) -> C + Copy + 'static {
    Box::new(move |a| Box::new(move |b| f(a, b)))
}

let curried_mul = curry(|x: i32, y: i32| x * y);
let times6 = curried_mul(6);
println!("{}", times6(7));  // 42

// Convert curried back to uncurried
fn uncurry<A, B, C, F, G>(f: F) -> impl Fn(A, B) -> C
where F: Fn(A) -> G, G: Fn(B) -> C {
    move |a, b| f(a)(b)
}
let plain_add = uncurry(add);
println!("{}", plain_add(3, 4)); // 7
```

## What This Unlocks

- **Partial application for free** — a curried function *is* a partial application factory; `add(5)` returns a reusable `+5` function.
- **Point-free style** — use `map(add(10), items)` instead of `map(|x| x + 10, items)` when functions are curried.
- **Type theory foundations** — understanding currying demystifies Rust's `Fn` type signatures, `impl Fn(A) -> impl Fn(B) -> C`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Curried by default | Yes — every function | No — must explicitly nest closures |
| Partial application | `let add5 = add 5` — automatic | `let add5 = add(5)` — only if curried |
| Type signature | `int -> int -> int` | `impl Fn(i32) -> impl Fn(i32) -> i32` |
| Multi-level return | Natural with type inference | Needs `Box<dyn Fn>` for 3+ levels |
| `curry` helper | `let curry f a b = f (a, b)` | Generic function or macro |
