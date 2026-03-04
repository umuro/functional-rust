# 257: State Machine — Turnstile

**Difficulty:** 2  **Level:** Intermediate

Model a finite state machine as enums and match transitions — the compiler guarantees exhaustive coverage.

## The Problem This Solves

A turnstile has two states: Locked and Unlocked. Two events: Coin (insert a coin) and Push (push the arm). The transition rules are simple — coin unlocks it, push locks it (if unlocked), and invalid events leave the state unchanged. This is a *finite state machine* (FSM): a fixed set of states, a fixed set of events, and defined transitions between them.

FSMs appear everywhere: network protocol parsers, user interface flows, vending machines, game character AI, compiler lexers. The key property is exhaustiveness: every (state, event) combination must have a defined outcome. In most languages, missing a case means a runtime error. In Rust (and OCaml), it means a compile error.

This example shows how to encode FSMs idiomatically in Rust: enums for states and events, `match` on a tuple `(state, event)` for transitions, and `Iterator::fold` / `Iterator::scan` for running a sequence of events through the machine.

## The Intuition

An FSM is like a flowchart with no loops to infinity. You're always in exactly one box (state). An event draws an arrow to the next box. If there's no arrow for that event from your current box, you stay put.

In Rust, the entire transition table is one `match` expression on `(self, event)`. Every arm is one row in the transition table. If you add a new state or event and forget to handle a combination, the compiler refuses to compile. This is the power of exhaustive pattern matching — the type system enforces completeness.

`Iterator::fold` drives the machine: start with the initial state, process each event in sequence, thread the resulting state through. `scan` is the same but collects every intermediate state — useful for tracing execution.

## How It Works in Rust

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State { Locked, Unlocked }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event { Coin, Push }

impl State {
    pub fn transition(self, event: Event) -> Self {
        match (self, event) {
            (Self::Locked,   Event::Coin) => Self::Unlocked, // coin unlocks
            (Self::Unlocked, Event::Push) => Self::Locked,   // push locks
            (Self::Locked,   Event::Push) => Self::Locked,   // push when locked: no-op
            (Self::Unlocked, Event::Coin) => Self::Unlocked, // coin when unlocked: no-op
        }
    }
}

// Run a sequence of events, returning the final state
pub fn run(initial: State, events: &[Event]) -> State {
    events.iter().fold(initial, |state, &event| state.transition(event))
}

// Collect every intermediate state (useful for testing and tracing)
pub fn trace(initial: State, events: &[Event]) -> Vec<State> {
    events.iter()
        .scan(initial, |state, &event| {
            *state = state.transition(event);
            Some(*state)
        })
        .collect()
}
```

`Copy` makes `State` and `Event` zero-cost to pass around — no cloning, no borrowing, just value semantics. This is idiomatic for small, cheaply-copied types.

## What This Unlocks

- **Protocol parsers** — model TCP states (SYN_SENT, ESTABLISHED, FIN_WAIT), HTTP parsing phases, or MQTT client states as enums.
- **UI flows** — modal dialogs, multi-step wizards, and menu navigation become FSMs with clear compile-time guarantees.
- **Game AI** — enemy states (Idle → Alert → Attacking → Fleeing) as enums; `transition` becomes the AI logic kernel.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type definition | `type state = Locked \| Unlocked` | `enum State { Locked, Unlocked }` |
| Transition | Free function `let transition state event = ...` | Method `impl State { fn transition(self, ...) }` |
| Tuple match | `match (state, event) with ...` | `match (self, event) { ... }` — identical syntax |
| Fold simulation | `List.fold_left transition initial events` | `events.iter().fold(initial, \|s, &e\| s.transition(e))` |
| Exhaustiveness | Type checker enforces it | `match` exhaustiveness check at compile time |
