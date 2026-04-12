📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1039-stack-via-vec)**

---

# 1039-stack-via-vec — Stack Using Vec
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A stack (LIFO — last in, first out) is one of the most fundamental data structures: it underlies function call frames, expression evaluation, undo history, and DFS traversal. Implementing a stack efficiently requires O(1) push and pop at one end.

Rust's `Vec<T>` provides exactly this: `push` appends to the end (O(1) amortized) and `pop` removes from the end (O(1)). The back of a `Vec` is the top of the stack. No additional data structure is needed — `Vec` IS a stack.

## Learning Outcomes

- Understand that `Vec::push` and `Vec::pop` implement a LIFO stack
- Wrap `Vec` in a newtype to provide a cleaner stack API
- Implement stack-based algorithms: balanced parentheses, expression evaluation
- Understand O(1) amortized push and O(1) pop complexity
- Compare `Vec`-based stacks to linked-list stacks

## Rust Application

`src/lib.rs` wraps `Vec<T>` in a `Stack<T>` newtype providing `push`, `pop`, `peek`, `is_empty`, and `size`. The implementation delegates entirely to `Vec` methods with the appropriate end (back = top of stack). A balanced parentheses checker and an infix-to-postfix converter demonstrate real algorithms built on the stack abstraction.

In practice, Rust code rarely uses a `Stack` wrapper — `Vec` is used directly as a stack, with `push`/`pop` making the intent clear. The wrapper is useful in APIs where you want to prevent indexed access or enforce the LIFO discipline.

## OCaml Approach

OCaml's `Stack` module provides a mutable stack backed by a linked list. The functional alternative is just a list:

```ocaml
(* Functional stack: list is a stack *)
let push value stack = value :: stack
let pop = function [] -> None | x :: rest -> Some (x, rest)
let peek = function [] -> None | x :: _ -> Some x
```

OCaml's built-in `Stack` module uses a linked list, so each push allocates a new cons cell. Rust's `Vec` uses amortized-O(1) heap reallocation, which is cache-friendlier.

## Key Differences

1. **Underlying structure**: Rust uses `Vec` (contiguous memory, O(1) amortized push); OCaml's `Stack` uses a linked list (O(1) push, but pointer-chasing).
2. **Functional vs imperative**: OCaml's list-as-stack is immutable and functional; Rust's `Vec`-as-stack is mutable and imperative.
3. **Peek without pop**: Rust's `Vec::last()` peeks without popping; OCaml's list head is accessible without modification.
4. **Memory**: Rust's `Vec` grows in chunks (amortized O(1)); OCaml's linked list allocates one cons cell per push.

## Exercises

1. Implement a `min_stack` that tracks the current minimum in O(1) by maintaining a parallel stack of minimums.
2. Write a `evaluate_rpn(tokens: &[&str]) -> Result<i64, String>` function that evaluates a Reverse Polish Notation expression using a `Stack<i64>`.
3. Implement `is_balanced_parens(s: &str) -> bool` that handles `()`, `[]`, and `{}` using the stack.
