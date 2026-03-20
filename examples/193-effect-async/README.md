📖 **[View on hightechmind.io →](https://hightechmind.io/rust/193-effect-async)**

---

# Effects as Async
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Asynchronous I/O is the most widely deployed algebraic effect in mainstream programming. Rust's `async/await` transforms async functions into state machines that yield control at each `await` point — the runtime (Tokio, async-std) is the "handler" that resumes computations when I/O completes. This example explores the conceptual connection between algebraic effects and async, showing how `Future` and `Poll` are the concrete Rust implementation of the "suspend and resume" effect model.

## Learning Outcomes

- Understand `async/await` as syntactic sugar over the `Future` trait and algebraic effects
- Learn how `Poll::Pending` and `Poll::Ready` correspond to effect suspension and resumption
- See how the Tokio runtime acts as an effect handler for I/O effects
- Connect the free monad interpretation (program as data) to the async state machine model

## Rust Application

`async fn compute() -> i32` compiles to a `Future` — a state machine that implements `poll`. Each `await` is a suspension point: if the inner future is `Pending`, the outer state machine returns `Pending` (performs the "await effect"); when the inner future completes, the runtime resumes the outer state machine from the suspension point. A simple cooperative scheduler (runtime) shows how `poll` + waker callbacks implement the handler mechanism.

## OCaml Approach

OCaml's `eio` library (built on OCaml 5 effects) implements async I/O via effects:
```ocaml
effect Await : 'a Promise.t -> 'a
let with_scheduler program =
  match_with program () { effc = fun (type a) e ->
    match e with
    | Await p -> Some (fun k ->
        Promise.on_resolve p (fun v -> continue k v))
    | _ -> None }
```
The `Await` effect is intercepted by the scheduler, which registers the continuation and resumes it when the promise resolves.

## Key Differences

1. **State machine vs. continuation**: Rust's `async/await` compiles to a state machine; OCaml's effect-based async uses captured continuations — different implementations of the same semantics.
2. **Zero-cost**: Rust's state machine approach has zero heap allocation per `await` in typical use; OCaml's continuation capture allocates on the heap.
3. **Expressiveness**: OCaml's effect-based async supports multi-shot continuations (forking); Rust's `Future` is single-shot.
4. **Ecosystem**: Rust's async ecosystem (Tokio, Hyper, SQLx) is mature and widely used; OCaml's `eio` is newer but gaining adoption.

## Exercises

1. Implement a minimal future executor that polls a set of futures in a round-robin loop until all complete.
2. Write a `join_all<F: Future>(futures: Vec<F>) -> Vec<F::Output>` that concurrently awaits all futures.
3. Implement a timeout combinator: `with_timeout(duration, future)` that returns `Err(Timeout)` if the future takes too long.
