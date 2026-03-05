📖 **[View on hightechmind.io →](https://hightechmind.io/rust/439-macro-assert-variants)**

---

# 439: assert_matches! and Variant Assertions

**Difficulty:** 2  **Level:** Intermediate

Use `matches!` and `assert_matches!` to write pattern-match assertions — checking enum variants and guard conditions in one readable expression without manually destructuring.

## The Problem This Solves

Testing functions that return enums is awkward with `assert_eq!`. You either need to derive `PartialEq` on every enum (including ones where equality semantics are unclear), or you write verbose destructuring in every test:

```rust
match parse("42") {
    Parsed::Int(n) => assert_eq!(n, 42),
    other => panic!("Expected Int(42), got {:?}", other),
}
```

That's five lines for one assertion. Multiply by a dozen tests and your test suite is more boilerplate than logic. It also doesn't compose — you can't use it inside `assert!()` or `all()`.

`matches!($val, Pattern)` solves this: it's a boolean expression that evaluates to `true` if `$val` matches the pattern. You can add guards (`if n > 50`), use it in `assert!`, compose it with `.all()` over iterators, or use it in `if` conditions without creating a separate variable.

## The Intuition

`matches!(expr, pattern)` is a syntactic shorthand for:

```rust
match expr { pattern => true, _ => false }
```

The compiler desugars it exactly that way — no overhead, no allocation, just a match expression. The `if` guard syntax works too: `matches!(x, Some(n) if n > 0)` becomes `match x { Some(n) if n > 0 => true, _ => false }`.

`assert_matches!` (stabilised in Rust 1.82) adds a panic message when the match fails, showing both the pattern and the actual value. For earlier Rust versions, `assert!(matches!(...))` gives the same effect with a slightly less informative message.

## How It Works in Rust

```rust
#[derive(Debug, PartialEq)]
enum Parsed { Int(i64), Float(f64), Invalid(String) }

fn parse(s: &str) -> Parsed { /* ... */ }

// ── matches! in assertions ────────────────────────────────────────────────────
assert!(matches!(parse("42"), Parsed::Int(42)));
assert!(matches!(parse("3.14"), Parsed::Float(_)));    // _ matches any value
assert!(matches!(parse("abc"), Parsed::Invalid(_)));

// ── With guard conditions ─────────────────────────────────────────────────────
assert!(matches!(parse("100"), Parsed::Int(n) if n > 50));
assert!(matches!(parse("3.14"), Parsed::Float(f) if f > 3.0));

// ── In if conditions — no binding variable needed ────────────────────────────
if matches!(parse("3.14"), Parsed::Float(f) if f > 3.0) {
    println!("Got float > 3");
}

// ── Composing with iterators ──────────────────────────────────────────────────
let results: Vec<Result<u32, String>> = vec![Ok(1), Ok(2), Ok(3)];
assert!(results.iter().all(|r| matches!(r, Ok(n) if *n > 0)));

// ── assert_matches! (Rust 1.82+) — shows pattern in failure message ──────────
// #![feature(assert_matches)]  // or stable in 1.82+
// use std::assert_matches::assert_matches;
// assert_matches!(parse("42"), Parsed::Int(_));  // better failure message

// ── Pattern matching without extracting the value ────────────────────────────
fn is_valid(s: &str) -> bool {
    !matches!(parse(s), Parsed::Invalid(_))
}
```

## What This Unlocks

- **Enum-returning function tests** — assert on shape without deriving `PartialEq` on the whole enum or writing multi-line destructuring.
- **Iterator assertions** — `assert!(results.iter().all(|r| matches!(r, Ok(_))))` — concise whole-collection checks.
- **Readable conditionals** — `if matches!(state, State::Ready | State::Idle)` is clearer than nested `if let` chains for multi-variant checks.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Pattern-match assertion | `OUnit: assert_equal` with custom equality; or `match ... | _ -> assert_failure` | `matches!` macro — inline boolean; `assert_matches!` in 1.82+ |
| Guard in assertion | Inline `when` clause in pattern | `matches!(x, Pat if guard)` |
| Composing with iterators | `List.for_all (fun x -> match x with ...) lst` | `.all(|x| matches!(x, Pattern))` |
| Partial match (ignore variant data) | `match x with Foo _ -> true | _ -> false` | `matches!(x, Foo(_))` |
