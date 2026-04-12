📖 **[View on hightechmind.io →](https://hightechmind.io/rust/283-exact-size-iterator)**

---

# 283: ExactSizeIterator for Known-Length Iterators
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Pre-allocating the right amount of memory before collecting an iterator avoids repeated resizing. Displaying progress bars requires knowing total count upfront. Splitting an iterator into equally-sized chunks requires knowing its length. The `ExactSizeIterator` trait signals that an iterator knows its exact remaining length in O(1) time, enabling these optimizations without counting all elements first.

## Learning Outcomes

- Understand `ExactSizeIterator` as providing O(1) `len()` for iterators with known size
- Implement `ExactSizeIterator` on a custom iterator struct with a computable length
- Use `len()` for pre-allocation and progress display without consuming the iterator
- Recognize which standard iterators implement `ExactSizeIterator`: slice iterators, ranges, `Vec` drains, but not filtered or chained iterators

## Rust Application

Implementing `ExactSizeIterator` requires `Iterator` and provides `len()`. `Vec::collect()` uses `size_hint()` for pre-allocation — `ExactSizeIterator` makes this exact:

```rust
pub struct FixedRange { current: usize, end: usize }

impl Iterator for FixedRange {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        if self.current >= self.end { return None; }
        let v = self.current;
        self.current += 1;
        Some(v)
    }
}

impl ExactSizeIterator for FixedRange {
    fn len(&self) -> usize { self.end - self.current }
}

let r = FixedRange::new(0, 10);
println!("Will collect {} items", r.len()); // 10, without consuming
let v: Vec<_> = r.collect(); // pre-allocates exactly 10 slots
```

## OCaml Approach

OCaml's `Seq` module is inherently forward-only without size information. Length is known only for `Array` and `List` (via `Array.length` and `List.length`). Sequences built from generators have no known length unless explicitly tracked alongside:

```ocaml
(* OCaml has no ExactSizeIterator equivalent — length must be tracked separately *)
let (length, seq) = (10, Seq.init 10 Fun.id)
```

## Key Differences

1. **Trait-based annotation**: Rust uses a marker trait to communicate "I know my size"; OCaml has no equivalent standard mechanism.
2. **Pre-allocation optimization**: `collect::<Vec<_>>()` uses `ExactSizeIterator` to pre-allocate the exact vector capacity; OCaml's `Array.of_seq` must grow or traverse twice.
3. **Composition loss**: Applying `filter()` or `flat_map()` to an `ExactSizeIterator` loses the `ExactSizeIterator` implementation — only size-preserving adapters like `map()` retain it.
4. **Progress bars**: Libraries like `indicatif` use `ExactSizeIterator::len()` to display accurate progress without pre-consuming the iterator.

## Exercises

1. Implement `ExactSizeIterator` for a custom `StridedRange` that yields every nth element from a range, where the exact count is computable as `(end - start + step - 1) / step`.
2. Write a function that takes an `ExactSizeIterator` and a progress-display closure, calling the closure with `(index, total)` for each element.
3. Show that `filter()` applied to an `ExactSizeIterator` loses the `ExactSizeIterator` implementation by checking the trait bounds in the type signature.
