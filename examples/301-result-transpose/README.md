📖 **[View on hightechmind.io →](https://hightechmind.io/rust/301-result-transpose)**

---

# 301: Result::transpose() — Flipping Nested Types

**Difficulty:** 2  **Level:** Intermediate

Convert `Result<Option<T>, E>` into `Option<Result<T, E>>` — or back again.

## The Problem This Solves

You're calling a function that returns `Result<Option<T>, E>` — perhaps a database lookup that might fail (IO error) or find nothing (the `None` case). Now you want to use it in a context that expects `Option<Result<T, E>>` — for example, to feed it into `Iterator::filter_map`. You end up writing a four-arm `match` to manually rearrange the two wrapper types.

This awkward nesting comes up constantly when combining fallible operations with optional values. A config key might not exist (`None`) *or* parsing it might fail (`Err`). A cache lookup might miss (`None`) *or* deserializing the cached value might fail (`Err`). The two layers compose in predictable ways — and `transpose()` encodes those rules.

Once you know the rules, the conversion is mechanical: `Ok(None)` means "success, no value" → `None`. `Ok(Some(v))` means "success, got value" → `Some(Ok(v))`. `Err(e)` means "failure" → `Some(Err(e))`. `transpose()` just applies these rules so you don't have to write the match.

## The Intuition

`transpose()` swaps the `Result` and `Option` wrappers — commuting the two layers in a predictable way that preserves all information.

## How It Works in Rust

```rust
// Result::transpose() rules:
let ok_some: Result<Option<i32>, &str> = Ok(Some(42));
let ok_none: Result<Option<i32>, &str> = Ok(None);
let err:     Result<Option<i32>, &str> = Err("bad");

ok_some.transpose()  // => Some(Ok(42))   — success with a value
ok_none.transpose()  // => None            — success with no value (becomes None)
err.transpose()      // => Some(Err("bad")) — failure becomes Some(Err)

// Option::transpose() goes the other direction:
let some_ok: Option<Result<i32, &str>> = Some(Ok(5));
let some_err: Option<Result<i32, &str>> = Some(Err("fail"));
let none: Option<Result<i32, &str>> = None;

some_ok.transpose()  // => Ok(Some(5))    — value present, no error
some_err.transpose() // => Err("fail")    — error propagates out
none.transpose()     // => Ok(None)       — absent is treated as success with no value

// Practical: parse an optional config value cleanly
let config_val: Option<&str> = Some("42");
let result: Result<Option<i32>, _> = config_val
    .map(|s| s.parse::<i32>())  // Option<Result<i32, ParseIntError>>
    .transpose();                // Result<Option<i32>, ParseIntError>
```

The `Option::transpose()` direction is the more commonly useful one — it lets you use `?` on an `Option<Result<T,E>>` after transposing.

## What This Unlocks

- **Clean iterator pipelines** — `filter_map(|opt| opt.map(parse).transpose())` filters `None` and propagates errors
- **Optional config parsing** — turn "key might not exist AND parsing might fail" into a single `Result<Option<T>>`
- **`?` on optional results** — after transposing, use `?` normally

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| `Ok(None)` → `None` | Manual match | `Result::transpose()` |
| `Ok(Some(v))` → `Some(Ok(v))` | Manual match | `Result::transpose()` |
| `Some(Ok(v))` → `Ok(Some(v))` | Manual match | `Option::transpose()` |
| Use case | Manual unwrapping | Composing optional + fallible operations |
