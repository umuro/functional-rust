# 060: State Monad

**Difficulty:** 3  **Level:** Advanced

Thread mutable state through pure functions without making everything `mut`.

## The Problem This Solves

You've written a function that needs to count something — lines parsed, IDs assigned, stack depth reached. The natural Rust instinct is `&mut counter`. Fine for one function. But what happens when you have a chain of 10 pure functions, and all of them need to read or update that counter?

Option 1: pass `&mut state` to every function. Every signature gains an extra parameter. Every call site needs to thread it through. The deeper the call stack, the worse it gets. A function that logically takes one argument now takes three because it happens to call helpers that need state.

Option 2: put it in a global with `Mutex`. Now you've introduced shared mutable state, potential deadlocks, and the function is no longer pure or testable in isolation.

Option 3: the functional approach — make state *explicit in the return type*. Every function that needs state returns both its result and the new state: `fn tick(state: i32) -> (i32, i32)`. This is already valid Rust! The State monad is just a clean wrapper around this pattern so you don't have to manually thread the state through every call.

```rust
// Without State monad — state threads manually through every call:
fn count_three(state: i32) -> ((i32, i32, i32), i32) {
    let a = state;       let state = state + 1;
    let b = state;       let state = state + 1;
    let c = state;       let state = state + 1;
    ((a, b, c), state)
}
// Works! But imagine 15 steps where state is used 5 times each...
// Every intermediate variable is named `state`, rebinding it each time.
// The business logic drowns in plumbing.
```

The State monad wraps `fn(S) -> (A, S)` in a struct and gives you `and_then` to chain these functions while passing state automatically. This exists to solve exactly that pain.

## The Intuition

Imagine every step of your computation is a machine that takes the current state, does something, and outputs both a result and the updated state. You chain these machines together: one machine's output state becomes the next machine's input state.

```
state₀ → [Machine A] → (resultA, state₁)
state₁ → [Machine B] → (resultB, state₂)
state₂ → [Machine C] → (resultC, state₃)
```

The State monad is just a way to connect these machines without manually writing `state₁`, `state₂`, `state₃` every time. You describe the chain, then "run" it by feeding in the initial state at the end.

**Jargon decoded:**
- *State monad* — a wrapper around functions of type `S -> (A, S)` with a `bind` that threads state automatically
- *`get`* — returns the current state as the result (state is unchanged)
- *`put(s)`* — replaces the current state with `s` (result is `()`)
- *`modify(f)`* — applies `f` to the current state (like `get` + `put`)
- *`run(initial_state)`* — actually execute the whole chain starting with `initial_state`

## How It Works in Rust

```rust
// The core type: a wrapper around a state-threading function
struct State<S, A> {
    run: Box<dyn FnOnce(S) -> (A, S)>,
}

impl<S: 'static, A: 'static> State<S, A> {
    fn new(f: impl FnOnce(S) -> (A, S) + 'static) -> Self {
        State { run: Box::new(f) }
    }

    // Execute the whole chain with an initial state
    fn run(self, s: S) -> (A, S) {
        (self.run)(s)
    }

    // Wrap a plain value — state passes through unchanged
    fn pure(a: A) -> Self {
        State::new(move |s| (a, s))
    }

    // Chain: run self, pass result + new state to f, run f
    fn and_then<B: 'static>(self, f: impl FnOnce(A) -> State<S, B> + 'static) -> State<S, B> {
        State::new(move |s| {
            let (a, s2) = self.run(s);   // run first step, get result + updated state
            f(a).run(s2)                  // feed result + state into next step
        })
    }
}
```

```rust
// Primitives for working with state:

fn get<S: Clone + 'static>() -> State<S, S> {
    // Copy the state into the result — state unchanged
    State::new(|s: S| (s.clone(), s))
}

fn put<S: 'static>(new_s: S) -> State<S, ()> {
    // Replace state with new_s — result is ()
    State::new(move |_| ((), new_s))
}

fn modify<S: 'static>(f: impl FnOnce(S) -> S + 'static) -> State<S, ()> {
    // Apply f to state — result is ()
    State::new(move |s| ((), f(s)))
}
```

```rust
// A counter step: read current value, increment state, return old value
fn tick() -> State<i32, i32> {
    get::<i32>().and_then(|n| {
        put(n + 1).map(move |()| n)  // increment state, return old n
    })
}

// Chain three ticks — state threads automatically
fn count3() -> State<i32, (i32, i32, i32)> {
    tick().and_then(|a|
        tick().and_then(move |b|
            tick().map(move |c| (a, b, c))))
}

// Run it:
let ((a, b, c), final_state) = count3().run(0);
// a=0, b=1, c=2, final_state=3
```

```rust
// Stack operations — state is Vec<i32>
fn push(x: i32) -> State<Vec<i32>, ()> {
    modify(move |mut stack: Vec<i32>| { stack.push(x); stack })
}

fn pop() -> State<Vec<i32>, Option<i32>> {
    State::new(|mut stack: Vec<i32>| {
        let val = stack.pop();
        (val, stack)
    })
}

// Chain push/pop operations:
let ops = push(1)
    .and_then(|()| push(2))
    .and_then(|()| push(3))
    .and_then(|()| pop());

let (top, remaining_stack) = ops.run(vec![]);
// top = Some(3), remaining_stack = [1, 2]
```

**Important:** Rust requires `Box<dyn FnOnce>` and `'static` bounds on captured values because the closure must be stored and called later. This is the main ergonomic cost vs. OCaml's approach.

**The honest truth:** For most Rust code, explicit `&mut state` is cleaner. The State monad shines when you're building a library of composable stateful operations that users can sequence however they like — like parser combinators or game engine scripting.

## What This Unlocks

- **Incremental ID generation:** A stateful counter wrapped in State can be composed into any sequence of operations that need unique IDs without passing `&mut next_id` everywhere.
- **Parser combinators:** A parser is naturally `State<&str, ParseResult>` — each parser step consumes some input (mutates state) and returns a result. State monad lets you compose parsers without manually tracking the remaining input.
- **Game scripting:** A sequence of game actions (move, attack, pick up item) that all read/update game state can be composed as `State<GameState, ()>` operations, enabling AI behavior trees to be written as pure data.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Type | `type ('s, 'a) state = State of ('s -> 'a * 's)` | `struct State<S, A> { run: Box<dyn FnOnce(S) -> (A, S)> }` |
| Closure storage | First-class values, no boxing needed | Requires `Box<dyn FnOnce>` for heap allocation |
| Lifetime bounds | No `'static` requirement | Captured values in boxed closures need `'static` |
| Idiomatic? | Yes — monadic state threading is natural | Rarely — Rust usually prefers explicit `&mut state` |
| Performance | GC handles allocation | Each `and_then` allocates a new `Box` — can be expensive in hot paths |
| When to choose | Almost always over explicit threading | Only when composability > performance (parsers, DSLs) |
