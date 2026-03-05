# Safe Linked List — Comparison

## Core Insight
OCaml's list IS a linked list — it's the default, most natural data structure. In Rust, linked lists fight the ownership system. `Option<Box<Node<T>>>` works for singly-linked lists but requires careful use of `take()` to move ownership. This is a key lesson in why Rust prefers `Vec`.

## OCaml Approach
- Lists are built-in: `[1; 2; 3]` is a linked list
- Immutable cons cells: `x :: xs` creates a new head
- Pattern matching for destructuring
- GC handles memory — no ownership concerns
- Custom ADT: `type 'a node = Nil | Cons of 'a * 'a node`

## Rust Approach
- `Option<Box<Node<T>>>` — Box for heap allocation, Option for null
- `take()` on Option to transfer ownership (replaces with None)
- Must implement `Drop` manually to avoid stack overflow on large lists
- Iterator requires lifetime-annotated struct
- No shared tails (unlike OCaml's persistent lists)

## Comparison Table

| Feature | OCaml | Rust (safe) |
|---|---|---|
| Built-in | Yes (`list`) | No (must build) |
| Mutability | Immutable | Mutable |
| Memory management | GC | Ownership + Box |
| Shared tails | Yes (free) | No (would need Rc) |
| Push front | `x :: xs` | `self.head.take()` dance |
| Pattern match | Native | `if let` / `match` |
| Drop behavior | GC | Must implement iterative Drop |
| Recommended | Always | Rarely — use `Vec` instead |
