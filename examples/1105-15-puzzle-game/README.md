# Example 1105: 15-Puzzle Game

**Difficulty:** ⭐⭐
**Category:** General / Games
**OCaml Source:** [Rosetta Code — 15 puzzle game](https://rosettacode.org/wiki/15_puzzle_game)

## Problem Statement

Implement the classic 15-puzzle: a 4×4 grid with 15 numbered tiles and one
empty space.  Tiles adjacent to the empty space can be slid into it.  The goal
is to reach the solved arrangement (tiles 1–15 in row-major order, empty at
bottom-right).

## Learning Outcomes

- Two inverse array representations for the same logical state: board-indexed
  (`board[position] = tile`) vs. tile-indexed (`positions[tile] = position`).
- How to make game logic deterministically testable by injecting an RNG closure
  instead of calling `rand` directly.
- Using `abs_diff` and checked arithmetic to reason about 2-D grid adjacency
  without bounds errors.
- The `Display` trait as the idiomatic Rust substitute for an OCaml `print`
  function that writes to stdout.

## OCaml Approach

The OCaml source uses a *tile-indexed* array: `p.(0)` holds the hole's board
position; `p.(n)` holds tile `n`'s board position.  Swapping `p.(0)` and
`p.(n)` moves tile `n` into the hole.  Shuffling is a simple imperative loop
calling `Random.int`.  Printing reconstructs the visual grid by scattering tile
labels into an output array keyed on board position.

## Rust Approach

The idiomatic Rust version uses a *board-indexed* array: `board[position] =
tile`.  This matches the mental model of "what is at this cell?" and makes
`Display` trivial — just iterate positions in row order.  The hole position is
cached so adjacency checks are O(1).  RNG is injected as a `FnMut() -> usize`
closure, enabling deterministic tests with a seeded LCG from `std` alone.

## Key Differences

1. **Array direction:** OCaml indexes by tile and stores position; Rust indexes
   by position and stores tile — the same information, inverse lookup direction.
2. **Randomness:** OCaml calls `Random.int` directly; Rust injects an RNG
   closure so `shuffle_with` is pure and testable without a random-number
   crate.
3. **Output:** OCaml `print` writes directly to stdout; Rust `display()` returns
   a `String`, and `Display` is implemented separately, keeping the core logic
   I/O-free and testable.
4. **Error handling:** OCaml's `move` silently performs any swap; Rust's
   `apply_move` returns `Result<(), &'static str>` so the caller can handle
   invalid input gracefully.
