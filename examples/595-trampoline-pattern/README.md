📖 **[View on hightechmind.io →](https://hightechmind.io/rust/595-trampoline-pattern)**

---

# Trampoline Pattern

## Problem Statement

Deep recursion causes stack overflow. Rust's default stack is 8MB — a recursive function that calls itself millions of times will overflow. Languages with tail-call optimization (OCaml, Scheme, Haskell) eliminate this through compiler transformation. Rust does not guarantee TCO. The trampoline pattern provides a library-level solution: instead of calling recursively, return a thunk (deferred computation). A loop `run`s the trampoline by repeatedly calling the thunk until `Done`. This converts O(n) stack depth to O(1) stack depth with O(n) heap allocation.

## Learning Outcomes

- What a trampoline is: a loop that evaluates thunks until a final value is produced
- How `Bounce<T> { Done(T), More(Box<dyn FnOnce() -> Bounce<T>>) }` models the trampoline
- How to convert a recursive function to trampoline style
- The tradeoff: O(1) stack, O(n) heap allocation for the thunk chain
- Where trampolines are used: stack-safe interpreters, Scala's `TailCalls`, Haskell's `Cont` monad

## Rust Application

`run<T>(b: Bounce<T>) -> T` loops: `Done(v)` returns the value; `More(th)` calls `th()` to get the next step. `fact_t(n, acc)` returns `More(Box::new(move || fact_t(n-1, n*acc)))` instead of calling recursively — each step is a thunk. The recursion depth is O(1) in stack space. The trampoline processes each step in the `run` loop, allocating one box per step on the heap.

Key patterns:
- `Bounce::More(Box::new(move || next_step()))` — defer next step
- `loop { match bounce { Done(v) => return v, More(f) => bounce = f() } }` — trampoline loop
- Accumulator-style `fact_t(n, acc)` — tail-recursive with trampoline

## OCaml Approach

OCaml has TCO for tail calls — trampolines are unnecessary for simple tail-recursive functions:

```ocaml
(* OCaml with TCO — no stack overflow: *)
let rec fact_acc n acc = if n <= 1 then acc else fact_acc (n-1) (n * acc)

(* If needed, OCaml also has trampolines: *)
type 'a bounce = Done of 'a | More of (unit -> 'a bounce)
let rec run = function Done v -> v | More f -> run (f ())
```

OCaml's `run` function itself uses a tail call and is therefore stack-safe.

## Key Differences

1. **TCO availability**: OCaml guarantees TCO for direct tail calls; Rust does not — trampolines are necessary for stack-safe recursion in Rust.
2. **Stack vs heap**: OCaml TCO uses O(1) stack AND O(1) heap per step; Rust trampolines use O(1) stack but O(n) heap for the thunk chain.
3. **Mutual recursion**: OCaml's TCO handles mutually tail-recursive functions; Rust requires explicit trampolining for mutual tail recursion.
4. **Ergonomics**: OCaml tail-recursive code reads like normal recursive code; Rust trampoline code is more verbose with explicit `Bounce` wrapping.

## Exercises

1. **Fibonacci trampoline**: Convert the naive recursive Fibonacci to trampoline style using an accumulator pair — verify it handles `fib(1_000_000)` without stack overflow.
2. **Mutual trampoline**: Implement mutually tail-recursive `is_even` and `is_odd` using the trampoline pattern where each returns `More(Box::new(|| other(n-1)))`.
3. **Cont monad**: Implement the Continuation monad as a `struct Cont<A, R>(Box<dyn FnOnce(Box<dyn FnOnce(A) -> R>) -> R>)` and show how it relates to the trampoline.
