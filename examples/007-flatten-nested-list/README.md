📖 **[View on hightechmind.io →](https://hightechmind.io/rust/007-flatten-nested-list)**

---

# Example 007: Flatten Nested List

**Difficulty:** ⭐⭐
**Category:** Recursive Data Structures & Enums
**OCaml Source:** OCaml 99 Problems #7

## Problem Statement

Flatten an arbitrarily nested list structure into a flat list.

## Learning Outcomes

- Model recursive data structures with Rust enums (algebraic data types)
- Compare OCaml's `type 'a node` with Rust's `enum Node<T>`
- Understand ownership transfer vs borrowing for tree-like structures
- Practice stack-based iteration as an alternative to recursion
- See how `flat_map` provides a declarative recursive solution

## OCaml Approach

Defines a recursive variant type `'a node = One of 'a | Many of 'a node list`, then uses a tail-recursive helper with an accumulator to flatten. The GC handles all intermediate allocations.

## Rust Approach

Three approaches:
1. **flat_map recursive**: Declarative, uses `flat_map` + pattern matching — clean but allocates intermediate `Vec`s
2. **Stack-based**: Explicit stack avoids recursion (Rust doesn't guarantee TCO), borrows data
3. **Consuming/owned**: Takes ownership via `into_iter`, zero cloning — most efficient when data isn't reused

## Key Differences

1. **Enum vs variant**: Rust's `enum Node<T>` is the same concept as OCaml's variant type, but owns its data on the heap
2. **No TCO guarantee**: Rust doesn't optimize tail calls, so deep nesting could overflow; stack-based iteration is safer
3. **Ownership matters**: The consuming version (`flatten_owned`) avoids all cloning by taking ownership — impossible in GC languages where sharing is implicit
4. **Clone bound**: Borrowing versions need `T: Clone` to extract values; OCaml copies freely under GC
5. **Memory layout**: Rust's `Vec<Node<T>>` is a contiguous heap buffer; OCaml's list is a chain of heap-allocated cons cells
