# 512: Closures as State Machine Transitions

**Difficulty:** 3  **Level:** Intermediate

Encode automaton states as closures that return their own successor.

## The Problem This Solves

A finite automaton has states and transitions. The naive approach is a loop over an enum with a match statement inside — functional but verbose, and the state logic is spread across a flat match. An alternative is to make each *state* a function (or closure) that consumes one input character and returns the *next* state. The "current state" is just the current closure. Transitioning is calling it.

This pattern keeps state logic co-located: all the rules for `state_after_a` live in one place. It's easy to add states without touching others. And because Rust closures can capture mutable variables via `move`, you can build stateful recognizers that accumulate values or counters as they run — no separate state struct needed.

The `make_lexer()` pattern in this example is the practical version: a single closure captures a mutable `LexState` enum, and each call advances the state. The closure *is* the state machine. Hand it characters, collect results.

## The Intuition

Imagine each state as a room. You enter a room, show it a character, and it points you to the next room. The rooms are closures. Moving between rooms is calling the current closure with the next input. The machine has "accepted" when it lands in a room labeled "Accept."

The stateful closure variant (`make_lexer`) is even simpler: there's one closure, it remembers where it is, and you just keep feeding it characters. State is hidden inside the closure's captured environment.

## How It Works in Rust

1. **State as closure** — each state is a `fn(char) -> StateResult`; `StateResult::Continue(Box<dyn Fn(char) -> StateResult>)` carries the next state.
2. **Fold over input** — `chars.fold(initial_state, |state, c| match state { Continue(f) => f(c), other => other })` runs the machine.
3. **Stateful closure** — `make_lexer()` returns `impl FnMut(char) -> LexState`; the `LexState` is captured as a `move` variable and updated on each call.
4. **`FnMut` for mutation** — since the lexer writes to its captured state, it's `FnMut`, not `Fn`.
5. **Acceptance check** — after folding, check whether the final state is the accepted terminal state (`InB` for the `a*b+` pattern).

## What This Unlocks

- Build lexers and parsers where each state's logic is isolated in its own closure.
- Compose state machines from smaller pieces — each state can delegate to sub-machines.
- Carry accumulated output (tokens, counts) inside the closure's captured environment with no external state struct.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| State as function | Natural; functions are first-class | Same; closures are first-class, can capture mutable state |
| Recursive state types | `type state = char -> state` (with `rec`) | Must box: `Box<dyn Fn(char) -> StateResult>` — no recursive type alias |
| Stateful closures | Mutable closures via `ref` capture | `move` capture + `FnMut` for closures that mutate their environment |
| Fold-based automaton | `List.fold_left` | `Iterator::fold` — identical pattern |
