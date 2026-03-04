# Example 268: Continuation-Passing Style

**Difficulty:** ⭐⭐⭐
**Category:** Higher-Order Functions | Functional Patterns | Recursion
**OCaml Source:** Standard FP pedagogy — CPS transform on factorial and tree sum

## Problem Statement

Transform direct-style recursive functions into continuation-passing style (CPS), where the "what to do next" computation is passed explicitly as a closure argument, making every recursive call structurally tail-recursive.

## Learning Outcomes

- How CPS externalises the call stack into heap-allocated closure chains
- Why Rust requires `Box<dyn FnOnce(T) -> R>` for heterogeneous continuation chains
- The difference between `Fn`, `FnMut`, and `FnOnce` and why CPS continuations are `FnOnce`
- Why structural tail-recursion alone does not guarantee stack safety in Rust (no TCO guarantee)

## OCaml Approach

OCaml uses `Fun.id` as the initial "identity" continuation and builds up a chain of closures with each recursive call. Because OCaml's compiler optimises self-tail-calls, the CPS factorial genuinely avoids stack growth. The tree variant threads two continuations — one for the left subtree, one for the right — composing them with a lambda.

## Rust Approach

Rust represents each continuation as `Box<dyn FnOnce(T) -> R>` because the type of a nested closure chain is infinite and heterogeneous, requiring heap allocation. Each `FnOnce` closure captures the previous continuation by move and calls it exactly once. While the CPS structure is correct, Rust does not guarantee tail-call optimisation, so the call stack still grows proportionally to the input size.

## Key Differences

1. **Tail-call optimisation:** OCaml optimises self-tail-calls; Rust does not guarantee TCO, so CPS alone does not prevent stack overflow for large inputs.
2. **Continuation type:** OCaml uses polymorphic higher-order functions transparently; Rust needs `Box<dyn FnOnce(...)>` to erase the infinite nested closure type.
3. **`FnOnce` vs `Fn`:** Each continuation is consumed exactly once (`FnOnce`); using `Fn` would require the closure to be callable multiple times, which conflicts with moving the captured previous continuation out of it.
4. **Ownership:** `move` closures transfer captured values (references, boxes) into the closure; lifetimes on `Box<dyn FnOnce(...) + 'a>` ensure the tree reference outlives the continuation chain.
