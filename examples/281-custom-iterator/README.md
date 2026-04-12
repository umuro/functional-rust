📖 **[View on hightechmind.io →](https://hightechmind.io/rust/281-custom-iterator)**

---

# 281: Implementing the Iterator Trait from Scratch
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Every iterator in Rust's standard library is built from the same foundation: a struct with state and a single `next()` method. Understanding this foundation is essential for building domain-specific sequences — streaming database rows, generating mathematical sequences, traversing custom data structures, or implementing infinite value generators. The entire iterator adapter ecosystem (`map`, `filter`, `zip`, etc.) becomes available for free once `next()` is implemented.

## Learning Outcomes

- Understand that implementing `Iterator` requires only defining `type Item` and `fn next(&mut self) -> Option<Self::Item>`
- Recognize that all other iterator adapters (`map`, `filter`, `sum`, etc.) are provided for free by the trait
- Build stateful iterators using struct fields to track iteration progress
- Implement custom termination logic by returning `None` from `next()`

## Rust Application

The `Iterator` trait has one required method. The `Squares` struct demonstrates state-carrying iteration:

```rust
pub struct Squares { current: u32, max: u32 }

impl Squares {
    pub fn new(max: u32) -> Self { Squares { current: 0, max } }
}

impl Iterator for Squares {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.current >= self.max { return None; }
        let val = self.current * self.current;
        self.current += 1;
        Some(val)
    }
}

// All iterator adapters work automatically
let sum: u32 = Squares::new(5).filter(|&x| x > 0).sum();
```

## OCaml Approach

OCaml does not have a single iterator trait. The closest equivalent is the `Seq` module's lazy sequence type, which is a function `unit -> 'a node` where `node` is `Nil | Cons of 'a * 'a Seq.t`. A custom generator is a closure returning the next `Cons` or `Nil`:

```ocaml
let squares max =
  let rec go n () =
    if n >= max then Seq.Nil
    else Seq.Cons (n * n, go (n + 1))
  in go 0
(* All Seq combinators (map, filter, fold_left) work on this *)
```

## Key Differences

1. **Interface size**: Rust's `Iterator` requires one method; OCaml's `Seq` is just a function type — both are minimal.
2. **Free methods**: Rust's `Iterator` provides ~70 free methods once `next()` is implemented; OCaml's `Seq` module provides a smaller set.
3. **State representation**: Rust stores state in a struct with named fields; OCaml uses closure-captured variables.
4. **Type safety**: Rust's `type Item` makes the element type explicit in the trait; OCaml's `'a Seq.t` is polymorphic.

## Exercises

1. Implement a `Fibonacci` iterator struct that yields Fibonacci numbers indefinitely using two accumulator fields.
2. Implement a `Range` iterator struct that yields integers from `start` to `end` with a configurable step size.
3. Implement a `TakeWhile` adapter struct that wraps another iterator and stops when a predicate returns false — implement `Iterator` on it manually.
