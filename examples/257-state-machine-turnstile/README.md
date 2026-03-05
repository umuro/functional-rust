📖 **[View on hightechmind.io →](https://hightechmind.io/rust/257-state-machine-turnstile)**

---

# Example 257: State Machine — Turnstile

**Difficulty:** ⭐⭐
**Category:** State Machines | Pattern Matching
**OCaml Source:** Classic finite state machine example

## Problem Statement

Model a turnstile that transitions between `Locked` and `Unlocked` states based on two events — inserting a `Coin` (unlock) or attempting a `Push` (lock if unlocked, stay if locked).

## Learning Outcomes

- How Rust `enum` variants map 1-to-1 to OCaml `type` constructors
- Using tuple patterns `(self, event)` in Rust `match` mirrors OCaml's tuple match syntax exactly
- Placing `transition` as a method on `State` vs. a free function — both are valid, method form is idiomatic Rust
- How `Iterator::fold` and `Iterator::scan` replace OCaml's `List.fold_left` for stateful iteration

## OCaml Approach

OCaml defines `type state` and `type event` as sum types, then writes `transition` as a free function matching on a `(state, event)` tuple. `List.fold_left` drives the simulation, threading state through each event.

## Rust Approach

Rust encodes `State` and `Event` as `enum` types with `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`. The `transition` logic lives as a method `State::transition(self, event: Event) -> Self`, keeping behaviour co-located with the type. The `Copy` bound means no ownership cost for passing states around.

## Key Differences

1. **Method vs free function:** OCaml `let transition state event = ...` is a free function; Rust `impl State { fn transition(self, ...) }` puts the behaviour on the type.
2. **Exhaustiveness:** Both languages guarantee all arms are covered at compile time — OCaml via its type checker, Rust via the `match` exhaustiveness check.
3. **Copy semantics:** Rust's `Copy` trait lets `State` and `Event` be passed by value and used after — no equivalent concept in OCaml (all values are already value-or-pointer transparent).
4. **Fold with side effects:** OCaml's `List.fold_left` with `Printf.printf` inside the closure is a natural pattern; Rust separates concerns by collecting transitions first, then printing.
