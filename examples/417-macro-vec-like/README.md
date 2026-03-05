📖 **[View on hightechmind.io →](https://hightechmind.io/rust/417-macro-vec-like)**

---

# 417: Implementing vec!-like Macros

**Difficulty:** 3  **Level:** Advanced

Build collection-literal macros using declarative macro repetition — the same pattern that powers `vec!`, `hashmap!`, and `format!`.

## The Problem This Solves

Constructing collections with literal syntax is natural in many languages but Rust has no built-in syntax for `{1: "a", 2: "b"}` or `{1, 2, 3}` as a `HashSet`. You need `HashMap::new()` + repeated `insert()` calls — five lines for what should be one. Rust's answer is: write the macro yourself, once, and use it everywhere.

`macro_rules!` gives you pattern matching on token streams. The key pattern is repetition: `$($x:expr),*` matches zero or more comma-separated expressions and `$(action)*` repeats a block for each captured match. This is how `vec![1, 2, 3]` works — it matches `$($x:expr),*` and expands to a series of `.push($x)` calls.

Understanding this pattern lets you build `hashset!`, `hashmap!`, `deque!`, or any domain-specific collection literal. The macro runs at compile time; the output is exactly what you'd write by hand.

## The Intuition

`$($x:expr),*` captures "zero or more comma-separated expressions" and `$(body)*` repeats the body once per capture — this repetition pattern is the core of all collection-building macros.

## How It Works in Rust

```rust
// Replicate how vec! works
macro_rules! my_vec {
    () => { Vec::new() };
    ($($x:expr),+ $(,)?) => {{   // $(,)? = optional trailing comma
        let mut v = Vec::new();
        $(v.push($x);)+          // expand once per captured $x
        v
    }};
}

// hashset! literal
macro_rules! hashset {
    ($($x:expr),* $(,)?) => {{
        let mut s = std::collections::HashSet::new();
        $(s.insert($x);)*
        s
    }};
}

// hashmap! with key => value syntax
macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {{
        let mut m = std::collections::HashMap::new();
        $(m.insert($k, $v);)*
        m
    }};
}

// Usage
let v = my_vec![1, 2, 3, 4];
let s = hashset!["a", "b", "c"];
let m = hashmap!["one" => 1, "two" => 2,];  // trailing comma ok
```

1. `macro_rules! name { (pattern) => { expansion } }` — arms match like `match`.
2. `$x:expr` captures one expression. `$($x:expr),*` captures a comma-separated list.
3. `$(body)+` expands `body` once per element (requires ≥1). `*` allows zero.
4. `$(,)?` at the end accepts an optional trailing comma — good style.

## What This Unlocks

- **Collection literals**: `hashmap!`, `hashset!`, `deque!` — write once, use like built-ins.
- **DSL syntax**: Custom repetition patterns for configuration, query building, test fixtures.
- **Zero runtime cost**: Macros expand at compile time to plain Rust code — no overhead over hand-written loops.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| List literal | `[1; 2; 3]` built-in | `vec![1, 2, 3]` via macro |
| Custom syntax | PPX (preprocessor extensions) | `macro_rules!` declarative macros |
| Repetition pattern | Camlp4/PPX sequence matching | `$($x:expr),*` + `$(body)*` |
| Compile-time expansion | PPX (complex setup) | `macro_rules!` — built into language |
| Map literal | No standard literal | Write `hashmap!` macro yourself |
