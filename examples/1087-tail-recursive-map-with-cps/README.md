# Example 1087: Tail-Recursive Map with CPS

**Difficulty:** ⭐⭐⭐
**Category:** Lists & HOF
**OCaml Source:** Standard functional programming — tail recursion and CPS techniques

## Problem Statement

Implement `map` over a list in multiple ways: naive recursion, tail-recursive with accumulator-and-reverse, and continuation-passing style (CPS). Compare how each strategy handles stack safety and element ordering.

## Learning Outcomes

- How CPS transforms non-tail-recursive functions into tail-recursive ones
- Why Rust doesn't need tail-call optimization for most map operations (iterators solve it)
- How `Box<dyn FnOnce>` serves as Rust's equivalent of OCaml's closure-based continuations
- The trade-off between heap-allocated continuations (CPS) and a simple reverse (accumulator style)

## OCaml Approach

OCaml provides three classic strategies. The naive `map` builds the result with `f h :: map f t`, which is not tail-recursive and overflows on large lists. The accumulator version `map_tr` collects results in reverse order then calls `List.rev`. The CPS version `map_cps` threads a continuation `k` that captures the output order, making it tail-recursive without needing a final reverse.

## Rust Approach

Idiomatic Rust sidesteps the problem entirely: `list.iter().map(f).collect()` is stack-safe and zero-overhead. For pedagogical purposes, the iterative translation of the accumulator pattern uses an explicit loop with `Vec::with_capacity`. The CPS version uses `Box<dyn FnOnce(Vec<U>) -> Vec<U>>` to heap-allocate the continuation chain, demonstrating the pattern even though it's not the practical choice in Rust.

## Key Differences

1. **Tail-call optimization:** OCaml guarantees TCO, so CPS is truly stack-safe. Rust does not guarantee TCO, so the CPS version still grows the call stack (though continuations move to the heap).
2. **Iterator abstraction:** Rust's iterator pipeline (`iter().map().collect()`) replaces all three OCaml variants for practical use — it's lazy, composable, and stack-safe by construction.
3. **Continuation representation:** OCaml closures are GC-managed and cheap to nest. Rust requires `Box<dyn FnOnce>` with explicit `'static` bounds, making CPS more verbose and allocation-heavy.
4. **Ownership in map:** OCaml's `map` produces a new list (immutable by default). Rust's `map` takes `&[T]` and produces `Vec<U>`, making the borrow/own distinction explicit in the type signature.
