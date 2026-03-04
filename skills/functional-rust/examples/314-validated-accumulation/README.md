# 314: Validated — Accumulating All Errors

**Difficulty:** 4  **Level:** Expert

Collect every validation failure at once — instead of stopping at the first one.

## The Problem This Solves

A user submits a registration form with an invalid name, a malformed email, and an out-of-range age. With `Result`, you'd validate the name, get an `Err`, and return immediately — the user sees only the first error, fixes it, resubmits, and gets the second error. This "one error at a time" UX is frustrating and broken.

The fundamental issue is that `Result` is a *monad* — it chains operations sequentially, and one failure stops the chain. But form validation doesn't have sequential dependencies: name validity is independent of email validity is independent of age validity. You want to run all three, collect all failures, and return everything at once.

This is the `Validated` pattern — sometimes called the applicative functor approach. The key operation is `combine`: merge two `Validated` values where if both fail, the error lists are concatenated (not short-circuited). The result is the functional programming equivalent of "gather all errors and show them all."

## The Intuition

`Validated` is `Result` with error accumulation: `combine` merges two independent results, collecting all errors from both sides instead of returning the first.

## How It Works in Rust

```rust
#[derive(Debug)]
enum Validated<T, E> {
    Valid(T),
    Invalid(Vec<E>),  // multiple errors, not just one
}

// The key function — this is what makes it applicative, not monadic
fn combine<A, B, E>(a: Validated<A, E>, b: Validated<B, E>) -> Validated<(A, B), E> {
    match (a, b) {
        (Validated::Valid(a), Validated::Valid(b)) =>
            Validated::Valid((a, b)),                    // both succeed
        (Validated::Invalid(mut e1), Validated::Invalid(e2)) => {
            e1.extend(e2);                               // BOTH fail: accumulate errors
            Validated::Invalid(e1)
        }
        (Validated::Invalid(e), _) | (_, Validated::Invalid(e)) =>
            Validated::Invalid(e),                       // one fails: carry forward
    }
}

// Usage: validate all fields independently, then combine
fn validate_registration(name: &str, email: &str, age: &str)
    -> Validated<(String, String, u8), String>
{
    // All three run — no short-circuiting
    combine(
        combine(validate_name(name), validate_email(email)),
        validate_age(age),
    ).map(|((n, e), a)| (n, e, a))
}

// With all fields invalid: returns 3 errors, not 1
match validate_registration("", "bad", "999") {
    Validated::Invalid(errs) => println!("All {} errors: {:?}", errs.len(), errs),
    Validated::Valid(_) => unreachable!(),
}
```

The contrast with `Result`:
- `Result` with `?`: "validate name — if bad, stop. validate email — if bad, stop. validate age."
- `Validated` with `combine`: "validate name AND email AND age — collect all failures."

Don't use `and_then` on `Validated` when you want accumulation — `and_then` is sequential and discards later errors, defeating the purpose.

## What This Unlocks

- **Complete form validation** — show all errors in one response; standard UX expectation for web forms and CLI tools
- **Batch input validation** — validate a CSV row, report all column errors at once
- **API request validation** — return a structured list of field errors to the client in one round-trip

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Category theory | Applicative functor | Custom `Validated` type with `combine` |
| Error accumulation | `Validation` library or manual | Custom `combine` function |
| Monad vs applicative | `let*` = monadic (sequential) | `and_then` = monadic; `combine` = applicative |
| Short-circuits? | Monad: yes; Applicative: no | `Result`/`?`: yes; `Validated`/`combine`: no |
