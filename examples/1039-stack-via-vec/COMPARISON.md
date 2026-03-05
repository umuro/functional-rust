# Stack Using Vec — Comparison

## Core Insight
A stack is the simplest data structure, and both languages have a natural fit. OCaml's list is literally a stack (cons cells). Rust's Vec has `push`/`pop` at the end, which is amortized O(1) and contiguous in memory.

## OCaml Approach
- List IS a stack: `x :: xs` = push, `List.hd` = peek, `List.tl` = pop
- Immutable — each push creates a new cons cell
- Module-based wrapper for cleaner API
- Pattern matching for safe pop/peek

## Rust Approach
- `Vec<T>` with `push`/`pop`/`last` — no wrapper needed
- Mutable, amortized O(1) operations
- `pop()` returns `Option<T>` for safe empty handling
- Wrapper struct adds type safety if desired

## Comparison Table

| Feature | OCaml (list) | Rust (`Vec`) |
|---|---|---|
| Push | `x :: xs` O(1) | `push(x)` amortized O(1) |
| Pop | Pattern match O(1) | `pop()` → `Option<T>` O(1) |
| Peek | `List.hd` / pattern match | `last()` → `Option<&T>` |
| Memory | Linked cons cells | Contiguous array |
| Mutability | Immutable | Mutable |
| Cache friendly | No | Yes |
| Wrapper needed | Optional | Optional |
