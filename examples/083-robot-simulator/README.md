[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 083 — Robot Simulator

## Problem Statement

Simulate a robot on a grid that can turn left, turn right, and advance. The robot's state (`x`, `y`, `dir`) is immutable — each instruction returns a new `Robot` rather than mutating in place. Execute a sequence of instructions using fold, and compare with OCaml's record update syntax `{ r with ... }`.

## Learning Outcomes

- Use `..self` struct update syntax for functional record updates in Rust
- Derive `Copy` for small state structs to enable pass-by-value semantics
- Represent direction rotation with match on a cyclic enum
- Fold a list of instructions with `Iterator::fold`
- Map Rust's `..self` spread to OCaml's `{ r with field = value }` update
- Understand when immutable state machines are preferable to mutable ones

## Rust Application

`Robot` and `Direction` both derive `Copy` — the struct is small enough to copy on every instruction. `advance(self) -> Self` returns a new `Robot` using `Robot { y: self.y + 1, ..self }` to update only the `y` field while keeping the rest. `turn_right` and `turn_left` cycle through four directions via match. `run` folds over a slice of `Instruction` using `fold(initial, |r, &inst| r.execute(inst))`. The functional style means each step in the computation is pure and testable in isolation.

## OCaml Approach

OCaml records have built-in update syntax: `{ r with y = r.y + 1 }`. This is semantically equivalent to Rust's `..self`. `List.fold_left execute r instructions` threads the robot state through the instruction list. OCaml records are not `Copy` by default but are value types (immutable by default), so the semantics are the same — each update produces a new value. The OCaml code is more concise because record update syntax is built into the language rather than being a struct initialiser shorthand.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Record update | `Robot { x: 5, ..self }` | `{ r with x = 5 }` |
| Immutability | Explicit `Copy` + value return | Default for records |
| Fold | `iter.fold(init, f)` | `List.fold_left f init lst` |
| Direction cycle | `match` on enum | `match` on variant |
| State type | Derives `Copy, Clone, PartialEq` | Record with field names |
| Code length | ~80 lines | ~25 lines |

The functional state machine pattern — where each operation returns a new state rather than mutating — maps cleanly to both languages. Rust requires explicit `Copy` and `..self` syntax; OCaml's record update is more terse but both express the same immutable update semantics.

## Exercises

1. Add bounds checking: return `Result<Robot, String>` from `advance` if the robot would leave a grid of size N×N.
2. Implement a `reverse_instructions` function that, given a final robot state and instruction sequence, returns the starting state.
3. Add a `TurnAround` instruction that composes two `TurnRight` calls.
4. Track the path: add `run_with_history` that returns `Vec<Robot>` of every state visited.
5. In OCaml, extend the robot to face `NorthEast | NorthWest | SouthEast | SouthWest` and implement eight-directional movement.
