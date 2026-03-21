# Example 1186: List.filter — Select Elements by Predicate
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Filter a list to keep only elements that satisfy a given predicate function. Given a list of integers, produce separate lists of even and odd numbers.

## Learning Outcomes

- How Rust's `.filter()` iterator adapter mirrors OCaml's `List.filter`
- The distinction between borrowing slices (`&[T]`) and consuming owned `Vec<T>`
- Using lifetime annotations when returning references from recursive functions
- Why Rust closures passed as `&F` require explicit lifetime parameters on the return type

## OCaml Approach

OCaml's `List.filter` takes a predicate `'a -> bool` and a list, returning a new list. It is a pure, polymorphic higher-order function from the standard library. The predicate is applied to each element; matching elements are collected into a new list without mutation.

## Rust Approach

Rust's `Iterator::filter` mirrors this exactly: a closure `Fn(&T) -> bool` is applied to each element of an iterator. The idiomatic version uses `items.iter().filter(pred).collect()`. For owned data, `into_iter()` consumes the original vector and yields owned values, avoiding any cloning.

## Key Differences

1. **Ownership model:** OCaml lists are immutable linked lists with structural sharing; Rust slices are contiguous borrowed views, and `Vec<T>` is heap-allocated owned data.
2. **Return type:** OCaml `List.filter` always returns a new list; Rust returns `Vec<&T>` (borrowed) or `Vec<T>` (owned) depending on the iterator source.
3. **Lifetime annotation:** When a recursive Rust function returns references derived from its input, lifetimes must be explicit (`'a`) to tell the compiler where the references come from.
4. **Predicate passing:** OCaml closures are first-class values passed directly; Rust closures are typed generics (`F: Fn(&T) -> bool`), zero-cost at runtime via monomorphization.
