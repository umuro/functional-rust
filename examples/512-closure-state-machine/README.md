📖 **[View on hightechmind.io →](https://hightechmind.io/rust/512-closure-state-machine)**

---

# Closure State Machine
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


A state machine where each state is a `Box<dyn Fn(char) -> StateResult>` — transitions are closures that return the next state or accept/reject — implementing a recogniser for the regex `a*b+`.

## Problem Statement

State machines appear in: lexers (tokenising source code), protocol parsers (HTTP state machine), UI event systems (button: idle → pressed → released), and regex engines. The traditional implementation uses an enum of states with a `match` on transitions. A functional alternative represents each state as a closure that takes input and returns the next state — the **continuation-passing** style. This approach is compositional: states are values that can be passed, stored, and combined.

## Learning Outcomes

- Represent states as `Box<dyn Fn(char) -> StateResult>` — closures as transitions
- Define `StateResult` as an enum: `Accept | Reject | Continue(Box<dyn Fn(char) -> StateResult>)`
- Drive the machine with a `run_machine(input: &str) -> bool` loop
- Understand how `Continue(next_state)` is the continuation-passing equivalent of `goto state`
- Recognise that free functions `state_start`, `state_after_a`, `state_after_b` serve as state closures

## Rust Application

`StateResult` carries the next state inline:

```rust
pub enum StateResult {
    Accept,
    Reject,
    Continue(Box<dyn Fn(char) -> StateResult>),
}
```

State functions return the next state as a `Continue`:

```rust
pub fn state_start(c: char) -> StateResult {
    match c {
        'a' => StateResult::Continue(Box::new(state_after_a)),
        'b' => StateResult::Continue(Box::new(state_after_b)),
        _ => StateResult::Reject,
    }
}
```

`run_machine` iterates characters, updating the current state function:

```rust
let mut state: Box<dyn Fn(char) -> StateResult> = Box::new(state_start);
for c in input.chars() {
    match state(c) {
        Continue(next) => state = next,
        Accept => return true,
        Reject => return false,
    }
}
```

## OCaml Approach

OCaml's algebraic types and first-class functions make this pattern natural:

```ocaml
type result = Accept | Reject | Continue of (char -> result)

let rec state_start c = match c with
  | 'a' -> Continue state_after_a
  | 'b' -> Continue state_after_b
  | _ -> Reject

and state_after_b c = match c with
  | 'b' -> Continue state_after_b
  | _ -> Reject

let run input =
  String.fold_left (fun state c -> match state c with
    | Continue next -> next
    | other -> Fun.const other) state_start input = Accept
```

OCaml's `and` (mutually recursive definitions) is cleaner than Rust's free functions here.

## Key Differences

1. **`Box` overhead**: Each state transition in Rust allocates a `Box`; OCaml's closures are GC-managed without explicit boxing.
2. **Mutual recursion**: OCaml uses `let rec ... and ...` for mutually recursive state functions; Rust uses separately defined free functions (no mutual `fn` recursion syntax needed).
3. **Type-state alternative**: For compile-time state machine verification, Rust's type-state pattern (using distinct types for each state) is preferred over runtime closures.
4. **`Debug` for `StateResult`**: Rust must manually implement `Debug` for `StateResult` because `Box<dyn Fn>` is not `Debug`; OCaml's polymorphic printing handles this automatically.

## Exercises

1. **Extend the DFA**: Add `StateResult::Error(String)` for states that can provide error messages, and implement a new state that recognises `(a|b)*c` (any sequence ending in `c`).
2. **State machine builder**: Design a `DfaBuilder` that accepts a `HashMap<(StateId, char), StateId>` transition table and generates the corresponding closure-based state machine.
3. **Type-state comparison**: Rewrite the `a*b+` recogniser using the type-state pattern (each state is a distinct struct type, transitions are methods) and compare the compile-time guarantees vs. the closure approach.
