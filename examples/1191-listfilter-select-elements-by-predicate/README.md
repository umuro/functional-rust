# Example 1191: List.filter — Select Elements by Predicate
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Given a list and a predicate function, return a new list containing only the elements for which the predicate returns true. Demonstrated with even/odd partitioning on `[1..8]`.

## Learning Outcomes

- How OCaml's `List.filter` maps directly to Rust's iterator `.filter()` combinator
- Using closures as higher-order function arguments in Rust
- The `partition` method as an ergonomic alternative when splitting into two groups
- Why recursive solutions require `predicate: &F` instead of `predicate: F` to avoid infinite type instantiation

## OCaml Approach

OCaml's `List.filter` takes a predicate `'a -> bool` and a list, returning a new list. The function is in the standard library and works naturally with anonymous functions (`fun x -> x mod 2 = 0`). The list is a linked list, so the operation is naturally recursive.

## Rust Approach

Rust's iterator `.filter()` takes a closure `&Self::Item -> bool`. Chaining `.iter().filter(predicate).copied().collect()` produces a `Vec<T>` in one expression. For simultaneous two-way splitting, `.partition()` is more efficient than calling `filter` twice.

## Key Differences

1. **Return type:** OCaml returns `'a list`; Rust collects into `Vec<T>` explicitly via `.collect()`
2. **Predicate argument:** OCaml takes `'a -> bool`; Rust's `.filter()` receives `&&T` (iterator yields `&T`, filter passes `&&T`) — use `.copied()` or dereference carefully
3. **Two-way split:** OCaml requires two separate `List.filter` calls; Rust has `Iterator::partition` for a single pass
4. **Recursive predicate passing:** Rust generics instantiate a new type per closure, so recursive helpers must take `predicate: &F` to avoid infinite monomorphization depth
