[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1001 — Event Loop
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement a functional event loop that dispatches `Event` variants to pure state transitions. The `dispatch` function takes the current `AppState` and an `Event`, returning the next `AppState`. Run the loop with `fold` over a vector of events, stopping at `Quit`. Compare with OCaml's recursive `run_event_loop` using a typed `handler` record.

## Learning Outcomes

- Model events as a Rust enum with variant data (`Click { x, y }`, `KeyPress(char)`)
- Implement `dispatch(state, event) -> AppState` as a pure function
- Use struct update syntax `AppState { clicks: state.clicks + 1, ..state }` for partial updates
- Use `fold` to thread state through a sequence of events
- Use `VecDeque` for a mutable event queue with O(1) push/pop
- Map Rust's fold-based loop to OCaml's tail-recursive `loop state events`

## Rust Application

`Event` is an enum with five variants carrying data. `AppState` is a plain struct deriving `Clone`. `dispatch` matches on the event and returns a new `AppState` with one field updated using struct spread `..state`. `run_event_loop` folds over the event vector with `fold(init, |state, event| dispatch(state, event))`. The fold skips updating after `Quit` but cannot short-circuit — the imperative queue version uses `VecDeque::pop_front` in a `while let` loop with an explicit `break` on `Quit`.

## OCaml Approach

OCaml uses a record `handler` with fields `on_click`, `on_key`, `on_timer`, `on_network`. `run_event_loop ~handler ~init events` is a recursive `loop state events` function that pattern-matches on the head event. `Quit` terminates by returning the current state; other events dispatch to the appropriate handler field. This approach decouples event routing from state logic.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Event type | `enum Event` | `type event` variant |
| State update | Struct spread `..state` | Record update `{ state with … }` |
| Loop style | `fold` (no short-circuit) | Tail recursion with pattern match |
| Queue | `VecDeque` for mutable queue | List head pattern |
| `Quit` handling | `break` in imperative loop | `\| Quit :: _ -> state` |
| Handler dispatch | Single `dispatch` function | Record of functions |

The functional event loop pattern is the foundation of Elm-style architecture and Redux. A pure `dispatch` function makes state transitions testable without side effects — each event produces a predictable new state.

## Exercises

1. Add a `Resize(u32, u32)` event and handle it in `dispatch` by adding `width`/`height` fields to `AppState`.
2. Implement `run_until_quit(queue: &mut VecDeque<Event>, init: AppState) -> AppState` using the imperative `while let` loop.
3. Add event logging: before dispatching, push a `String` description of each event to a `Vec<String>` log.
4. Implement an undo mechanism: keep a `Vec<AppState>` history and add an `Undo` event that pops the last state.
5. In OCaml, implement a priority event queue using a `Map` ordered by event priority, processing higher-priority events first.
