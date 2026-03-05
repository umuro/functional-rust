📖 **[View on hightechmind.io →](https://hightechmind.io/rust/285-iterator-adapter-pattern)**

---

# 285: Building Custom Iterator Adapters

**Difficulty:** 3  **Level:** Advanced

Wrap an existing iterator in a struct to transform its output — the same pattern used by `map`, `filter`, and `zip` in the standard library.

## The Problem This Solves

You've written the same `.filter(...).map(...).step_by(...)` chain in five places in your codebase. Or you need a transformation that doesn't exist in the standard library — emitting adjacent pairs, sampling every Nth element, annotating elements with their distance from a reference point. You could write a function that returns a `Vec`, but that forces eager evaluation and allocates even when your caller might only need the first element.

The iterator adapter pattern solves this: wrap your transformation in a struct that holds the inner iterator, then implement `Iterator` on it. Your adapter is now lazy, composable, and chains seamlessly with every other iterator adapter. This is exactly how `std::iter::Map`, `std::iter::Filter`, and `std::iter::Zip` are implemented in the standard library.

The extension trait pattern (adding your adapter as a method on all iterators via a trait) makes the API ergonomic — callers write `.every_nth(3)` instead of `EveryNth::new(iter, 3)`.

## The Intuition

An iterator adapter is a struct `MyAdapter<I>` that wraps an inner iterator `I: Iterator` and transforms what `next()` returns. Implement `Iterator for MyAdapter<I>` — call `self.inner.next()` internally, apply your transformation, and return the result.

```rust
struct MyAdapter<I> { inner: I }
impl<I: Iterator> Iterator for MyAdapter<I> {
    type Item = I::Item;  // (or a different type)
    fn next(&mut self) -> Option<I::Item> {
        self.inner.next().map(|x| transform(x))
    }
}
```

## How It Works in Rust

```rust
// Adapter 1: yield every Nth element
struct EveryNth<I> { inner: I, n: usize, count: usize }

impl<I: Iterator> Iterator for EveryNth<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        loop {
            let item = self.inner.next()?;  // ? propagates None (exhausted)
            let emit = self.count % self.n == 0;
            self.count += 1;
            if emit { return Some(item); }
            // otherwise loop: skip this element, try the next
        }
    }
}

// Adapter 2: yield adjacent pairs (sliding window of 2)
struct Pairs<I: Iterator> { inner: I, prev: Option<I::Item> }

impl<I: Iterator> Iterator for Pairs<I> where I::Item: Clone {
    type Item = (I::Item, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        let prev = self.prev.replace(next.clone())?;  // swap prev with next
        Some((prev, next))
    }
}

// Extension trait: add adapters as methods on all iterators
trait IteratorExt: Iterator + Sized {
    fn every_nth(self, n: usize) -> EveryNth<Self> {
        EveryNth { inner: self, n, count: 0 }
    }
    fn pairs(self) -> Pairs<Self> where Self::Item: Clone {
        let mut inner = self;
        let prev = inner.next();  // prime with first element
        Pairs { inner, prev }
    }
}
impl<I: Iterator> IteratorExt for I {}  // blanket impl for all iterators

// Usage: chain custom adapters with standard ones
let thirds: Vec<i32> = (0..12).every_nth(3).collect();
// → [0, 3, 6, 9]

let pairs: Vec<(i32, i32)> = [10, 20, 30, 40, 50].iter().copied().pairs().collect();
// → [(10,20), (20,30), (30,40), (40,50)]

// Adapters compose with each other and with standard adapters
let result: Vec<(i32, i32)> = (0i32..20).every_nth(2).pairs().collect();
```

## What This Unlocks

- **Reusable lazy transformations** — package any multi-step adapter chain into a named, composable unit.
- **Library-quality API** — expose your domain's natural iteration patterns as chainable methods on all iterators.
- **Zero-allocation pipelines** — adapters are structs on the stack; no `Vec` is created until `.collect()`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Custom adapter | Higher-order function over `Seq` | Struct + `impl Iterator` |
| Extension method | Module or `|>` pipeline | Extension trait with blanket impl |
| Lazy by default | `Seq` is lazy, `List` is not | All iterators are lazy |
| Composability | `Seq` functions compose via `|>` | All adapters chain via method calls |
| Type | `'a Seq.t -> 'b Seq.t` | `AdapterStruct<I>` where `I: Iterator` |
