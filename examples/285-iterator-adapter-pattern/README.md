📖 **[View on hightechmind.io →](https://hightechmind.io/rust/285-iterator-adapter-pattern)**

---

# 285: Building Custom Iterator Adapters
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The standard library's `map`, `filter`, and `zip` adapters cover many cases, but domain-specific transformations often need their own adapters — a rate limiter that throttles output, a deduplicator that removes consecutive duplicates, or a strider that yields every nth element. Building custom adapters in Rust follows the same pattern as the standard library: wrap an inner iterator in a struct and implement `Iterator` on it, making the adapter composable with the entire ecosystem.

## Learning Outcomes

- Understand the adapter pattern: wrap `I: Iterator` in a struct, implement `Iterator` on the wrapper
- Build a `EveryNth` adapter that yields every nth element from any iterator
- Recognize that custom adapters gain the full standard library API for free
- Use generic struct parameters `<I: Iterator>` to make adapters work with any iterator source

## Rust Application

The `EveryNth<I>` struct wraps an inner iterator and tracks a modular counter. Each call to `next()` loops until it finds an element to yield:

```rust
pub struct EveryNth<I> { inner: I, n: usize, count: usize }

impl<I: Iterator> Iterator for EveryNth<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        loop {
            let item = self.inner.next()?;
            let emit = self.count % self.n == 0;
            self.count += 1;
            if emit { return Some(item); }
        }
    }
}
```

This pattern is identical to how `std::iter::Filter`, `std::iter::Map`, etc. are implemented.

## OCaml Approach

OCaml's `Seq` module allows custom adapters as functions returning new sequences. Since `Seq.t` is just `unit -> node`, a custom adapter is simply a function that wraps the original sequence:

```ocaml
let every_nth n seq =
  let rec go i s () = match s () with
    | Seq.Nil -> Seq.Nil
    | Seq.Cons (x, rest) ->
      if i mod n = 0 then Seq.Cons (x, go (i+1) rest)
      else go (i+1) rest ()
  in go 0 seq
```

Both approaches create composable, lazy transformations.

## Key Differences

1. **State storage**: Rust adapters store state in struct fields; OCaml adapters capture state in closure variables.
2. **Type system integration**: Rust adapters implement the `Iterator` trait and gain all standard adapters; OCaml functions on `Seq.t` gain all `Seq` module functions.
3. **Zero-cost**: Rust's struct-based adapters are monomorphized and inlined by the compiler; OCaml's functional adapters use closures with potential allocation.
4. **Ecosystem integration**: Custom Rust adapters work with `Rayon` parallel iterators if they also implement the right parallel traits.

## Exercises

1. Build a `Deduplicate<I>` adapter that yields consecutive elements only when they differ from the previous one (run-length deduplication).
2. Build a `Buffered<I>` adapter that collects elements into batches of size N and yields `Vec<T>` per batch.
3. Build a `TimeoutIterator<I>` adapter that stops yielding elements after a `Duration` has elapsed (simulated with an iteration count).
