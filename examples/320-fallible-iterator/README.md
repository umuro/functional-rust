📖 **[View on hightechmind.io →](https://hightechmind.io/rust/320-fallible-iterator)**

---

# 320: Fallible Iterators

**Difficulty:** 2  **Level:** Intermediate

An iterator that can fail — `Iterator<Item = Result<T, E>>` — and how to drive it with fail-fast or best-effort collection.

## The Problem This Solves

Standard iterators are assumed infallible: each `.next()` call either gives you a value or signals that the sequence is done. But many real sources of data can fail mid-stream: reading lines from a file, parsing records from a network socket, deserializing rows from a database cursor. Pretending each step can't fail leads to panics or swallowed errors.

The idiomatic Rust answer is `Iterator<Item = Result<T, E>>`. Each element is either a successful value or an error. This keeps the iterator API intact while exposing the possibility of failure at each step. The hard part is deciding what to do with those errors — collect everything and report the first failure? Skip bad records and continue? Partition successes and failures for separate handling?

Rust's standard library provides all these strategies through different collection patterns, and they compose naturally with existing iterator adapters.

## The Intuition

Think of a fallible iterator as a conveyor belt where any item might be marked "defective." You have three choices: stop the line on first defect (fail-fast), let defective items fall off (filter), or sort them into two bins (partition). All three are expressible with a single `Iterator<Item = Result<T, E>>` as input.

## How It Works in Rust

```rust
// Fail-fast: collect into Result — short-circuits on first Err
let numbers: Result<Vec<i32>, _> = ["1", "2", "bad", "4"]
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();
// → Err(ParseIntError) — stops at "bad", never processes "4"

// Best-effort: filter_map drops errors, collects successes
let valid: Vec<i32> = ["1", "bad", "3"]
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();
// → [1, 3]

// Partition: collect both successes and failures
let (oks, errs): (Vec<_>, Vec<_>) = inputs
    .iter()
    .map(|s| s.parse::<i32>())
    .partition(Result::is_ok);
let values: Vec<i32> = oks.into_iter().map(Result::unwrap).collect();

// Process one at a time with manual control
for item in iter.map(|s| s.parse::<i32>()) {
    match item {
        Ok(n) => process(n),
        Err(e) => eprintln!("skipping: {}", e),
    }
}
```

## What This Unlocks

- **Safe streaming** — process file lines, network packets, or database rows with explicit error handling at each step, not a panic buried in production
- **Composable strategies** — choose fail-fast, skip, or partition based on the application's tolerance for partial results, without changing the iterator source
- **Library compatibility** — the `fallible-iterator` crate extends this pattern with an ergonomic `FallibleIterator` trait if you need `?` to work naturally at each step

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Fallible sequence | `Seq.t` with `option`/`result` element | `Iterator<Item=Result<T,E>>` |
| Fail-fast collect | Manual `fold` with early exit | `.collect::<Result<Vec<_>,_>>()` |
| Skip errors | `Seq.filter_map` | `.filter_map(\|r\| r.ok())` |
| Partition | `List.partition` | `.partition(Result::is_ok)` |
| Lazy evaluation | `Seq` (lazy by default) | Iterators (lazy by default) |
