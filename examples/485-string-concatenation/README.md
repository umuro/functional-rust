📖 **[View on hightechmind.io →](https://hightechmind.io/rust/485-string-concatenation)**

---

# String Concatenation
**Difficulty:** ⭐  
**Category:** Functional Programming  



Rust offers four main concatenation strategies — the `+` operator (moves the left operand), `format!` (flexible but always allocates), `join` (separator-aware), and iterator `collect` — each with different ownership, allocation, and readability trade-offs.

## Problem Statement

String concatenation appears everywhere: building HTTP responses, constructing file paths, assembling SQL queries. The naive approach of `a + b + c + ...` allocates a new string at every `+`. A good concatenation strategy should: reuse existing allocations when possible, handle N parts efficiently, and not require transferring ownership unnecessarily. Understanding the trade-offs between the four Rust strategies prevents O(N²) allocation bugs in hot code paths.

## Learning Outcomes

- Use `+` to append a `&str` to a `String` (moves the left side, no extra allocation)
- Use `format!` when mixing types or needing complex formatting (always allocates)
- Use `.join(separator)` for joining a slice of strings efficiently
- Collect from an iterator of `&str` into a `String` via `FromIterator`
- Pre-allocate with `String::with_capacity` when the final size is known

## Rust Application

The `+` operator calls `String::add(self, rhs: &str) -> String`, consuming `self` and appending `rhs` to its buffer in-place when capacity allows:

```rust
let a = String::from("hi");
let b = String::from("!");
let s = a + &b;   // a is moved; b is borrowed
```

`format!` is the most flexible but always allocates a new `String`:

```rust
format!("{}-{}", 1, 2)  // "1-2"
```

`.join` avoids N-1 intermediate allocations for slice joining:

```rust
vec!["a", "b", "c"].join("-")  // "a-b-c"
```

## OCaml Approach

OCaml's `^` operator always allocates a new string:

```ocaml
"hi" ^ "!"  (* new allocation *)
```

For efficient multi-part concatenation, `Buffer` is the idiom:

```ocaml
let buf = Buffer.create 16 in
Buffer.add_string buf "hi";
Buffer.add_string buf "!";
Buffer.contents buf
```

`String.concat "-" ["a";"b";"c"]` is the equivalent of `.join`. OCaml has no `+` move semantics; all strings are immutable.

## Key Differences

1. **Move semantics in `+`**: Rust's `a + &b` moves `a` into the result, reusing its allocation; OCaml's `a ^ b` always creates a new string.
2. **`format!` cost**: Both Rust's `format!` and OCaml's `Printf.sprintf` always allocate; but Rust's `write!(buf, ...)` amortises over a pre-allocated `String`.
3. **Iterator collect**: Rust collects `Iterator<Item=&str>` directly into `String` via `FromIterator`; OCaml needs `String.concat "" (List.map ...)`.
4. **`Buffer` vs. `with_capacity`**: OCaml's `Buffer` is a dedicated mutable builder; Rust's `String` is its own builder via `push_str` with optional `with_capacity`.

## Exercises

1. **Benchmark four strategies**: Use `criterion` to compare `+`, `format!`, `join`, and `push_str` for concatenating 100 strings of 10 chars each.
2. **Allocation-free path builder**: Implement `join_path(components: &[&str]) -> String` using `String::with_capacity` (pre-calculate the exact capacity) and `push_str`.
3. **Interleave**: Write `interleave(parts: &[&str], sep: &str) -> String` equivalent to `.join` but skipping empty parts, using `filter` + `collect` on an iterator.
