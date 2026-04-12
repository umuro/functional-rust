📖 **[View on hightechmind.io →](https://hightechmind.io/rust/063-stack-module)**

---

# 063 — Stack Module
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A stack is a last-in, first-out (LIFO) data structure — one of the two fundamental abstractions alongside the queue. Stacks appear in function call management (the call stack), expression evaluation (operators and operands), undo/redo systems, DFS graph traversal, and balanced-parentheses checking (example 064). The stack abstraction — `push`, `pop`, `peek`, `is_empty` — hides implementation details from callers.

This example contrasts two implementations: a mutable stack (`Stack<T>` wrapping `Vec`) and an immutable persistent stack (`FnStack<T>` as a recursive enum). Persistent stacks are the default in OCaml; Rust typically uses mutable `Vec`-backed stacks for performance.

## Learning Outcomes

- Implement a mutable stack using `Vec` with `push`, `pop`, `peek`, `is_empty`, `size`
- Implement an immutable persistent stack as a recursive enum (`Cons` list)
- Understand the trade-offs: mutable (O(1) amortized push/pop) vs persistent (O(1) push/pop, O(n) size)
- Use `Option` returns for safe pop and peek operations
- Recognize the `Vec`-backed stack as Rust idiom, the enum stack as functional idiom

- Back the `Stack<T>` with `Vec<T>` using `push` (O(1) amortized), `pop` (O(1)), and `peek` (O(1))
- Return `Option<T>` from `pop` and `peek` to handle empty-stack case without panicking

## Rust Application

`Stack<T>` wraps `Vec<T>`: `push` calls `Vec::push`, `pop` calls `Vec::pop` (returns `Option<T>`), `peek` returns `elements.last()`. `FnStack<T>` is `Empty | Cons(T, Box<FnStack<T>>)` — the functional persistent stack. `push` creates `Cons(item, Box::new(self))` returning a new stack without modifying the old one. This enables sharing: multiple "versions" of the stack share the tail.

## OCaml Approach

OCaml's functional stack is the list: `type 'a stack = 'a list`. `push x s = x :: s`, `pop = function [] -> None | x :: t -> Some (x, t)`, `peek = List.nth_opt s 0`. The OCaml `Stack` module provides a mutable imperative stack like Rust's `Vec`-based version. Functional OCaml code normally just uses lists directly.

## Key Differences

1. **List = persistent stack**: OCaml's list is a persistent stack by construction. Prepending (`x :: list`) is O(1) and creates a new list sharing the old tail. Rust's `Vec` does not share structure.
2. **Pop semantics**: OCaml's functional pop returns `(element, new_stack)` as a pair since the stack is immutable. Rust's mutable `Vec::pop()` returns just the element, modifying the stack in place.
3. **`Box` for cons cell**: Rust's `Cons(T, Box<FnStack<T>>)` requires `Box` for the recursive type. OCaml's `'a list = [] | (::) of 'a * 'a list` is built in.
4. **Stack overflow**: Deep OCaml lists can overflow the stack in recursive operations. Rust's `Vec`-based stack avoids recursion entirely.

1. **`Vec` as a stack:** Rust's `Vec` already supports stack operations: `push` (O(1) amortized), `pop` (O(1)), `last` (O(1) peek). OCaml's list is also naturally used as a stack with `h :: t` (push) and `match l with h :: t -> ...` (pop).
2. **Module system:** OCaml's `module Stack = struct ... end` encapsulates the stack implementation. Rust uses `struct Stack<T>` with an `impl` block — the same encapsulation, different syntax.
3. **Error handling:** `pop` and `peek` return `Option<T>` — safe, no panic. OCaml's `match` on an empty list raises `Match_failure` if not handled. Explicit `None` is safer than exceptions.
4. **`Vec` amortized push:** `Vec::push` is O(1) amortized — occasionally triggers a reallocation doubling the capacity. The `Stack` wrapper hides this detail. OCaml's list `h :: t` is always O(1) — no reallocation.

## Exercises

1. **Two-stack queue**: Implement a FIFO queue using two stacks (the classic interview question): one for enqueue, one for dequeue. Amortized O(1) per operation.
2. **Expression evaluator**: Write a postfix (RPN) expression evaluator using a `Stack<f64>`. Process `"3 4 + 2 * 7 /"` by pushing numbers and applying operators.
3. **Linked stack iterator**: Implement `Iterator` for `FnStack<T: Clone>` that yields each element from top to bottom. This requires traversing the linked list structure.

4. **Min-stack**: Implement a stack that tracks the minimum element in O(1) time by maintaining a parallel "min stack" — each push records the current minimum alongside the pushed value.
5. **Stack-based evaluator**: Use the `Stack` module to implement a reverse Polish notation (RPN) evaluator: `evaluate(tokens: &[&str]) -> Result<i64, String>` that handles numbers and operators `+`, `-`, `*`, `/`.
