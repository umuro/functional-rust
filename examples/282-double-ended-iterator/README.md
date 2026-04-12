📖 **[View on hightechmind.io →](https://hightechmind.io/rust/282-double-ended-iterator)**

---

# 282: DoubleEndedIterator and rev()
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Some traversal algorithms need to process elements from both ends of a sequence — reversing a sequence, checking palindromes, implementing a two-pointer algorithm, or finding the last matching element efficiently. `DoubleEndedIterator` extends the `Iterator` trait with a `next_back()` method, enabling traversal from the back end without reversing the sequence in memory. The `rev()` adapter wraps a `DoubleEndedIterator` to produce a forward iterator that yields elements from back to front.

## Learning Outcomes

- Understand `DoubleEndedIterator` as adding `next_back()` to enable consumption from both ends
- Use `rev()` to iterate a bidirectional collection in reverse without allocating a reversed copy
- Implement `DoubleEndedIterator` on a custom struct with independent front and back pointers
- Recognize which standard iterators implement `DoubleEndedIterator`: slice iterators, `Range`, `Vec`, but not `Chain` by default

## Rust Application

A custom `Counter` iterator implements both `Iterator` and `DoubleEndedIterator` using `front` and `back` index fields. When `front > back`, both ends have met and iteration terminates:

```rust
impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.front > self.back { return None; }
        let val = self.front;
        self.front += 1;
        Some(val)
    }
}
impl DoubleEndedIterator for Counter {
    fn next_back(&mut self) -> Option<i32> {
        if self.front > self.back { return None; }
        let val = self.back;
        self.back -= 1;
        Some(val)
    }
}
// c.rev() now works — no allocation
```

## OCaml Approach

OCaml's `Seq` module is inherently forward-only. Bidirectional iteration typically requires converting to an array (`Array.of_seq`) and using index-based access from both ends, or using a doubly-linked data structure:

```ocaml
let reverse_seq seq =
  let arr = Array.of_seq seq in
  Array.to_seq (Array.init (Array.length arr)
    (fun i -> arr.(Array.length arr - 1 - i)))
(* Allocates: no zero-copy reverse *)
```

## Key Differences

1. **Zero-copy reverse**: Rust's `rev()` on a `DoubleEndedIterator` is zero-allocation; OCaml's sequence reversal requires materializing into an array.
2. **Two-pointer idiom**: Rust's `DoubleEndedIterator` enables the classic two-pointer algorithm without materializing the full sequence.
3. **Adapter compatibility**: `rev()` composes with `map`, `filter`, and other adapters; OCaml requires restructuring the algorithm.
4. **Standard collections**: `Vec`, slices, `BTreeMap`, `LinkedList` implement `DoubleEndedIterator` in Rust; hash maps and forward-only structures do not.

## Exercises

1. Use `rev()` to compare a `Vec<char>` with its reverse, implementing a palindrome checker without allocating a reversed copy.
2. Implement `DoubleEndedIterator` on a custom `GridRow` type that iterates over a 2D matrix row from either end.
3. Use the two-pointer pattern: simultaneously consume elements from both ends of a `DoubleEndedIterator` to check if all pairs sum to the same value.
