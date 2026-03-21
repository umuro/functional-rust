📖 **[View on hightechmind.io →](https://hightechmind.io/rust/932-stack-module)**

---

# 932-stack-module — Stack Module with Signature
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Module signatures in OCaml enforce interface contracts: you declare what a module provides (types and functions), and the compiler verifies that implementations satisfy the contract. This enables abstract data types, multiple implementations of the same interface, and documentation by contract. Rust's traits serve the same role: they define a set of methods that concrete types must implement. A `Stack` trait with `push`, `pop`, `peek`, `empty`, and `is_empty` methods can be satisfied by `Vec`, `LinkedList`, or a custom persistent stack — all interchangeable behind the trait interface.

## Learning Outcomes

- Define a `Stack` trait as the Rust equivalent of OCaml's `module type STACK`
- Implement the trait for a concrete `ListStack<T>` backed by `Vec<T>`
- Understand why persistent (functional, non-mutating) stacks require `&self` returning new values
- Use associated types (`type Item`) for the element type
- Compare Rust's traits with OCaml's module signatures and `include` for multiple implementations

## Rust Application

`trait Stack` declares `type Item`, `empty()`, `is_empty()`, `push(&self, item) -> Self`, `peek() -> Option<&Self::Item>`, `pop() -> Option<Self>`. The `push` and `pop` return new `Self` values — persistent (immutable) style. `ListStack<T>` implements this by cloning the inner `Vec` on each push/pop. The test verifies that multiple stack operations chain correctly. The persistent style ensures the original stack remains valid after push/pop — you can keep multiple versions.

## OCaml Approach

`module type STACK = sig type t; type item; val empty: t; val push: item -> t -> t; val peek: t -> item option; val pop: t -> t option; ... end`. A persistent `module Stack : STACK with type item = int = struct ... end`. OCaml's module system supports first-class modules, functor instantiation, and abstract types opaque to users of the module. Multiple stacks can be created from the same functor `MakeStack(T: OrderedType)`.

## Key Differences

1. **Abstract types**: OCaml module signatures can make `type t` fully abstract (users cannot create values directly); Rust traits cannot hide the implementing type.
2. **Functional vs object**: OCaml modules are structural values; Rust traits are interface contracts on objects. Conceptually similar but mechanically different.
3. **Multiple implementations**: Rust multiple `impl Trait for Type` blocks per type; OCaml multiple modules satisfying the same `module type`.
4. **First-class**: OCaml modules are first-class values that can be passed as function arguments; Rust trait objects (`dyn Trait`) provide similar dynamic dispatch.

## Exercises

1. Implement a `MutableStack<T>` that satisfies a `MutStack` trait with `push(&mut self, item: T)` and `pop(&mut self) -> Option<T>` methods.
2. Add a `map_stack<U>` method to the `Stack` trait that applies a function to all elements and returns a new stack.
3. Implement `Stack` for `VecDeque<T>` and write a `use_stack<S: Stack<Item = i32>>(s: S)` function that works with any implementation.
