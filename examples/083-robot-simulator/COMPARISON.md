# Robot Simulator — Comparison

## Core Insight
Both OCaml and Rust support "functional update" syntax for records/structs, creating a new value with some fields changed. Rust's `Copy` trait makes small structs behave like OCaml values — no ownership complications.

## OCaml Approach
- `{ r with y = r.y + 1 }` — record update syntax
- `List.fold_left execute r instructions` — fold over instruction list
- Pattern matching on variants for direction and instruction
- Records are immutable by default

## Rust Approach
- `Robot { y: self.y + 1, ..self }` — struct update syntax
- `#[derive(Copy, Clone)]` makes Robot value-like (no moves)
- `instructions.iter().fold(self, |r, &i| r.execute(i))`
- Methods take `self` by value (Copy), return new Robot

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Update syntax | `{ r with field = val }` | `Struct { field: val, ..self }` |
| Immutability | Default | Via Copy + value semantics |
| Fold | `List.fold_left` | `.iter().fold()` |
| Methods | Free functions | `impl` methods |
| String parsing | Not shown | `run_string` with char fold |

## Learner Notes
- Rust struct update `..self` copies remaining fields (requires Copy or explicit clone)
- Small enums and structs should derive Copy for functional style
- `[Instruction::TurnRight; 4]` creates array with 4 copies (requires Copy)
- OCaml record update and Rust struct update are remarkably similar
