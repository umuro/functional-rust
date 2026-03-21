**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[currying-partial on hightechmind.io](https://hightechmind.io/posts/functional-rust/currying-partial)

---

## Problem Statement

Explore currying, partial application, and function sections in Rust. Unlike OCaml, where all functions are curried by default, Rust functions take all arguments at once. Partial application is achieved through closures that capture some arguments. Implement a `curry` converter, an `uncurry` converter, and a `pipeline` function that folds a value through a chain of unary functions.

## Learning Outcomes

- Understand that Rust functions are NOT curried by default; closures provide partial application
- Implement `add_curried(x) -> impl Fn(i32) -> i32` to mimic OCaml's default currying
- Write a generic `curry<A, B, C>` converter that turns `fn(A, B) -> C` into `Fn(A) -> Box<dyn Fn(B) -> C>`
- Write the inverse `uncurry` converter
- Implement `pipeline(init, &[&dyn Fn(i32) -> i32]) -> i32` as a fold over unary functions

## Rust Application

```rust
pub fn add(x: i32, y: i32) -> i32 { x + y }

// Partial application via closure capture
pub fn add5() -> impl Fn(i32) -> i32 {
    move |y| add(5, y)
}

// Curried function — returns a closure
pub fn add_curried(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// curry converter: (A, B) -> C  becomes  A -> (B -> C)
pub fn curry<A, B, C, F>(f: F) -> impl Fn(A) -> Box<dyn Fn(B) -> C>
where
    A: Copy + 'static,
    B: 'static,
    C: 'static,
    F: Fn(A, B) -> C + Clone + 'static,
{
    move |a: A| {
        let f = f.clone();
        Box::new(move |b: B| f(a, b))
    }
}

// uncurry converter: (A -> B -> C)  becomes  (A, B) -> C
pub fn uncurry<A, B, C>(f: impl Fn(A) -> Box<dyn Fn(B) -> C>) -> impl Fn(A, B) -> C {
    move |a, b| f(a)(b)
}

// pipeline: fold initial value through a chain of unary functions
pub fn pipeline(initial: i32, funcs: &[&dyn Fn(i32) -> i32]) -> i32 {
    funcs.iter().fold(initial, |acc, f| f(acc))
}
```

`add_curried(5)` returns a closure that adds 5 to any `i32` — the same as `add 5` in OCaml. The `move` keyword captures `x` by value into the returned closure.

The `curry` converter requires `A: Copy + 'static` because the closure captures `a` by value and the inner `Box<dyn Fn(B) -> C>` must be `'static`. This is a lifetime/ownership price that OCaml's GC avoids.

`pipeline` demonstrates that a list of unary functions is equivalent to function composition. `pipeline(x, [f, g, h])` computes `h(g(f(x)))`.

## OCaml Approach

```ocaml
(* OCaml functions are curried by default *)
let add x y = x + y
let add5 = add 5          (* partial application — no closure needed *)
let add_curried x y = x + y  (* identical to add *)

let pipeline initial funcs =
  List.fold_left (fun acc f -> f acc) initial funcs

(* curry/uncurry as identity since OCaml is already curried *)
let curry f x y = f (x, y)
let uncurry f (x, y) = f x y
```

In OCaml, `add 5` is free — no closure allocation; the runtime tracks the partial application. `curry` and `uncurry` in OCaml convert between tuple-argument and curried-argument styles, which is the reverse of Rust's use case.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Default currying | No — use closures | Yes — all multi-arg functions are auto-curried |
| Partial application | `move |y| f(5, y)` | `f 5` — no closure syntax needed |
| `Box<dyn Fn>` overhead | Required for generic curried return type | No equivalent overhead (closures are GC values) |
| Pipeline | `fold` over `&[&dyn Fn]` | `List.fold_left (fun acc f -> f acc)` |
| Operator sections | `(|x| x * 2)` | `(( * ) 2)` — more concise |

Rust's type system makes generic higher-order combinators like `curry` awkward — the `A: Copy + 'static` constraints are artifacts of ownership, not logic. For practical code, prefer direct closures over a generic curry combinator.

## Exercises

1. Implement `compose(f, g) -> impl Fn(A) -> C` where `compose(f, g)(x) = g(f(x))` (left-to-right composition).
2. Implement `compose_n(funcs: Vec<Box<dyn Fn(i32) -> i32>>) -> impl Fn(i32) -> i32` that composes a dynamic list of functions.
3. Use `add_curried` to create `add1`, `add10`, `add100` and apply them via `pipeline`.
4. Implement a `memoize` higher-order function that wraps `Fn(i32) -> i32` with a `HashMap` cache.
5. Explore Rust's `FnOnce`, `FnMut`, `Fn` distinction: write examples that compile only with `FnOnce` and only with `Fn`.
