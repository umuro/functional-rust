# OCaml vs Rust: State Automata

## State and Event Types

### OCaml
```ocaml
type state = Idle | Running of int | Paused of int | Done of int
type event = Start | Tick | Pause | Resume | Stop
```

### Rust
```rust
enum State { Idle, Running(u32), Paused(u32), Done(u32) }
enum Event { Start, Tick, Pause, Resume, Stop }
```

## Transition Function

### OCaml
```ocaml
let transition state event =
  match (state, event) with
  | (Idle,       Start)  -> Running 0
  | (Running n,  Tick)   -> Running (n+1)
  | (Running n,  Pause)  -> Paused n
  | (Running n,  Stop)   -> Done n
  | (Paused n,   Resume) -> Running n
  | (Paused n,   Stop)   -> Done n
  | (s, _)               -> s
```

### Rust
```rust
fn transition(state: State, event: Event) -> State {
    match (state, event) {
        (State::Idle, Event::Start) => State::Running(0),
        (State::Running(n), Event::Tick) => State::Running(n + 1),
        (State::Running(n), Event::Pause) => State::Paused(n),
        (State::Running(n), Event::Stop) => State::Done(n),
        (State::Paused(n), Event::Resume) => State::Running(n),
        (State::Paused(n), Event::Stop) => State::Done(n),
        (s, _) => s,
    }
}
```

## Key Pattern: (State, Event) Tuple

The tuple `(state, event)` makes transition tables explicit and readable:
- Each arm is one valid transition
- Wildcard `(s, _)` handles invalid/ignored transitions
- Compiler ensures exhaustiveness

## Running the Machine

### OCaml
```ocaml
let final_state = List.fold_left transition Idle events
```

### Rust
```rust
let final_state = events.iter().fold(State::Idle, |s, &e| transition(s, e));
```

## Benefits

1. **Clear transition table** - All transitions in one place
2. **Exhaustiveness** - Compiler checks all cases
3. **Pure function** - No hidden state mutation
4. **Testable** - Easy to test individual transitions
