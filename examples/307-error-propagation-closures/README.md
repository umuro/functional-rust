📖 **[View on hightechmind.io →](https://hightechmind.io/rust/307-error-propagation-closures)**

---

# 307: Error Propagation in Closures
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The `?` operator propagates errors from the enclosing function. Inside a closure passed to `map()` or `and_then()`, `?` propagates from the closure, not the outer function. This distinction matters for `Iterator::map()`: the closure returns `Result<T, E>`, not `T`, and the results must be collected or handled. Understanding how errors flow through closures is essential for writing correct iterator pipelines over fallible operations.

## Learning Outcomes

- Understand that `?` inside a closure propagates from the closure, not the outer function
- Use `map(|s| s.parse::<i32>())` to produce `Iterator<Item = Result<i32, E>>`
- Collect results with short-circuiting via `collect::<Result<Vec<_>, _>>()`
- Use `filter_map(|s| s.parse().ok())` to silently drop errors

## Rust Application

The key insight is that closures returning `Result` make the iterator yield `Result` values:

```rust
// collect::<Result<Vec<_>, _>>() short-circuits on first error
pub fn parse_all(inputs: &[&str]) -> Result<Vec<i32>, String> {
    inputs.iter().map(|s| parse_number(s)).collect()
}

// filter_map with .ok() silently drops parse errors
pub fn parse_valid(inputs: &[&str]) -> Vec<i32> {
    inputs.iter().filter_map(|s| s.parse::<i32>().ok()).collect()
}

// ? inside a closure propagates from the closure:
fn process(inputs: &[&str]) -> Result<Vec<i32>, String> {
    let doubled: Result<Vec<i32>, String> = inputs.iter()
        .map(|s| { let n = parse_number(s)?; Ok(n * 2) })
        .collect();
    doubled
}
```

## OCaml Approach

OCaml's `let*` binding with `Seq` or `List` functions provides similar behavior — errors propagate within the `let*` chain, not from closures:

```ocaml
let parse_all inputs =
  List.fold_right (fun s acc ->
    let* lst = acc in
    let* n = parse_number s in
    Ok (n :: lst)
  ) inputs (Ok [])
```

## Key Differences

1. **Closure boundary**: `?` propagates to the enclosing function; inside a closure passed to `map()`, it propagates from the closure (making the closure return `Result`).
2. **Two strategies**: Collect-with-short-circuit vs filter-and-continue are the two fundamental choices for handling errors in iterator pipelines.
3. **Closure return type**: The `?` operator requires the closure to return `Result<T, E>` or `Option<T>` — it doesn't work in closures returning plain `T`.
4. **Accumulating errors**: To collect all errors (not just the first), use `fold()` building up a `Vec<E>` instead of `collect::<Result<_, _>>`.

## Exercises

1. Write a pipeline that parses `&[&str]` into numbers, doubles them, and collects both parsed values and unparseable strings into separate `Vec`s in a single pass.
2. Implement `try_map<T, U, E>(v: Vec<T>, f: impl Fn(T) -> Result<U, E>) -> Result<Vec<U>, E>` using iterator combinators.
3. Demonstrate the closure boundary rule: show that `?` inside a closure does NOT propagate to the outer function by using it inside a `map()` closure.
