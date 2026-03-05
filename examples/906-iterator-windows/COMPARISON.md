# OCaml vs Rust: Sliding Windows over Slices

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml has no built-in windows — must allocate a new sub-array each step *)
let windows n arr =
  let len = Array.length arr in
  if n > len then [||]
  else Array.init (len - n + 1) (fun i -> Array.sub arr i n)

(* Moving average using allocated windows *)
let moving_average k arr =
  let ws = windows k arr in
  Array.map (fun w ->
    float_of_int (Array.fold_left (+) 0 w) /. float_of_int k
  ) ws
```

### Rust (idiomatic)
```rust
// Zero-copy: windows() yields &[T] references into the original slice
pub fn moving_average(data: &[i32], k: usize) -> Vec<f64> {
    data.windows(k)
        .map(|w| w.iter().sum::<i32>() as f64 / k as f64)
        .collect()
}
```

### Rust (functional/chained)
```rust
// Local maxima: combine windows() with enumerate() and filter()
pub fn local_maxima(data: &[i32]) -> Vec<usize> {
    data.windows(3)
        .enumerate()
        .filter(|(_, w)| w[1] > w[0] && w[1] > w[2])
        .map(|(i, _)| i + 1)
        .collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Windows function | `val windows : int -> 'a array -> 'a array array` | `fn windows(&[T], usize) -> impl Iterator<Item=&[T]>` |
| Each window | `'a array` (heap-allocated copy) | `&[T]` (borrowed sub-slice, zero-copy) |
| Result collection | `Array.map` returning `'a array` | `.collect::<Vec<_>>()` |
| Memory model | Allocates `L - n + 1` new arrays | Single allocation for the original slice |

## Key Insights

1. **Zero-copy vs allocation:** Rust's `windows(n)` returns `&[T]` references into the *original* data — no heap allocation per window. OCaml's `Array.sub` allocates a fresh array for every window, which means O(L·n) allocations for a slice of length L with window size n.

2. **Built-in vs hand-rolled:** `windows()` is a first-class method on Rust slices (`&[T]`). OCaml's standard library has no equivalent; idiomatic OCaml requires `Array.init` + `Array.sub` or a recursive helper, both of which allocate.

3. **Iterator composition:** Because `windows()` returns an `Iterator`, it chains directly with `.enumerate()`, `.filter()`, `.map()`, `.all()`, `.any()` — no intermediate collections needed until `.collect()` at the very end.

4. **Bounds safety:** Index arithmetic (`data[i-1]`, `data[i]`, `data[i+1]`) requires manual bounds checking and is a common source of off-by-one bugs. `windows(3)` guarantees every sub-slice has exactly 3 elements, so `w[0]`, `w[1]`, `w[2]` are always valid.

5. **Window count formula:** For a slice of length `L` and window size `n`, both languages produce exactly `L - n + 1` windows (or zero if `n > L`). The formula is identical; only the mechanism differs.

## When to Use Each Style

**Use `windows()` iterator chain when:** you need any sliding-window computation — moving averages, monotonicity checks, local extrema, n-gram extraction, or consecutive-pair comparisons. It is the idiomatic, zero-cost abstraction in Rust.

**Use recursive Rust when:** you are explicitly demonstrating the structural recursion that mirrors OCaml's pattern-matching style, or when the window logic itself is recursive in nature (e.g., building a tree from overlapping segments).
