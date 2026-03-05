# OCaml vs Rust: Step By, Enumerate, Rev

## Side-by-Side Code

### OCaml
```ocaml
(* step_by: filter by index modulo *)
let step_by n lst = List.filteri (fun i _ -> i mod n = 0) lst

(* enumerate: pair with index *)
let enumerate lst = List.mapi (fun i x -> (i, x)) lst

(* rev_map: map over reversed list *)
let rev_map f lst = List.map f (List.rev lst)
```

### Rust (idiomatic — zero-cost adapters)
```rust
fn every_nth(data: &[i32], n: usize) -> Vec<i32> {
    data.iter().step_by(n).copied().collect()
}

fn indexed_filter(data: &[i32], pred: impl Fn(&i32) -> bool) -> Vec<(usize, i32)> {
    data.iter().enumerate().filter(|(_, x)| pred(x)).map(|(i, &x)| (i, x)).collect()
}

fn rev_map(data: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32> {
    data.iter().rev().map(|&x| f(x)).collect()
}
```

### Rust (functional/explicit — range_step recursive analog)
```rust
fn range_step(start: i32, stop: i32, step: usize) -> Vec<i32> {
    (start..stop).step_by(step).collect()
}

fn enumerate_reversed(data: &[i32]) -> Vec<(usize, i32)> {
    data.iter().rev().enumerate().map(|(i, &x)| (i, x)).collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Step by n | `val step_by : int -> 'a list -> 'a list` | `fn every_nth(data: &[i32], n: usize) -> Vec<i32>` |
| Enumerate | `val enumerate : 'a list -> (int * 'a) list` | `Iterator::enumerate() -> (usize, &T)` |
| Reverse map | `val rev_map : ('a -> 'b) -> 'a list -> 'b list` | `fn rev_map(data: &[i32], f: impl Fn(i32) -> i32) -> Vec<i32>` |
| Index type | `int` (signed, 63-bit) | `usize` (unsigned, pointer-sized) |
| Collection type | `'a list` (linked list) | `&[T]` (contiguous slice) |

## Key Insights

1. **Zero-cost vs linear scan**: OCaml's `List.filteri` with `i mod n = 0` visits every element; Rust's `step_by` internally skips `n-1` elements between yields, making the intent explicit and the implementation potentially more efficient on slices.

2. **Lazy vs eager evaluation**: Rust's iterator adapters are lazy — `step_by`, `enumerate`, and `rev` produce no allocation and do no work until `.collect()` or consumption. OCaml's list operations are strict and produce intermediate lists at each step.

3. **`DoubleEndedIterator` as a capability**: Rust's `rev()` requires the underlying iterator to implement `DoubleEndedIterator`. Slice iterators satisfy this because slices are contiguous memory with known endpoints. OCaml's singly-linked lists can only be traversed forwards; `List.rev` allocates a new list.

4. **Composition is type-safe**: `step_by(2).rev()` compiles only when the step iterator still implements `DoubleEndedIterator`. The compiler enforces composability. In OCaml, any two list functions compose freely but correctness is the programmer's responsibility.

5. **`usize` vs `int`**: Rust uses `usize` for indices — it cannot be negative, matching the reality that list positions are natural numbers. OCaml uses plain `int`, so negative indices are possible at the type level (and `List.filteri` would simply never match them).

## When to Use Each Style

**Use idiomatic Rust (`.step_by()`, `.enumerate()`, `.rev()`)** when working with standard slices, ranges, or any iterator — these adapters are the zero-cost default and compose cleanly with the rest of the iterator API.

**Use the recursive/explicit style** when you need custom step logic tied to values rather than positions (e.g., "advance until condition"), or when implementing a non-standard traversal over a recursive data structure where the iterator protocol doesn't apply.
