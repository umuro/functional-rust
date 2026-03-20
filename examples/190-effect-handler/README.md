📖 **[View on hightechmind.io →](https://hightechmind.io/rust/190-effect-handler)**

---

# Effect Handlers

## Problem Statement

An effect handler is the runtime component that intercepts effects, provides a value, and resumes the computation. Different handlers for the same effects give different behaviors — the same program can run in a logging context, a testing context, or a production context. This is the effect-system equivalent of the free monad interpreter pattern, but operating at the runtime level rather than the data-structure level.

## Learning Outcomes

- Implement effect handlers as closures that intercept and respond to effects
- See how swapping handlers changes program behavior without modifying the program
- Learn how handlers compose (nested handlers for multiple effects)
- Understand the relationship between effect handlers and exception handlers

## Rust Application

The simulation uses a `Handler<E, A>` type: a function from `Effect<E>` to `HandlerResult<A>` where `HandlerResult` is either `Resume(value)` or `Abort(value)`. The program is a closure that calls the handler with each effect. `with_handler(effects, handler, program)` runs the program, intercepting effects and routing them to the handler. Multiple handlers compose by wrapping: the outer handler intercepts its effects; unhandled effects propagate to the inner handler.

## OCaml Approach

OCaml 5's `match_with` is the native handler:
```ocaml
let with_state initial program =
  let state = ref initial in
  match_with program ()
    { effc = fun (type a) (eff : a eff) ->
        match eff with
        | Get -> Some (fun k -> continue k !state)
        | Put v -> Some (fun k -> state := v; continue k ())
        | _ -> None }
```
Nested `match_with` handlers compose naturally — each handles its own effects and forwards others upward.

## Key Differences

1. **Handler composition**: OCaml's nested `match_with` calls compose handlers automatically; Rust's simulation requires explicit forwarding of unhandled effects.
2. **Effect typing**: OCaml's effect type system (planned for OCaml 5.x) will statically type which effects a function performs; Rust's simulation is untyped.
3. **Resumption semantics**: OCaml's handlers can resume the computation at the point of the effect; Rust's simulation uses callback functions instead.
4. **Standard uses**: OCaml's async concurrency (`eio`) is built on effects; Rust's `async/await` uses a different mechanism (state machine transformation).

## Exercises

1. Implement a `with_transaction` handler that wraps all state operations in a simulated transaction, rolling back on error.
2. Write a handler that counts the number of each effect type performed.
3. Compose two handlers: one for logging and one for state, showing that both can intercept their respective effects on the same program.
