[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 087 — Iterator Adapters

## Problem Statement

Implement custom iterator adapters — `MyMap`, `MyFilter`, and `MyTake` — by wrapping an inner iterator and implementing `next`. Bundle them into an extension trait `MyIterExt` for fluent chaining. Compare with OCaml's `Seq`-based `my_map`, `my_filter`, and `my_take` functions.

## Learning Outcomes

- Model an iterator adapter as a struct holding an inner iterator and a function
- Use `FnMut` for adapter closures that may need mutable state (e.g. counting)
- Implement `next` for adapters by delegating to the inner iterator
- Create an extension trait to add adapter methods to all iterators
- Understand why adapter structs are generic over `I: Iterator` and `F: FnMut`
- Map Rust's adapter structs to OCaml's higher-order `Seq`-based functions

## Rust Application

`MyMap<I, F>` holds `iter: I` and `f: F`. Its `next` delegates to `self.iter.next().map(&mut self.f)`. `MyFilter<I, P>` loops with `while let Some(item) = self.iter.next()` until the predicate passes. `MyTake<I>` decrements `remaining` on each call and returns `None` when exhausted. The `MyIterExt` extension trait adds `my_map`, `my_filter`, and `my_take` as methods on any `Iterator + Sized`, enabling `iter.my_map(f).my_filter(p).my_take(n)` fluent chains.

## OCaml Approach

OCaml's `Seq` adapters are plain functions: `my_map f s` returns a thunk `fun () -> match s () with Seq.Nil -> … | Seq.Cons(x, rest) -> Seq.Cons(f x, my_map f rest)`. `my_filter` recurses to skip non-matching elements. `my_take` decrements a counter. There is no extension trait; composition is achieved with the `|>` pipe operator. Both approaches achieve the same lazy pipeline; Rust's adapter structs make the type of each step explicit.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Adapter type | Generic struct `MyMap<I, F>` | Higher-order function `'a Seq.t -> 'a Seq.t` |
| Closure mutability | `FnMut` allowed | Mutable via `ref` if needed |
| Extension | `trait MyIterExt` | `|>` pipe operator |
| Filter inner loop | `while let` | Tail-recursive `my_filter p rest ()` |
| Composability | Fluent method chain | `|>` composition |
| Laziness mechanism | Pull-based `next` | Thunk evaluation |

Building custom adapters from scratch reveals the mechanics behind `std::iter`. In production, use the standard adapters — they are optimised and tested. But understanding the internals is essential for designing custom data sources and specialised iteration patterns.

## Exercises

1. Implement a `MyZip<A: Iterator, B: Iterator>` adapter that yields pairs `(A::Item, B::Item)` and stops when either iterator is exhausted.
2. Write a `MyFlatMap<I, F, J>` adapter where `F: FnMut(I::Item) -> J` and `J: Iterator`.
3. Add a `my_enumerate` method to `MyIterExt` that wraps items as `(usize, Item)`.
4. Implement `MyChain<A: Iterator<Item=T>, B: Iterator<Item=T>>` that yields all of `A` then all of `B`.
5. In OCaml, write a `seq_flat_map : ('a -> 'b Seq.t) -> 'a Seq.t -> 'b Seq.t` and test it on a sequence of string words split into characters.
