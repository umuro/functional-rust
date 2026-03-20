📖 **[View on hightechmind.io →](https://hightechmind.io/rust/508-closure-partial-application)**

---

# Closure Partial Application
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


Partial application fixes some arguments of a multi-argument function, producing a specialised single-argument function — implemented in Rust with closures that capture the fixed arguments.

## Problem Statement

A function `add(x, y)` needs to be passed to `map` which expects `Fn(i32) -> i32`. Partial application solves this: `partial(add, 5)` returns `|y| add(5, y)` — the `5` is baked in. This pattern appears everywhere: creating specialised predicates (`between(0, 100, x)`), building pipeline stages with fixed configuration, and adapting multi-argument functions to single-argument interfaces. Haskell and OCaml have partial application built into the language via currying; Rust requires explicit closures.

## Learning Outcomes

- Implement generic `partial` that fixes the first argument of a 2-arg function
- Implement `partial2` that fixes the first two arguments of a 3-arg function
- Understand `A: Copy` bound — the fixed argument is copied into each returned closure
- Write manual partial application with `create_adder(n)` and `create_range_checker(lo, hi)`
- Recognise the connection between partial application and currying

## Rust Application

Generic `partial` fixes the first argument of any 2-arg function:

```rust
pub fn partial<A: Copy, B, C, F>(f: F, a: A) -> impl Fn(B) -> C
where F: Fn(A, B) -> C {
    move |b| f(a, b)
}
```

Usage:

```rust
let add5 = partial(|x, y| x + y, 5);
assert_eq!(add5(10), 15);

let clamp_to_100 = partial2(clamp, 0, 100);
assert_eq!(clamp_to_100(150), 100);
```

Manual factory functions are often clearer than generic `partial`:

```rust
pub fn create_range_checker(lo: i32, hi: i32) -> impl Fn(i32) -> bool {
    move |x| between(lo, hi, x)
}
```

## OCaml Approach

OCaml functions are curried by default — partial application is syntactic:

```ocaml
let add x y = x + y
let add5 = add 5         (* partial application — no extra syntax *)
let () = assert (add5 10 = 15)

let clamp lo hi x = max lo (min hi x)
let clamp_to_100 = clamp 0 100  (* fix first two args *)
```

This is a fundamental difference: OCaml's multi-argument functions are syntactic sugar for nested single-argument functions; Rust's multi-argument functions are genuinely multi-argument.

## Key Differences

1. **Implicit vs. explicit**: OCaml has built-in partial application via currying — `add 5` is legal. Rust requires an explicit closure or `partial` helper.
2. **`Copy` constraint**: Rust's `partial` requires `A: Copy` so the fixed argument is copied into each returned closure; OCaml copies `int`/`float` primitives implicitly.
3. **`partial2` generality**: Rust's `partial2` must be defined separately from `partial`; OCaml's currying handles any number of fixed arguments uniformly.
4. **Specialisation at compile time**: Rust's `partial` is monomorphised — the returned closure is a distinct type per call; OCaml uses uniform representation for all closures.

## Exercises

1. **`partial_right`**: Write `fn partial_right<A, B: Copy, C, F>(f: F, b: B) -> impl Fn(A) -> C where F: Fn(A, B) -> C` that fixes the *second* argument.
2. **URL builder**: Use `partial` or manual closures to build `get_users`, `get_orders` from a generic `make_request(method, endpoint)` base function.
3. **Pipeline with partial**: Build a numeric pipeline `[partial(clamp, 0), partial(|x,n| x*n, 2), |x| x as f64 / 100.0]` using `chain_closures` from example 505 and verify the output.
