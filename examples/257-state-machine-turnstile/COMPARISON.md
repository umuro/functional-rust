# OCaml vs Rust: State Machine — Turnstile

## Side-by-Side Code

### OCaml

```ocaml
type state = Locked | Unlocked
type event = Coin | Push

let transition state event = match state, event with
  | Locked, Coin -> Unlocked
  | Unlocked, Push -> Locked
  | Locked, Push -> Locked
  | Unlocked, Coin -> Unlocked

let state_name = function Locked -> "Locked" | Unlocked -> "Unlocked"

let () =
  let events = [Coin; Push; Push; Coin; Coin; Push] in
  let final = List.fold_left (fun s e ->
    let s' = transition s e in
    Printf.printf "%s -> %s\n" (state_name s) (state_name s');
    s'
  ) Locked events in
  Printf.printf "Final: %s\n" (state_name final)
```

### Rust (idiomatic — method on enum)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State { Locked, Unlocked }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event { Coin, Push }

impl State {
    pub fn transition(self, event: Event) -> Self {
        match (self, event) {
            (Self::Locked,   Event::Coin) => Self::Unlocked,
            (Self::Unlocked, Event::Push) => Self::Locked,
            (Self::Locked,   Event::Push) => Self::Locked,
            (Self::Unlocked, Event::Coin) => Self::Unlocked,
        }
    }

    pub fn name(self) -> &'static str {
        match self { Self::Locked => "Locked", Self::Unlocked => "Unlocked" }
    }
}

pub fn run_machine(initial: State, events: &[Event]) -> (State, Vec<(State, State)>) {
    let mut transitions = Vec::with_capacity(events.len());
    let final_state = events.iter().fold(initial, |s, &e| {
        let next = s.transition(e);
        transitions.push((s, next));
        next
    });
    (final_state, transitions)
}
```

### Rust (functional style — free function mirrors OCaml)

```rust
pub fn transition(state: State, event: Event) -> State {
    state.transition(event)
}

/// Mirrors: List.fold_left (fun s e -> transition s e) initial events
/// scan yields every intermediate state, including the initial value.
pub fn fold_events(initial: State, events: &[Event]) -> Vec<State> {
    std::iter::once(initial)
        .chain(events.iter().scan(initial, |s, &e| {
            *s = transition(*s, e);
            Some(*s)
        }))
        .collect()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| State type | `type state = Locked \| Unlocked` | `enum State { Locked, Unlocked }` |
| Event type | `type event = Coin \| Push` | `enum Event { Coin, Push }` |
| Transition | `val transition : state -> event -> state` | `fn transition(self, Event) -> Self` |
| Fold over events | `List.fold_left f initial events` | `events.iter().fold(initial, f)` |
| Scan with trace | `List.fold_left` + side effects | `Iterator::scan` |
| Copy value | implicit (OCaml values copied freely) | `#[derive(Clone, Copy)]` explicit |

## Key Insights

1. **Tuple match syntax is identical:** Both OCaml `match state, event with` and Rust `match (self, event)` destructure a pair of discriminants. The arm bodies differ only in syntax, not in logic.

2. **Method vs free function:** OCaml's free `transition` function is equally valid in Rust, but placing it as `State::transition` is idiomatic — behaviour lives with the type. Both forms appear in `lib.rs` to show the parallel.

3. **`Copy` eliminates borrowing ceremony:** Marking `State` and `Event` as `Copy` means every function can take values instead of references. No `&State` vs `State` decisions needed — the type is as cheap as an integer.

4. **`scan` is fold with observable intermediate states:** OCaml uses `fold_left` with `printf` inside the accumulator closure to observe each step. Rust's `scan` achieves the same: it threads mutable state through an iterator, yielding every intermediate value. `once(initial).chain(scan(...))` produces the full trace including the starting state.

5. **Exhaustiveness is guaranteed in both languages:** The compiler verifies that every `(State, Event)` pair is handled. Adding a new variant to either enum immediately produces a compile error, making state machine evolution safe by construction.

## When to Use Each Style

**Use method-on-enum (idiomatic Rust) when:** you want the transition logic discoverable through the type, benefit from IDE autocompletion on `state.`, and prefer keeping related behaviour co-located with the data.

**Use free-function style when:** you are porting OCaml directly and want the parallel to remain visually obvious, or when the transition function needs to be passed as a first-class value (e.g., `events.iter().fold(initial, |s, &e| transition(s, e))`).
