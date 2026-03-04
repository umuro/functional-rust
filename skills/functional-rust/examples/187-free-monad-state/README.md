# 187: Free Monad with State

**Difficulty:** ⭐⭐⭐⭐⭐  **Level:** Expert

Encode mutable state as a free algebra — describe `Get` and `Put` operations as data, then thread state through a pure interpreter with no actual mutation.

## The Problem This Solves

Mutable state is hard to test, hard to reason about, and hard to compose. If a function reads and writes a counter, testing it requires setting up real mutable state, running the function, and inspecting side effects. You can't easily replay it, record it, or run it in a different order. The state is implicit — woven into the execution itself rather than being an explicit part of the computation's description.

The free monad approach makes state *explicit*: instead of actually mutating anything, your program *describes* what it wants to do. `get_state()` doesn't read state — it returns a data structure that says "I want to read state here." `put_state(42)` doesn't write state — it returns a data structure that says "I want to write 42 here." The actual state threading happens in a separate `run_state` interpreter that walks the description and carries the state value as a function argument.

This separation means you can inspect the program structure before running it, swap out the interpreter (thread state through a log, run it in reverse, replay specific steps), and test programs by checking the *description* rather than observing side effects.

## The Intuition

Imagine a chef who writes a recipe instead of cooking. The recipe says "read current stock of flour" and "update flour stock to X - 200g". The recipe is just text — it doesn't touch the flour. A kitchen manager takes the recipe and *runs* it: for each step, the manager carries the current pantry state, reads or updates it as instructed, and passes the new state to the next step. The chef and the manager are fully separated.

The free monad is the recipe format. `Free::Free(StateF::Get(...))` is a recipe step that says "give me the current state." `Free::Free(StateF::Put(new_s, next))` says "set state to `new_s`, then continue." The recipe is a tree of these instructions. `run_state` is the manager — it carries state through the tree without the recipe knowing anything about how.

## How It Works in Rust

```rust
// The "instruction" type — describes what the program wants
enum StateF<S, A> {
    Get(Box<dyn FnOnce(S) -> A>),  // "give me S, I'll produce A"
    Put(S, A),                      // "set state to S, then produce A"
}

// Free monad: either a plain value, or an instruction with a continuation
enum Free<S, A> {
    Pure(A),                        // computation complete, here's the value
    Free(Box<StateF<S, Free<S, A>>>), // one instruction, rest of computation in continuation
}

// Smart constructor: "I want to read state"
fn get_state<S: Clone + 'static>() -> Free<S, S> {
    Free::Free(Box::new(StateF::Get(Box::new(|s| Free::Pure(s)))))
    //                                          ^^ continuation: receive S, wrap it
}

// Smart constructor: "I want to write state"
fn put_state<S: 'static>(s: S) -> Free<S, ()> {
    Free::Free(Box::new(StateF::Put(s, Free::Pure(()))))
}

// Monadic bind: chain two computations
fn bind<S, A, B, F>(m: Free<S, A>, f: F) -> Free<S, B>
where F: FnOnce(A) -> Free<S, B> + 'static, ... {
    match m {
        Free::Pure(x) => f(x),  // computation done, apply f to its result
        Free::Free(instr) => match *instr {
            // Push f down into the continuation — build up a larger description
            StateF::Get(cont) => Free::Free(Box::new(StateF::Get(
                Box::new(move |s| bind(cont(s), f))
            ))),
            StateF::Put(s, next) => Free::Free(Box::new(StateF::Put(s, bind(next, f)))),
        },
    }
}

// Interpreter: walk the description, thread real state
fn run_state<S: Clone, A>(init: S, program: Free<S, A>) -> (A, S) {
    let mut state = init;
    let mut current = program;
    loop {
        match current {
            Free::Pure(x) => return (x, state),          // done
            Free::Free(instr) => match *instr {
                StateF::Get(cont) => current = cont(state.clone()), // pass current state to cont
                StateF::Put(new_s, next) => { state = new_s; current = next; }
            },
        }
    }
}

// Example program — pure description, no actual mutation:
let program = bind(get_state::<i32>(), |n|
    bind(put_state(n + 1), |_|
        get_state::<i32>()
    )
);
let (result, final_state) = run_state(0, program);
// result = 1, final_state = 1
```

## What This Unlocks

- **Testable stateful logic** — inspect or replay the free monad tree without running the interpreter; assert on the structure of the computation, not its effects.
- **Alternative interpreters** — `run_state` threads real state; a logging interpreter could record every Get/Put; a dry-run interpreter could count state mutations without applying them.
- **Pure functional state** — enables reasoning with equational laws: `get >> put s >> get ≡ put s >> return s`. Valid because the description is data, not execution.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| GADT instructions | `type 's state_f = Get : ('s -> 'a) -> 'a state_f` — return type varies per constructor | `enum StateF<S, A>` — monomorphic; single A type per instance |
| Bind | Recursive, naturally polymorphic with locally abstract types | Requires explicit `'static` bounds due to `Box<dyn FnOnce>` |
| Interpreter | `let rec go s = function \| Pure x -> (x,s) \| ...` — idiomatic recursion | Iterative loop with mutable `state` and `current` to avoid stack overflow |
| Ergonomics | Cleaner with `let*` syntax (monadic sugar) | Deeply nested `bind` calls; consider macros for real use |
| Applicability | First-class pattern in OCaml FP ecosystems | Less common in Rust; prefer `async/await` or `RefCell` for most cases; free monads are for DSL design |
