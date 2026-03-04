# 054: List Map from Scratch

**Difficulty:** 1  **Level:** Beginner

Implement `map` from first principles — and discover it's a special case of `fold`.

## The Problem This Solves

You have a list of numbers and you want a list of their squares. A list of filenames and you want a list of file sizes. A list of users and you want a list of their emails. The operation is always the same: walk the list, apply a transformation to each element, collect the results.

Without `map`, you write a `for` loop, a mutable accumulator, and the same boilerplate for every transformation. The loop structure obscures the intent. With `map`, you declare the transformation and `map` handles the iteration — the loop is gone, only the logic remains.

The deeper lesson: `add1`, `to_string`, and `double` all have the same shape. The **Abstraction Principle** says when two functions share a shape, factor out the difference into a parameter. `map` is that factoring — it takes the transformation as a parameter.

## The Intuition

`map` is a conveyor belt. Each item on the belt goes through a machine (your function `f`) and comes out the other side transformed. The belt is the same; only the machine changes.

In OCaml, `map` recurses over a cons-list: empty list produces empty list, head gets transformed and prepended to the recursively mapped tail. In Rust, the iterator does the walking; you just describe the transformation.

## How It Works in Rust

```rust
// Idiomatic: iterator adapter
pub fn map<A, B, F: Fn(&A) -> B>(list: &[A], f: F) -> Vec<B> {
    list.iter().map(f).collect()
}

// Recursive: mirrors OCaml's head-cons pattern
pub fn map_recursive<A, B, F: Fn(&A) -> B>(list: &[A], f: F) -> Vec<B> {
    match list {
        [] => vec![],
        [head, tail @ ..] => {  // [head, tail @ ..] ≈ OCaml's h :: t
            let mut result = vec![f(head)];
            result.extend(map_recursive(tail, f));
            result
        }
    }
}

// Fold: map as a fold over an accumulator
pub fn map_fold<A, B, F: Fn(&A) -> B>(list: &[A], f: F) -> Vec<B> {
    list.iter().fold(Vec::new(), |mut acc, x| {
        acc.push(f(x));  // fold accumulates transformed elements
        acc
    })
}
```

All three produce the same result. The fold version reveals that `map` is mechanically a `fold` — fold is the universal list combinator, and `map` is one of its specialisations.

## What This Unlocks

- **Any element-wise transformation** — change types, compute derived values, format for display.
- **Understanding fold's universality** — `map` and `filter` are both derivable from `fold`; this example shows the first half.
- **Slice patterns** — `[head, tail @ ..]` is the Rust idiom for recursive list processing, directly mirroring OCaml's `h :: t`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type signature | `('a -> 'b) -> 'a list -> 'b list` | `fn<A, B, F: Fn(&A)->B>(list: &[A], f: F) -> Vec<B>` |
| Head-tail pattern | `h :: t` | `[head, tail @ ..]` |
| Partial application | `let add1 = map (fun x -> x+1)` | Explicit closure or wrapper function |
| Allocation | GC manages cons cells | `collect()` allocates one `Vec` |
| Iterator version | `List.map` in stdlib | `Iterator::map` (lazy, allocates on `collect`) |
