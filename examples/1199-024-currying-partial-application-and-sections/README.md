# Example 1199-024: Currying, Partial Application, and Operator Sections

**Difficulty:** ⭐⭐  
**Category:** Higher-Order Functions | Closures  
**OCaml Source:** Real World OCaml / Standard library idioms

## Problem Statement

Show how OCaml's automatic currying, partial application, operator sections, and
labeled arguments translate to idiomatic Rust using closures and higher-order functions.

## Learning Outcomes

- Rust functions return closures to simulate OCaml's automatic currying
- `impl Fn(T) -> U` is the return type for a partially-applied function
- Operator sections become closure factories (`multiply(2)` instead of `( * ) 2`)
- OCaml labeled args (`~scale ~shift`) become Rust curried parameter order
- `Box<dyn Fn>` is needed when the inner closure type must be named (e.g., in `curry`/`uncurry`)

## OCaml Approach

OCaml curries every function by default: `let add x y = x + y` is actually
`fun x -> fun y -> x + y`. This means `add 5` is already a value of type
`int -> int` — no extra syntax required. Operator sections like `( * ) 2` and
`Fun.flip ( / ) 2` exploit this to build predicates and transformers inline.

## Rust Approach

Rust functions are not automatically curried. To get `add(5)` as a value of type
`impl Fn(i32) -> i32`, we return a capturing closure: `move |y| x + y`. The
`move` keyword transfers ownership of `x` into the closure. For generic
`curry`/`uncurry` conversions, `Box<dyn Fn(B) -> C>` stands in for the unnamed
inner closure type that OCaml expresses transparently.

## Key Differences

1. **Automatic vs explicit currying:** OCaml curries for free; Rust requires returning a closure.
2. **Operator sections:** OCaml writes `( * ) 2`; Rust writes `move |x| x * 2` or a factory.
3. **`Fun.flip`:** OCaml flips argument order with a combinator; Rust just writes the closure directly.
4. **Labeled arguments:** OCaml's `~scale ~shift` allows out-of-order partial application; Rust fixes the order and curries sequentially.
5. **`impl Fn` vs `Box<dyn Fn>`:** Rust uses `impl Fn` for simple cases and `Box<dyn Fn>` when the type must be stored or returned through a trait object boundary.
