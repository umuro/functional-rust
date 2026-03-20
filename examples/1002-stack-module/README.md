# Example 1002: Stack Module with Signature

## Problem Statement
Implement a persistent functional stack: a data structure where `push` and `pop` return new stacks rather than mutating in place. Model the OCaml module signature pattern in Rust's type system.

## Learning Outcomes
- How OCaml module signatures map to Rust trait bounds and struct APIs
- Implement a persistent interface using consuming methods (`self` by value) so each operation returns a new stack
- Use `Result` to handle the empty-stack error case that OCaml raises as an exception

## Rust Application
`Stack<T>` wraps a `Vec<T>` and exposes `push(self, x) -> Self`, `pop(self) -> Result<Self, Empty>`, and `peek(&self) -> Result<&T, Empty>`. All mutating operations consume `self` and return a new stack, enabling method chaining (`Stack::empty().push(1).push(2).push(3)`) that mirrors OCaml's pipeline operator style.

## OCaml Approach
OCaml defines a `STACK` module signature and implements it with `ListStack`, backed by a list. `push` is `x :: s` (prepend), `pop` returns the tail, `peek` returns the head. Errors raise the `Empty` exception rather than returning an option type.

## Key Differences
1. **Error handling:** OCaml raises the `Empty` exception on `peek`/`pop`; Rust returns `Result<_, Empty>`, making the error case explicit in the type
2. **Persistence mechanism:** OCaml's list prepend is O(1) and naturally persistent; Rust's `Vec`-backed implementation clones on each `push`/`pop` call for logical persistence
3. **Module vs. type:** OCaml uses a module signature to define the stack interface; Rust encodes the same contract as inherent methods on a concrete struct

## Exercises
1. Implement a `Stack` backed by a singly-linked list using `Option<Box<Node<T>>>` instead of `Vec<T>` for O(1) push and pop without cloning
2. Add a `peek_n(n)` method that returns a reference to the nth element from the top without removing anything
3. Implement `From<Vec<T>>` for `Stack<T>` and `From<Stack<T>>` for `Vec<T>` to make the stack interoperable with standard collections
