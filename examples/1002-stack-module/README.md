# Example 1002: Stack Module with Signature
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement a persistent functional stack: a LIFO data structure where `push` and `pop` return entirely new stack values rather than mutating in place. The OCaml original uses a module signature (`STACK`) and a list-backed implementation (`ListStack`) to show how interface and implementation are separated. The Rust version models the same contract using a consuming-method API on a concrete struct, demonstrating how OCaml-style module signatures translate into Rust's type system. The goal is to understand both the structural equivalence and the differences in how each language expresses abstraction and error handling.

## Learning Outcomes

- How OCaml module signatures (`module type STACK`) translate to Rust structs with well-defined consuming-method APIs
- Why consuming `self` by value (`push(self, x) -> Self`, `pop(self) -> Result<Self, Empty>`) achieves a logically persistent interface in Rust
- How method chaining (`Stack::empty().push(1).push(2).push(3)`) mirrors OCaml's pipeline operator style (`empty |> push 1 |> push 2 |> push 3`)
- How OCaml's `exception Empty` maps to Rust's `Result<_, Empty>`, making the empty-stack error case explicit and statically checked
- Why Rust encodes the module-signature contract as inherent methods rather than a trait, and when a trait would be more appropriate

## OCaml Approach

OCaml defines the `STACK` module signature with a polymorphic type `'a t`, an `Empty` exception, and five operations: `empty`, `is_empty`, `push`, `peek`, and `pop`. `ListStack` implements this signature using an `'a list` as the backing store: `push x s = x :: s` (O(1) prepend), `pop = List.tl`, `peek = List.hd`. Both `peek` and `pop` raise the `Empty` exception on an empty list. The signature seals the implementation — callers cannot observe that the stack is a list. The pipeline operator `|>` makes construction read left-to-right: `empty |> push 1 |> push 2`.

## Rust Application

`Stack<T>` wraps a `Vec<T>` and exposes `push(mut self, x: T) -> Self`, `pop(mut self) -> Result<Self, Empty>`, and `peek(&self) -> Result<&T, Empty>`. All mutating operations consume `self` and return a new stack value, making the interface appear persistent. In practice, `push` and `pop` mutate the inner `Vec` in place before returning `self` — this is an optimization invisible to callers. The `Empty` error is a unit struct returned in a `Result`, requiring callers to handle the empty case explicitly rather than catching an exception. Method chaining reads as naturally as OCaml's pipeline operator.

## Key Differences

1. **Error handling:** OCaml raises `Empty` as an exception that callers may or may not catch; Rust returns `Result<_, Empty>`, making the error branch statically visible in every call site's type
2. **Abstraction mechanism:** OCaml uses a module signature to seal the implementation type; Rust makes `items: Vec<T>` a private field — callers cannot see the backing store without a trait
3. **Persistence semantics:** OCaml's list prepend is inherently persistent and O(1); Rust's `Vec`-backed version mutates in place under a consuming interface, giving the appearance of persistence while reusing memory
4. **Interface definition:** OCaml separates signature (`STACK`) from implementation (`ListStack`) explicitly; Rust combines both in the `impl Stack<T>` block, relying on field privacy for encapsulation

## Exercises

1. Implement a truly persistent `Stack` backed by a singly-linked list using `Option<Box<Node<T>>>` so that `push` and `pop` are O(1) without cloning or mutating a `Vec`
2. Add a `peek_n(n: usize) -> Result<&T, Empty>` method that returns a reference to the nth element from the top without removing anything from the stack
3. Implement `From<Vec<T>> for Stack<T>` and `From<Stack<T>> for Vec<T>` conversion traits, then write a round-trip test that verifies the element order is preserved correctly
