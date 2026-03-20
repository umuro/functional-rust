📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1076-y-combinator)**

---

# Example 1076: Y Combinator — Anonymous Recursion

**Difficulty:** ⭐⭐⭐
**Category:** Higher-Order Functions
**OCaml Source:** https://rosettacode.org/wiki/Y_combinator#OCaml

## Problem Statement

Implement the Y combinator — a fixed-point combinator that enables recursion without named function bindings. Use it to define factorial and fibonacci as anonymous recursive functions.

## Learning Outcomes

- How Rust handles recursive types with `Rc<RefCell>` vs OCaml's algebraic type wrapper
- Interior mutability patterns for self-referencing closures
- Trait objects (`dyn Fn`) as the Rust equivalent of OCaml's first-class functions
- Three different approaches to anonymous recursion in Rust

## OCaml Approach

OCaml uses an algebraic type `Fix` to wrap the recursive function reference, then builds the fixed-point by pattern matching on the wrapper. The type system handles the recursion through the `Fix` constructor, and garbage collection manages the cyclic reference.

## Rust Approach

Rust cannot have direct cyclic references due to ownership rules. We use `Rc<RefCell<Option<...>>>` to create a shared mutable cell that holds the closure after construction. The Fix-based approach uses `Arc` and boxed trait objects. A third trait-based approach avoids heap allocation entirely.

## Key Differences

1. **Recursive types:** OCaml uses `type 'a fix = Fix of ('a fix -> 'a)` directly; Rust needs `Rc`/`Box` indirection
2. **Self-reference:** OCaml's GC handles cycles naturally; Rust needs `RefCell` for interior mutability
3. **Type erasure:** OCaml functions are first-class; Rust needs `dyn Fn` trait objects for dynamic dispatch
4. **Memory management:** OCaml's GC cleans up the cycle; Rust's `Rc` reference counting handles it

## Exercises

1. Use the Y combinator to implement a recursive `sum` function that adds all integers from 1 to `n` without defining a named recursive function.
2. Implement a memoized Y combinator variant: wrap the fixed-point combinator so that results for previously computed arguments are cached in a `HashMap`.
3. Compare the Y combinator implementation with Rust's explicit recursive closures using a `Box<dyn Fn>` self-reference; benchmark both for computing Fibonacci(30) and explain the overhead difference.
