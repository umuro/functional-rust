# 402: Index and IndexMut Traits

**Difficulty:** 2  **Level:** Intermediate

Implement `[]` indexing for your own types — with any index type, not just integers.

## The Problem This Solves

You've built a `Matrix` type and want to access elements with `m[(row, col)]`. You've built a `Config` type and want `cfg["host"]`. Without the `Index` trait, you must expose a getter method: `m.get(row, col)`, `cfg.get("host")`. The method call syntax is fine, but it disrupts familiar patterns, breaks slice-compatible code, and makes your type feel like a second-class citizen.

The `Index` and `IndexMut` traits let you override the `[]` operator for any index type. Your `Matrix` can index by `(usize, usize)` tuples. Your `Config` can index by `&str`. The compiler desugars `m[(row, col)]` to `*m.index((row, col))` — giving you a reference, not a copy. `IndexMut` gives you a mutable reference, enabling `m[(0, 0)] = 99.0`.

## The Intuition

`Index<Idx>` is a trait generic over the index type. `Output` is an associated type — what you get back. `index(&self, idx: Idx) -> &Self::Output` returns an immutable reference. `IndexMut` adds `index_mut(&mut self, idx: Idx) -> &mut Self::Output`.

The compiler uses these automatically when it sees `container[index]`. On the right of an assignment it calls `index`. On the left it calls `index_mut`. This is why `m[(0, 0)] = 99.0` works for mutable matrices.

## How It Works in Rust

```rust
use std::ops::{Index, IndexMut};

struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix { rows, cols, data: vec![0.0; rows * cols] }
    }
}

// Index by (row, col) tuple — not restricted to usize!
impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &f64 {
        assert!(row < self.rows && col < self.cols, "out of bounds");
        &self.data[row * self.cols + col]  // returns reference into internal storage
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        assert!(row < self.rows && col < self.cols, "out of bounds");
        &mut self.data[row * self.cols + col]
    }
}

// String-indexed config
use std::collections::HashMap;
struct Config(HashMap<String, String>);

impl Index<&str> for Config {
    type Output = String;
    fn index(&self, key: &str) -> &String {
        self.0.get(key).unwrap_or_else(|| panic!("Key not found: {}", key))
    }
}

fn main() {
    let mut m = Matrix::new(3, 3);
    m[(1, 2)] = 42.0;             // uses IndexMut
    println!("m[1][2] = {}", m[(1, 2)]);  // uses Index

    let mut cfg = Config(HashMap::new());
    cfg.0.insert("host".to_string(), "localhost".to_string());
    println!("host = {}", cfg["host"]);   // &str index
}
```

**Common patterns for Index:**
- `Vec<T>` uses `Index<usize>` — zero-overhead, returns reference into heap data
- `HashMap<K, V>` uses `Index<&K>` — panics on missing key (use `.get()` for `Option`)
- `str`/`String` use `Index<Range<usize>>` — slicing with `s[1..4]`

## What This Unlocks

- **Domain-specific indexing** — `m[(row, col)]`, `cfg["key"]`, `graph[NodeId(5)]` — types document their own natural index shape.
- **Generic algorithms** — code written for slices (`fn sum<T: Index<usize>>(...)`) works with any indexable type.
- **Write-through indexing** — `IndexMut` enables `matrix[(i, j)] += 1.0` as ergonomically as built-in array access.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Custom indexing | `get m r c` — free function, no operator | `m[(r, c)]` — `Index` trait desugars `[]` |
| Index type | Always functional application | Any type that satisfies the `Index<Idx>` impl |
| Mutable indexing | `set m r c v` — separate function | `IndexMut` — `m[(r,c)] = v` assigns through reference |
| Panic on bounds | `failwith "out of bounds"` | `panic!` — same semantics, use `.get()` for `Option` |
