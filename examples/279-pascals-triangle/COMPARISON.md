# OCaml vs Rust: Pascal's Triangle

## Side-by-Side Code

### OCaml
```ocaml
let next_row row =
  List.map2 (+) (0 :: row) (row @ [0])

let pascal n =
  let rec go row i =
    if i > n then []
    else row :: go (next_row row) (i + 1)
  in go [1] 1
```

### Rust (idiomatic)
```rust
pub fn next_row(row: &[u64]) -> Vec<u64> {
    std::iter::once(&0)
        .chain(row.iter())
        .zip(row.iter().chain(std::iter::once(&0)))
        .map(|(a, b)| a + b)
        .collect()
}

pub fn pascal(n: usize) -> Vec<Vec<u64>> {
    std::iter::successors(Some(vec![1u64]), |prev| Some(next_row(prev)))
        .take(n)
        .collect()
}
```

### Rust (functional/recursive)
```rust
pub fn pascal_recursive(n: usize) -> Vec<Vec<u64>> {
    fn go(row: Vec<u64>, i: usize, n: usize) -> Vec<Vec<u64>> {
        if i > n { return Vec::new(); }
        let next = next_row(&row);
        let mut result = vec![row];
        result.extend(go(next, i + 1, n));
        result
    }
    if n == 0 { return Vec::new(); }
    go(vec![1], 1, n)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Next row | `val next_row : int list -> int list` | `fn next_row(row: &[u64]) -> Vec<u64>` |
| Triangle | `val pascal : int -> int list list` | `fn pascal(n: usize) -> Vec<Vec<u64>>` |
| Zip-add | `List.map2 (+) xs ys` | `xs.zip(ys).map(\|(a, b)\| a + b)` |
| Prepend zero | `0 :: row` | `once(&0).chain(row.iter())` |
| Append zero | `row @ [0]` | `row.iter().chain(once(&0))` |

## Key Insights

1. **`List.map2` vs `zip`:** OCaml's `List.map2 f xs ys` applies `f` pairwise in one call; Rust separates zipping from mapping — `.zip().map()` — which is more composable but requires two steps
2. **Prepend/append symmetry:** OCaml's `0 :: row` (O(1) prepend) and `row @ [0]` (O(n) append) have different costs; Rust's `once().chain()` is lazy in both directions — no allocation until `collect()`
3. **`successors` as recursion replacement:** Rust's `std::iter::successors(seed, f)` generates `[seed, f(seed), f(f(seed)), ...]` lazily — it's the iterator equivalent of OCaml's recursive `go` function, but doesn't consume stack frames
4. **Borrowing in `next_row`:** Takes `&[u64]` (borrows the slice) and returns an owned `Vec<u64>` — the caller keeps the previous row while the function builds the next one. In OCaml, GC handles this automatically
5. **Overflow risk:** OCaml's `int` won't overflow on 64-bit (it's 63-bit signed). Rust's `u64` will panic on overflow in debug mode — for very large triangles, consider `u128` or a bigint library

## When to Use Each Style

**Use idiomatic Rust when:** You want clean, readable code — `successors` + `take` is the most Rust-native way to express "generate a sequence from a recurrence relation." It's lazy, composable, and stack-safe.

**Use recursive Rust when:** Teaching the algorithm structure — the recursive version maps 1:1 to the OCaml code and makes the "build row, recurse" pattern explicit. Good for understanding before refactoring to iterators.
