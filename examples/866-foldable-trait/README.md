📖 **[View on hightechmind.io →](https://hightechmind.io/rust/866-foldable-trait)**

---

# Foldable Trait
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Folding — reducing a collection to a single value by repeatedly applying a combining function — is the most general way to consume a data structure. `sum`, `product`, `max`, `min`, `to_vec`, `count`, `any`, `all`, `find` are all specific instances of fold. A `Foldable` trait abstracts this: any type that supports `fold_left` and `fold_right` can be reduced to any type. This powers generic algorithms that work over trees, lists, and custom containers without knowing the specific structure. OCaml's `List.fold_left` and Haskell's `Foldable` typeclass are the canonical implementations. Rust's `Iterator::fold` is the iterator-based equivalent.

## Learning Outcomes

- Define the `Foldable` trait with `fold_left` and `fold_right` methods
- Implement for `List<T>` (linked list) and `Tree<T>` (binary tree)
- Derive useful operations from fold: `sum`, `length`, `to_vec`, `max`, `min`, `any`, `all`
- Understand left vs. right fold: different associativity, different stack behavior
- Recognize that `fold_right` is naturally recursive; `fold_left` is tail-recursive for lists

## Rust Application

```rust
pub trait Foldable {
    type Item;
    fn fold_left<B, F: FnMut(B, &Self::Item) -> B>(&self, init: B, f: F) -> B;
    fn fold_right<B, F: FnMut(&Self::Item, B) -> B>(&self, init: B, f: F) -> B;
}
// Derived operations from fold_left:
pub fn sum<C: Foldable<Item=i32>>(c: &C) -> i32 {
    c.fold_left(0, |acc, x| acc + x)
}
pub fn to_vec<C: Foldable>(c: &C) -> Vec<&C::Item> {
    c.fold_left(vec![], |mut acc, x| { acc.push(x); acc })
}
```

The `Foldable` trait uses `type Item` as an associated type. `fold_left` takes `FnMut` because the accumulator is passed by value and may need mutable state (e.g., building a Vec). The trait is implemented for `List<T>` and `Tree<T>` — each structure knows its own traversal order. Derived operations are generic over any `Foldable` implementor, enabling code reuse across all data structures that support folding.

## OCaml Approach

OCaml's `Foldable` as a module signature: `module type FOLDABLE = sig type 'a t; val fold_left : ('b -> 'a -> 'b) -> 'b -> 'a t -> 'b; val fold_right : ('a -> 'b -> 'b) -> 'a t -> 'b -> 'b end`. Derived operations: `let sum (module F: FOLDABLE with type 'a t = int t) = F.fold_left (+) 0`. OCaml's `List.fold_left` and `Array.fold_left` implement this for their respective types. The `Tree` implementation uses pattern matching: `Leaf -> acc`, `Node(l, v, r) -> fold_right f r (f (fold_right f l acc) v)`.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Trait/signature | `trait Foldable` | `module type FOLDABLE` |
| Associated type | `type Item` | `type 'a t` parameterized |
| Generic over F | `<C: Foldable<Item=i32>>` | First-class module `(module FOLDABLE)` |
| List fold order | `fold_left` is left-to-right | Same |
| Tree fold order | Implementation-defined | Same |
| Stack safety | `fold_right` recurses | OCaml TCO for tail-recursive folds |

## Exercises

1. Implement `Foldable` for a custom binary tree and derive `sum`, `length`, `max`, and `in_order_list` from the fold.
2. Implement `fold_right` for `List<T>` and verify that `fold_right(cons, [], xs) == xs.clone()`.
3. Show that `to_vec` using `fold_left` and `fold_right` produce reversed lists — use this to implement `reverse`.
4. Implement `any` and `all` as early-exit folds using `try_fold` on Rust's Iterator trait.
5. Measure the stack depth of `fold_right` on a list of 100,000 elements and compare with the stack-safe `fold_left` version.
