# OCaml vs Rust: ExactSizeIterator

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: List.length is always O(n) — no compile-time size guarantee *)
let process_with_progress lst =
  let total = List.length lst in   (* traverses the entire list *)
  List.mapi (fun i x ->
    Printf.sprintf "[%d/%d] Processing %d" (i + 1) total x
  ) lst

(* Arrays have O(1) length, but you can't query length mid-pipeline *)
let chunks_exact n arr =
  let len = Array.length arr in
  let num_chunks = len / n in
  Array.init num_chunks (fun i -> Array.sub arr (i * n) n)
```

### Rust (idiomatic — ExactSizeIterator)
```rust
// Slices, Vec::iter(), ranges, .map(), .enumerate(), .zip() all implement
// ExactSizeIterator — .len() is O(1) and guaranteed exact.
pub fn process_with_progress(data: &[i32]) -> Vec<String> {
    let total = data.len(); // O(1) — ExactSizeIterator guarantee
    data.iter()
        .enumerate()
        .map(|(i, &x)| format!("[{}/{}] Processing {}", i + 1, total, x))
        .collect()
}

// Pre-allocate exactly — zero reallocations because size is known upfront
pub fn map_preallocated<T, U>(data: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    let mut result = Vec::with_capacity(data.len());
    for item in data { result.push(f(item)); }
    result
}
```

### Rust (functional/recursive — custom ExactSizeIterator)
```rust
pub struct Countdown { remaining: usize }

impl Iterator for Countdown {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.remaining == 0 { return None; }
        self.remaining -= 1;
        Some(self.remaining)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

// Opt into ExactSizeIterator — the compiler verifies size_hint is exact
impl ExactSizeIterator for Countdown {}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| List length | `List.length : 'a list -> int` (O(n)) | `<[T]>::len(&self) -> usize` (O(1)) |
| Exact size trait | none — no trait for this | `ExactSizeIterator` |
| Size hint | n/a (lists are linked) | `Iterator::size_hint() -> (usize, Option<usize>)` |
| Array length | `Array.length arr` (O(1)) | `data.len()` via `ExactSizeIterator` |
| Chunks without remainder | `Array.sub` + manual math | `slice::chunks_exact(n)` |

## Key Insights

1. **O(n) vs O(1) length:** OCaml linked lists must traverse the entire structure to count elements. Rust slices, `Vec`, and ranges store their length, making `.len()` O(1) and safe to call in hot loops.

2. **Compile-time size contract:** `ExactSizeIterator` is a trait — implementing it is an explicit promise that `size_hint()` returns exact bounds. The compiler enforces the contract at every call site; breaking it is `unsafe` territory.

3. **Adapter propagation:** Standard adapters like `.map()`, `.enumerate()`, `.zip()`, and `.take(n)` automatically implement `ExactSizeIterator` when both inputs do. Adapters like `.filter()` cannot, because they conditionally drop elements — the size is unknown until the predicate runs.

4. **Pre-allocation without guessing:** `Vec::with_capacity(iter.len())` allocates exactly once when the iterator is `ExactSizeIterator`. Without it, `collect()` falls back to `size_hint()` lower bound and may reallocate several times.

5. **Progress bars and batch processing:** Knowing the exact remaining count enables correct progress fractions (`completed / total`) without consuming the iterator. OCaml achieves this only with arrays or by pre-computing `List.length` (an O(n) pass) before iteration begins.

## When to Use Each Style

**Use `ExactSizeIterator::len()`** when building progress UIs, pre-allocating output buffers, or splitting work into batches — any time knowing "how many remain" is cheaper than a full traversal.

**Use `size_hint()` only** when working with adapters like `.filter()` that can't guarantee exact counts; treat it as an optimization hint, not a hard contract.

**Implement `ExactSizeIterator` on custom types** whenever your iterator wraps a fixed-size collection or counter — it's a zero-cost trait that unlocks better downstream ergonomics and avoids unnecessary reallocations for callers.
