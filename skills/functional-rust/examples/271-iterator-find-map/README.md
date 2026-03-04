# 271: Transform-and-Find with find_map()

**Difficulty:** 2  **Level:** Intermediate

Find the first element that both matches and transforms successfully — returns the transformed value, not the original.

## The Problem This Solves

You're scanning a list looking for the first element that passes a test *and* needs to be transformed before you use it. Parse the first valid integer from a list of strings. Find the first filename with a `.rs` extension and return just the stem. Find the first key=value pair in a config file and return the parsed pair. 

Without `find_map`, you'd write `filter_map(f).next()` — or a manual loop with an `if let Some(x) = transform(element)` that breaks on success. Both work, but `find_map` makes the intent explicit: "find the first element for which this transformation succeeds."

The key is that the closure returns `Option<B>` — `None` means "skip this element", `Some(value)` means "found it, return this value." OCaml doesn't have a built-in equivalent; you'd use `List.find_map` (added in OCaml 4.10) or compose `filter_map` with `List.nth`.

## The Intuition

`find_map(f)` applies `f` to each element in order. The first time `f` returns `Some(value)`, it stops and returns that `Some(value)`. If `f` returns `None` for every element, the result is `None`.

```rust
let strings = ["hello", "42", "world", "17"];
let first_int = strings.iter().find_map(|s| s.parse::<i32>().ok());
// → Some(42)   stops at "42", never processes "world" or "17"
```

## How It Works in Rust

```rust
let strings = ["hello", "42", "world", "17", "foo"];

// Parse: first valid integer
let first_int = strings.iter().find_map(|s| s.parse::<i32>().ok());
// → Some(42)

// Conditional transform: first word longer than 4 chars, return its length
let first_long_len = strings.iter()
    .find_map(|s| if s.len() > 4 { Some(s.len()) } else { None });
// → Some(5)  ("hello" has 5 chars)

// Parse key=value pairs — use ? operator inside closure
let env_vars = ["PATH=/usr/bin", "HOME=/root", "BAD", "USER=alice"];
let first_kv = env_vars.iter().find_map(|s| {
    let mut parts = s.splitn(2, '=');
    let key = parts.next()?;   // None if empty
    let val = parts.next()?;   // None if no '='
    Some((key, val))
});
// → Some(("PATH", "/usr/bin"))

// Strip suffix: first .rs file stem
let files = ["main.txt", "lib.rs", "README.md", "util.rs"];
let first_rs_stem = files.iter().find_map(|f| f.strip_suffix(".rs"));
// → Some("lib")

// find_map(f) == filter_map(f).next()  — they're equivalent
let equiv = strings.iter().filter_map(|s| s.parse::<i32>().ok()).next();
assert_eq!(first_int, equiv);
```

The `?` operator inside the closure is idiomatic — it converts `None`/`Err` to `None` and returns early from the closure.

## What This Unlocks

- **Search + parse in one pass** — scan strings/tokens and return the first successfully parsed value.
- **Conditional extraction** — return the first element that passes a condition *and* its derived value (not the original).
- **Early-exit transformation** — stop as soon as you find what you need, without allocating an intermediate collection.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Find and transform | `List.find_map` (4.10+) | `iter.find_map(f)` |
| Stops at first `Some` | Yes | Yes — lazy, early exit |
| Equivalent to | `filter_map f lst \|> List.hd_opt` | `.filter_map(f).next()` |
| Returns | `'b option` | `Option<B>` |
| Closure return type | `'a -> 'b option` | `FnMut(T) -> Option<B>` |
