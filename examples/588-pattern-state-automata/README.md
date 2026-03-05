# 588: Finite Automata with Match

**Difficulty:** 3  **Level:** Intermediate

Model state machines with enums and match — states carry data, transitions are exhaustively verified.

## The Problem This Solves

State machines are everywhere: connection lifecycle, order status, UI flows, protocol parsers. Most implementations use either an integer/string state field (fragile, no compiler help) or a class hierarchy with polymorphic dispatch (lots of boilerplate, easy to forget transitions).

The classic bug: you add a new state but forget to handle the transition from it in one of your event handlers. The machine silently drops events or enters an invalid state. You find out at runtime.

The enum approach makes states first-class types. States can carry data (a `Running` state carries the tick count, a `Paused` state remembers where it was). The `match (state, event)` pattern is a 2D transition table. The compiler checks every combination — if you add a state or event variant, every unhandled combination is a compile error.

## The Intuition

A finite automaton has states, events, and a transition function: `transition(state, event) -> state`. In Rust: one enum for states, one enum for events, one function that matches on `(state, event)` pairs. The tuple match reads like a transition table.

States that carry data are the key upgrade over traditional FSMs. `Running(tick_count)` isn't just a state — it's a state with embedded context. `Paused(where_we_stopped)` remembers history. When you transition back to `Running` from `Paused`, you resume with the correct tick count automatically. No separate fields to keep in sync.

The catch-all arm `(s, _) => s` handles all unrecognized transitions: ignore the event, stay in current state. This is a deliberate design choice made visible in the code.

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
enum State { Idle, Running(u32), Paused(u32), Done(u32) }

#[derive(Debug, Clone, Copy)]
enum Event { Start, Tick, Pause, Resume, Stop }

// Transition table as match on (state, event) tuple
fn transition(state: State, event: Event) -> State {
    match (state, event) {
        (State::Idle,       Event::Start)  => State::Running(0),
        (State::Running(n), Event::Tick)   => State::Running(n + 1),  // increment counter
        (State::Running(n), Event::Pause)  => State::Paused(n),       // save position
        (State::Running(n), Event::Stop)   => State::Done(n),
        (State::Paused(n),  Event::Resume) => State::Running(n),      // restore position
        (State::Paused(n),  Event::Stop)   => State::Done(n),
        (s, _)                             => s,  // ignore invalid events
    }
}

// Drive the machine through a sequence of events
let events = [Event::Start, Event::Tick, Event::Tick, Event::Pause,
              Event::Resume, Event::Tick, Event::Stop];
let mut state = State::Idle;
for event in events {
    state = transition(state, event);
    println!("{:?} -> {:?}", event, state);
}
// Output: Running(0), Running(1), Running(2), Paused(2), Running(2), Running(3), Done(3)

// Simple cyclic automaton — no data needed
#[derive(Clone, Copy)]
enum Traffic { Red, Green, Yellow }

fn next_traffic(t: Traffic) -> Traffic {
    match t {
        Traffic::Red    => Traffic::Green,
        Traffic::Green  => Traffic::Yellow,
        Traffic::Yellow => Traffic::Red,
    }
}
```

## What This Unlocks

- **Data-carrying states** — states embed their context; no separate fields to keep in sync with the state variable.
- **Compiler-verified transitions** — add `State::Crashed` and every unhandled `(Crashed, Event)` pair is a compile error.
- **Readable transition table** — each match arm is literally a row in your state machine diagram.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| State type | `type state = Idle \| Running of int \| ...` | `enum State { Idle, Running(u32), ... }` |
| Transition | `let transition state event = match (state, event) with` | `fn transition(state: State, event: Event) -> State { match (state, event) { ... } }` |
| Data in state | `Running of int` | `Running(u32)` |
| Catch-all transition | `\| (s, _) -> s` | `(s, _) => s` |
| Ownership of state | GC — state can be aliased | Moved in transition; no accidental sharing |
