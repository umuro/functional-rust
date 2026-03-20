📖 **[View on hightechmind.io →](https://hightechmind.io/rust/292-option-combinators)**

---

# 292: Option Combinators
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Null pointer errors are the "billion dollar mistake" — unhandled absence of a value causing runtime crashes. Rust's `Option<T>` encodes optionality in the type system, and its combinator methods enable composing operations on optional values without null checks or sentinel values. This mirrors OCaml's `option` type and Haskell's `Maybe` monad — the foundational functional programming approach to nullable values.

## Learning Outcomes

- Use `map()` to transform `Some(T)` while passing `None` through unchanged
- Use `filter()` to conditionally discard a `Some` value based on a predicate
- Chain optional lookups with `and_then()` — the monadic bind for `Option`
- Provide defaults with `unwrap_or()`, `unwrap_or_else()`, and `unwrap_or_default()`

## Rust Application

Option combinators eliminate explicit `None` checks:

```rust
// map: transform Some, pass None through
pub fn double_option(opt: Option<i32>) -> Option<i32> {
    opt.map(|x| x * 2)
}

// filter: discard Some if condition fails
pub fn filter_even(opt: Option<i32>) -> Option<i32> {
    opt.filter(|&x| x % 2 == 0)
}

// and_then: chain optional lookups
pub fn parse_and_sqrt(s: &str) -> Option<f64> {
    s.parse::<f64>().ok().and_then(|x| if x >= 0.0 { Some(x.sqrt()) } else { None })
}

// or_else: provide alternative
pub fn first_valid(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    a.or(b)
}
```

## OCaml Approach

OCaml's `Option` module provides `Option.map`, `Option.bind`, and `Option.fold`. The `let*` syntax (OCaml 4.08+) desugars to `Option.bind`:

```ocaml
let parse_and_sqrt s =
  let* x = float_of_string_opt s in
  if x >= 0.0 then Some (sqrt x) else None
```

## Key Differences

1. **Naming**: Rust uses `and_then` for monadic bind; OCaml uses `Option.bind` and `let*` syntax.
2. **Filter**: Rust provides `Option::filter()` directly; OCaml requires `Option.bind (fun x -> if pred x then Some x else None)`.
3. **Defaults**: Rust has three variants: `unwrap_or(val)` (eager), `unwrap_or_else(f)` (lazy), `unwrap_or_default()` (type's `Default`).
4. **Zipping**: `Option::zip(other)` combines two options into a pair — no OCaml equivalent without manual matching.

## Exercises

1. Chain three `Option`-returning lookups (user → profile → avatar URL) using `and_then()`, returning `None` if any step fails.
2. Implement a `safe_divide(a: f64, b: f64) -> Option<f64>` and compose it with `filter(|&x| x.is_finite())` using only combinators.
3. Convert between `Option<Result<T, E>>` and `Result<Option<T>, E>` using `transpose()` and verify the two directions invert each other.
