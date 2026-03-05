📖 **[View on hightechmind.io →](https://hightechmind.io/rust/313-try-trait)**

---

# 313: The Try Trait — What ? Actually Does

**Difficulty:** 4  **Level:** Expert

Understand the mechanism behind `?` — and how to extend it to custom types.

## The Problem This Solves

You use `?` constantly, but it's a black box: it somehow short-circuits on failure, converts error types, and continues on success. What's actually happening? And why can't you use `?` on your custom `Validated<T, E>` type that accumulates errors instead of short-circuiting?

The answer is the `Try` trait (currently unstable as `std::ops::Try`). It defines two operations: decompose a value into "output" (success, continue) or "residual" (failure, short-circuit), and reconstruct a value from either. `?` calls these operations. `Result` and `Option` both implement `Try`. Your custom type doesn't — which is why `?` doesn't work on it.

On stable Rust, you can't implement `Try` for your own types. But understanding what `?` does lets you replicate the *behavior* manually using `and_then` chains. And when `Try` stabilizes, you'll know exactly which trait to implement.

## The Intuition

`?` desugars to: "decompose this value — if it's the success case, extract the value and continue; if it's the failure case, convert and return early." The `Try` trait defines what those two cases mean for each type.

## How It Works in Rust

```rust
// What ? actually desugars to (simplified):
// expr?
// becomes:
match Try::branch(expr) {
    ControlFlow::Continue(v) => v,       // success: extract value, continue
    ControlFlow::Break(r) => {
        return FromResidual::from_residual(r);  // failure: convert and return
    }
}

// For Result<T, E>, the Try impl is:
// - branch(Ok(v))  => ControlFlow::Continue(v)
// - branch(Err(e)) => ControlFlow::Break(Err(e))
// The FromResidual impl calls From::from(e) to convert the error type.

// On stable: replicate ? behavior with and_then
fn process_stable(input: &str) -> Result<String, AppError> {
    parse_number(input)           // Result<i32, ParseError>
        .map_err(AppError::Parse) // convert error type (what ? + From does)
        .and_then(|n| {           // continue if Ok (what ? does on success)
            validate(n).map_err(AppError::Validation)
        })
        .map(|n| format!("result: {}", n))
}

// Custom type: can't use ?, but can implement the same semantics manually
impl<T, E> Validated<T, E> {
    // This is what ? would call if Validated implemented Try
    fn branch(self) -> Result<T, Vec<E>> {
        match self {
            Validated::Ok(v) => Ok(v),
            Validated::Err(es) => Err(es),
        }
    }
    // and_then gives us sequential chaining (like ? but explicit)
    fn and_then<U, F: FnOnce(T) -> Validated<U, E>>(self, f: F) -> Validated<U, E> {
        match self { Validated::Ok(v) => f(v), Validated::Err(es) => Validated::Err(es) }
    }
}
```

The reason `Validated` can't use `?` even conceptually: `?` short-circuits — it returns on the first failure. `Validated` *accumulates* failures. These two semantics are incompatible. `?` is monadic (sequential), `Validated` is applicative (parallel). They solve different problems.

## What This Unlocks

- **Deep understanding of `?`** — know exactly what happens at every `?` site, including the `From` call for error conversion
- **Debugging type errors** — when `?` doesn't compile, you know exactly which `From` impl or `Try` bound is missing
- **Custom control flow types** — when `Try` stabilizes, you can implement it for `Option`-like types with different semantics

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Monadic bind | `let*` in `Result` or custom monad | `?` operator (calls `Try::branch`) |
| Custom monad | Custom `let*` via ppx or module | `Try` trait (unstable) — use `and_then` on stable |
| Short-circuit | `Result.bind` returns early on `Error` | `?` returns early on `ControlFlow::Break` |
| Applicative | Separate `map2` / `both` | No `?` for applicative — use explicit `combine` |
