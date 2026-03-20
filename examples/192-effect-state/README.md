📖 **[View on hightechmind.io →](https://hightechmind.io/rust/192-effect-state)**

---

# Effects as State

## Problem Statement

The state monad is a functional pattern for threading state through a computation without explicit mutable state. The effect-based equivalent provides `Get` and `Put` effects that a handler satisfies by threading state. This unifies state management with the effect handler model, enabling stateful computations to be tested with different initial states, logged, and combined with other effects — all without modifying the computation itself.

## Learning Outcomes

- Implement `Get` and `Put` as algebraic effects satisfied by a state handler
- Compare effect-based state with the state monad and with explicit `&mut` passing
- See how swapping state handlers enables different state management strategies
- Understand how state effects compose with other effects in a handler stack

## Rust Application

The simulation: `enum StateEffect<S> { Get, Put(S) }`. The program uses a callback channel to perform effects: `perform(StateEffect::Get)` suspends until the handler provides a value. The `with_state` handler threads state through: on `Get`, it provides `current_state` and resumes; on `Put(s)`, it updates `current_state` and resumes with `()`. The program description is decoupled from the state management strategy.

## OCaml Approach

OCaml 5 native state effects:
```ocaml
effect Get : int
effect Put : int -> unit
let with_state initial f =
  let state = ref initial in
  match_with f () {
    effc = fun (type a) (e : a eff) ->
      match e with
      | Get -> Some (fun k -> continue k !state)
      | Put v -> Some (fun k -> state := v; continue k ())
      | _ -> None }
```
The `ref` is local to the handler — the program itself has no access to mutable state.

## Key Differences

1. **Native vs. simulated**: OCaml's effect-based state runs at near-native speed; Rust's simulation has closure/heap overhead.
2. **Isolation**: OCaml's handler encapsulates the `ref` cell — the program cannot access it directly; Rust's simulation must structure the callbacks carefully to prevent state leakage.
3. **Composition with other effects**: OCaml's nested handlers compose state with logging or exceptions naturally; Rust's simulation requires explicit effect routing.
4. **Practical alternative**: In Rust, `&mut State` passed through functions is the production idiom; effects are educational/advanced-pattern territory.

## Exercises

1. Implement a `with_pure_state` handler that uses an immutable value thread instead of `ref`.
2. Add a `Modify(f: impl Fn(S) -> S)` effect that atomically applies a function to the state.
3. Compose state and logging: a handler that logs every `Get` and `Put` operation while also managing the state.
