# OCaml vs Rust: Fixed-Size Chunks Iteration

## Side-by-Side Code

### OCaml
```ocaml
let chunks n lst =
  let rec aux acc current count = function
    | [] ->
      if current = [] then List.rev acc
      else List.rev (List.rev current :: acc)
    | x :: xs ->
      if count = n then aux (List.rev current :: acc) [x] 1 xs
      else aux acc (x :: current) (count + 1) xs
  in
  aux [] [] 0 lst
```

### Rust (idiomatic)
```rust
pub fn chunk_sums(data: &[i32], n: usize) -> Vec<i32> {
    data.chunks(n).map(|c| c.iter().sum()).collect()
}

pub fn full_chunks<T: Clone>(data: &[T], n: usize) -> Vec<Vec<T>> {
    data.chunks_exact(n).map(<[T]>::to_vec).collect()
}
```

### Rust (functional/recursive — OCaml-style)
```rust
pub fn chunks_recursive<T: Clone>(data: &[T], n: usize) -> Vec<Vec<T>> {
    if data.is_empty() || n == 0 {
        return vec![];
    }
    let (head, tail) = data.split_at(n.min(data.len()));
    let mut result = vec![head.to_vec()];
    result.extend(chunks_recursive(tail, n));
    result
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Chunk function | `val chunks : int -> 'a list -> 'a list list` | `fn chunks(n: usize) -> ChunksIter<T>` (slice method) |
| Element collection | `'a list` | `&[T]` (slice) |
| Result | `'a list list` | `impl Iterator<Item = &[T]>` |
| Partial last chunk | handled via `[]` base case | automatic — last chunk may be shorter |
| Exact chunks only | filter manually | `chunks_exact(n)` + `.remainder()` |

## Key Insights

1. **Zero allocation vs. recursive building:** Rust's `chunks()` is a zero-copy iterator that yields sub-slices (`&[T]`) directly from the original data. OCaml must build new lists via accumulator recursion, allocating on every step.

2. **`chunks` vs `chunks_exact`:** Rust provides two variants — `chunks(n)` includes the final short chunk if `len % n != 0`, while `chunks_exact(n)` skips it and exposes the leftover via `.remainder()`. OCaml's manual implementation must handle the short tail in the base case.

3. **Ownership and borrowing:** Because `chunks()` returns sub-slices (`&[T]`), no data is copied. Turning them into owned `Vec<T>` requires an explicit `.to_vec()` call — making the allocation visible and optional.

4. **Iterator composability:** The returned iterator composes freely with `.map()`, `.filter()`, `.enumerate()` etc., enabling batch-processing pipelines without intermediate allocations. OCaml would need `List.map` over the constructed list.

5. **Stack safety:** The recursive Rust version mirrors OCaml's accumulator pattern but is not tail-recursive in safe Rust — for large inputs the idiomatic iterator form is always preferred.

## When to Use Each Style

**Use idiomatic Rust (`chunks` / `chunks_exact`) when:** Processing real data in batches — database writes, image row rendering, pagination — where zero-copy sub-slice access and iterator composability matter.

**Use recursive Rust when:** Teaching the OCaml-to-Rust translation, or when you need to understand the underlying algorithm without relying on stdlib methods.
