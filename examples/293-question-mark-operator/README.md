📖 **[View on hightechmind.io →](https://hightechmind.io/rust/293-question-mark-operator)**

---

# 293: The ? Operator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Chaining fallible operations with `and_then()` is composable but visually noisy when there are many sequential steps. The `?` operator provides syntactic sugar for early return on failure: `expr?` desugars to `match expr { Ok(v) => v, Err(e) => return Err(e.into()) }`. This makes error propagation code read like imperative code while retaining the type safety of explicit `Result` types. It is Rust's equivalent of OCaml's `let*` syntax and Haskell's `do` notation.

## Learning Outcomes

- Understand `?` as desugaring to early-return on `Err` (or `None`)
- Recognize that `?` calls `From::from(e)` on the error — enabling automatic type conversion
- Use `?` in functions returning `Result` to chain multiple fallible operations
- Understand when to use `?` vs `and_then()`: sequential steps vs branching/nested logic

## Rust Application

The `?` operator works in any function returning `Result<_, E>` (or `Option<_>`) where `E: From<OtherError>`:

```rust
fn process(s: &str, divisor: i32) -> Result<i32, AppError> {
    let n: i32 = s.parse()?;         // ParseIntError -> AppError via From
    if n < 0 { return Err(AppError::NegativeInput); }
    let q = n / divisor;             // no ?; not fallible here
    if divisor == 0 { return Err(AppError::DivByZero); }
    Ok(q * 2)
}
```

The `impl From<ParseIntError> for AppError` enables automatic error conversion at the `?` site.

## OCaml Approach

OCaml's `let*` binding (4.08+) is the exact equivalent — it desugars `let* x = expr in rest` to `Result.bind expr (fun x -> rest)`:

```ocaml
let process s divisor =
  let* n = int_of_string_opt s |> Option.to_result ~none:`NotANumber in
  if n < 0 then Error `Negative
  else Ok (n / divisor * 2)
```

OCaml does not perform automatic error type conversion at `let*` — the error type must already match.

## Key Differences

1. **Auto-conversion**: Rust's `?` calls `From::from()` on the error automatically; OCaml's `let*` requires the error type to already unify.
2. **Works on both**: Rust's `?` works on both `Result` and `Option` in the same function returning `Option`; OCaml has separate bind for each.
3. **Syntactic position**: `?` is a postfix operator in Rust; `let*` is a prefix binding form in OCaml.
4. **Propagation level**: `?` always returns from the enclosing function; `and_then` can propagate within an expression without returning.

## Exercises

1. Rewrite a function that uses three `and_then()` calls to use `?` instead, and verify they produce identical results.
2. Implement two custom error types and use `?` to convert between them automatically via `impl From`.
3. Write a function that uses `?` in a `main()` returning `Result<(), Box<dyn Error>>` to demonstrate the top-level error propagation pattern.
