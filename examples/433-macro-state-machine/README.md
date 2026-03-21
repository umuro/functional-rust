📖 **[View on hightechmind.io →](https://hightechmind.io/rust/433-macro-state-machine)**

---

# 433: Macro-Defined State Machines
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

State machines (finite automata) model systems with discrete states and transitions: TCP connection lifecycle, UI component states, traffic lights, parser automata. Writing state machines by hand requires defining state enums, transition tables, and guards repeatedly. A `state_machine!` macro generates the enum, transition logic, and validity checks from a compact declaration. This keeps the state structure and transitions co-located and prevents invalid transitions from being written at all.

State machine macros appear in embedded systems (motor controllers, protocol implementations), parser generators, UI frameworks, and any system where invalid state transitions must be prevented.

## Learning Outcomes

- Understand how enums naturally model finite state machine states
- Learn how `next()` transition methods enforce valid state progression
- See how macro-generated state machines prevent invalid transitions at compile time
- Understand the difference between enum-based (closed, compile-time) and table-based (open, runtime) state machines
- Learn how `matches!` simplifies guard conditions in state machine methods

## Rust Application

In `src/lib.rs`, `TrafficLight` with `next()` implements a cyclic state machine (Red → Green → Yellow → Red). `DoorState` has guarded transitions: `can_open()` checks `Closed`, `can_close()` checks `Open`, `lock()` returns `None` if already locked. The exhaustive `match` in `next()` ensures every state has a transition — adding a variant requires updating the transition logic. This is the closed-set, compile-time-safe approach.

## OCaml Approach

OCaml state machines use algebraic types with pattern matching. `type state = Red | Yellow | Green` with `let next = function Red -> Green | Green -> Yellow | Yellow -> Red`. OCaml's exhaustiveness checking prevents forgetting a state in transitions. The `with` keyword in OCaml's record syntax enables expressing state updates cleanly. OCaml's module system can encapsulate the state machine behind an abstract type, hiding implementation details.

## Key Differences

1. **Type-state pattern**: Rust can encode state in the type system (`struct Door<S: DoorState>`) making invalid transitions compile errors; OCaml can do this with GADTs but it's less common.
2. **Exhaustiveness**: Both Rust `match` and OCaml `match` enforce exhaustive handling; both emit compiler warnings/errors for incomplete matches.
3. **Mutability**: Rust's `next()` takes `self` by value (returning new state); OCaml's transition functions return new state values functionally.
4. **Macro generation**: Rust macros can generate state machine boilerplate from declarations; OCaml uses functor-based state machine libraries.

## Exercises

1. **Type-state door**: Implement `Door<S>` where `S` is a phantom type parameter (`struct Open; struct Closed; struct Locked`). Write `fn open(door: Door<Closed>) -> Door<Open>` and `fn lock(door: Door<Closed>) -> Door<Locked>`. Prove that `fn open(door: Door<Locked>)` doesn't compile.
2. **Parser state machine**: Implement a simple JSON token parser as a state machine with states `Start`, `InString`, `InNumber`, `InArray`, `Complete`, `Error`. The `next_char(c: char)` transition method drives the machine.
3. **State machine macro**: Write `state_machine!(Light { Red --trigger:Timer--> Green, Green --trigger:Timer--> Yellow, Yellow --trigger:Timer--> Red })` that generates the enum, a `trigger` enum, and a `transition(event: Trigger) -> Option<Self>` method.
