📖 **[View on hightechmind.io →](https://hightechmind.io/rust/510-closure-currying)**

---

# Closure Currying

Currying transforms an N-argument function into a chain of N single-argument functions — `add(x, y)` becomes `add(x)(y)`. Rust implements currying explicitly through nested closures returning closures, with `curry` and `uncurry` as conversion utilities.

## Problem Statement

Currying (named after Haskell Curry, formalised by Schönfinkel) is the theoretical foundation of lambda calculus — every multi-argument function can be expressed as nested single-argument functions. Practical benefits: unified partial application syntax (just call with fewer arguments), function composition works naturally on single-argument functions, and point-free style becomes possible. OCaml and Haskell have currying built in; Rust requires explicit nested closures, but the pattern is expressible and useful.

## Learning Outcomes

- Write curried functions returning `impl Fn(i32) -> i32` from `fn add(x: i32) -> impl Fn(i32) -> i32`
- Implement three-argument currying with nested `Box<dyn Fn>`
- Write a generic `curry<F>` that converts an uncurried 2-arg function to curried form
- Write `uncurry` that converts curried form back to a 2-arg function
- Implement `flip` that reverses the argument order of a curried function

## Rust Application

Two-argument curried functions:

```rust
pub fn add(x: i32) -> impl Fn(i32) -> i32 { move |y| x + y }
pub fn mul(x: i32) -> impl Fn(i32) -> i32 { move |y| x * y }

add(5)(3)  // 8 — partial: add(5) returns Fn(i32)->i32
```

Generic curry/uncurry conversions:

```rust
pub fn curry<A: Copy, B: Copy, C, F>(f: F)
    -> Box<dyn Fn(A) -> Box<dyn Fn(B) -> C>>
where F: Fn(A, B) -> C + Copy { ... }

pub fn uncurry<A, B, C, F, G>(f: F) -> impl Fn(A, B) -> C
where F: Fn(A) -> G, G: Fn(B) -> C {
    move |a, b| f(a)(b)
}
```

## OCaml Approach

OCaml functions are curried by default:

```ocaml
let add x y = x + y   (* add : int -> int -> int — already curried *)
let add5 = add 5       (* partial application — no extra syntax *)
let () = assert (add5 3 = 8)

(* Three-arg currying *)
let clamp lo hi x = max lo (min hi x)
let clamp_0_100 = clamp 0 100

(* Flip *)
let flip f x y = f y x
let sub_from_10 = flip (-) 10   (* fun x -> 10 - x *)
```

OCaml's default currying makes all of these patterns natural without any boilerplate.

## Key Differences

1. **Implicit currying**: OCaml's `add x y` is syntactic sugar for `fun x -> (fun y -> x + y)` — partial application is free. Rust's `add(x, y)` is a true 2-arg function requiring explicit closure wrapping.
2. **`Box<dyn Fn>` overhead**: Rust's generic `curry` must use `Box<dyn Fn>` to erase the return type of the nested closure; OCaml's type system handles this transparently.
3. **`Copy` constraints**: Rust's `curry` requires `A: Copy, B: Copy` to capture arguments in nested closures; OCaml copies integers implicitly.
4. **`flip` ergonomics**: Rust's `flip` requires complex lifetime/trait bounds; OCaml's `let flip f x y = f y x` is trivial.

## Exercises

1. **Point-free style**: Implement `sum_of_squares: Vec<i32> -> i32` using only `map`, `fold`, and curried `add`/`mul` — no explicit lambda bodies.
2. **Curry3**: Implement `curry3` that converts `Fn(A, B, C) -> D` to `Fn(A) -> (Fn(B) -> (Fn(C) -> D))`.
3. **Practical currying**: Use curried `add`, `mul`, and `partial` to build a `tax_calculator(rate: f64)(price: f64) -> f64` and a `discounted_tax(discount: f64)(rate: f64)(price: f64) -> f64`.
