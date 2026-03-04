# 192: Effect Handlers for State

**Difficulty:** 4  **Level:** Expert

Thread mutable state through pure functions using the State monad and a free-monad effect interpreter.

## The Problem This Solves

Most programs need state — counters, accumulators, configuration. The obvious answer is a mutable variable. But mutable variables hide dependencies: any function can read or change state invisibly, making code hard to test and reason about. You can't look at a function's signature and know whether it reads a counter or resets it.

OCaml 5 solves this with *algebraic effects*: you declare `Get` and `Put` as effects, perform them inside pure-looking code, and install a *handler* at the boundary that decides what state means. The handler is separate from the logic. The logic doesn't know whether state lives in a `ref`, a database, or a test mock.

Rust has no built-in algebraic effects. But the same idea maps to two well-known patterns: the **State monad** (a function from old state to new value + new state) and a **free monad / interpreter** (encode Get/Put as data, interpret them with a runner). Both keep state visible in types, testable without mutation, and composable.

## The Intuition

The State monad wraps a computation as a *function*: "give me the current state and I'll give you back a result and the new state." You never touch the state directly — you describe transformations, and `run_state(initial)` executes the whole description at once.

Think of it as a recipe vs cooking: `get` is "read the current value", `put(x)` is "replace the value with x". The recipe doesn't execute until you hand it an initial state. Chaining recipes with `and_then` creates a larger recipe that runs them in sequence.

The free-monad approach goes further: `Get` and `Put` are just enum variants — pure data. An *interpreter* walks the data structure and decides what to do at each step. You can swap interpreters to log state changes, run in-memory, or replay a recorded sequence.

## How It Works in Rust

```rust
// State<S, A> wraps a function: S -> (A, S)
struct State<S, A> {
    run: Box<dyn FnOnce(S) -> (A, S)>,
}

impl<S: 'static, A: 'static> State<S, A> {
    // `get` returns the current state as the value, leaving it unchanged
    fn get() -> State<S, S> where S: Clone {
        State::new(|s: S| (s.clone(), s))
    }

    // `put` replaces the state, returning () as the value
    fn put(new_s: S) -> State<S, ()> {
        State::new(|_old| ((), new_s))
    }

    // `and_then` sequences two stateful computations (monadic bind)
    fn and_then<B: 'static, F: FnOnce(A) -> State<S, B> + 'static>(self, f: F) -> State<S, B> {
        State::new(move |s| {
            let (a, s2) = (self.run)(s);   // run first computation
            (f(a).run)(s2)                 // pass result + new state to second
        })
    }

    fn run_state(self, init: S) -> (A, S) { (self.run)(init) }
}

// Usage: increment a counter three times
let program = State::<i32, _>::get()
    .and_then(|n| State::put(n + 1))
    .and_then(|_| State::get())
    .and_then(|n| State::put(n + 1))
    .and_then(|_| State::get());

let (final_val, final_state) = program.run_state(0);
// final_val = 2, final_state = 2
```

The free-monad interpreter encodes `Get`/`Put` as enum variants and runs them with a `&mut S` reference, allowing imperative efficiency while keeping the interface declarative.

## What This Unlocks

- **Testable state logic** — pass different initial states in tests without global variables or `RefCell` gymnastics.
- **State transformers** — compose independent stateful subsystems (counter + log + config) as nested `State` types.
- **Effect simulation** — the free-monad pattern lets you swap interpreters: production vs test vs logging, with no changes to business logic.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Effect declaration | `effect Get : int` | Enum variant in `StateOp<S>` |
| Effect performance | `perform Get` | `State::get()` or free-monad `Get` variant |
| Handler | `match_with` / `effect_handler` | `run_state(init)` or interpreter function |
| Sequencing | `let* x = expr in ...` | `.and_then(\|x\| ...)` |
| Recursion | GC handles stack | `Box<dyn FnOnce>` allocates on heap |
