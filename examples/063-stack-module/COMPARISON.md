## Core Insight

A stack is LIFO (last in, first out). Both languages encapsulate the implementation: OCaml with module signatures, Rust with struct visibility. The functional stack uses a linked list; the Rust stack wraps Vec.

## OCaml Approach
- Module with signature hiding internals
- Immutable stack using list (cons = push, tail = pop)
- `push`, `pop`, `peek`, `is_empty` operations
- Each operation returns a new stack (persistent)

## Rust Approach
- `struct Stack<T> { elements: Vec<T> }` with private field
- `impl Stack<T>` with `push`, `pop`, `peek`
- Mutable methods modify in place
- Can also implement immutable version

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Encapsulation | Module signature | Private fields |
| Push | `x :: stack` O(1) | `.push(x)` O(1) amortized |
| Pop | `List.tl stack` | `.pop()` → `Option<T>` |
| Peek | `List.hd stack` | `.last()` → `Option<&T>` |
| Mutability | Immutable (new stack) | Mutable in-place |
