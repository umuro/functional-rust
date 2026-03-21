📖 **[View on hightechmind.io →](https://hightechmind.io/rust/194-coroutines-gen)**

---

# Coroutines and Generators
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Generators and coroutines produce sequences of values lazily — yielding one value at a time rather than computing the entire sequence upfront. This enables infinite sequences, pipeline processing without intermediate allocations, and cooperative multitasking. Python's `yield`, JavaScript's `function*`, C#'s `yield return`, and Rust's nightly `Generator` trait all implement this pattern. Understanding generators builds intuition for `async/await` (generators that yield futures).

## Learning Outcomes

- Understand coroutines as functions that can suspend at `yield` points and be resumed
- Learn how to simulate generators in stable Rust using iterators and closures
- See the connection between generators and `async/await` (both are coroutines)
- Understand lazy evaluation and infinite sequences through the generator lens

## Rust Application

In stable Rust, generators are simulated with closures and `impl Iterator`. `struct Generator<S, F: FnMut(&mut S) -> Option<T>>` holds a state and a step function. The iterator implementation calls `step(&mut state)` to produce the next value — equivalent to resuming a coroutine. Infinite sequences like Fibonacci, natural numbers, and range iterators are natural generators. Nightly Rust provides `std::ops::Generator` with `yield` keyword for proper coroutine support.

## OCaml Approach

OCaml 5's effect handlers implement generators:
```ocaml
effect Yield : 'a -> unit
let generate f =
  let next = ref (fun () -> None) in
  (* install an effect handler that captures the continuation *)
  ...
```
OCaml's `Seq` module provides lazy sequences equivalent to generators: `type 'a node = Nil | Cons of 'a * 'a t` where `'a t = unit -> 'a node`. Each `Cons` holds a thunk for the next element — a pull-based generator.

## Key Differences

1. **Native support**: Rust nightly has `Generator` with `yield`; OCaml 5 uses effects for generators; both stable-Rust and pre-5 OCaml use iterators/Seq as approximations.
2. **Push vs. pull**: Rust iterators are pull-based (caller calls `next`); OCaml's `Seq` is pull-based too; push-based generators (callbacks) are also possible.
3. **Infinite sequences**: Both support infinite lazy sequences; Rust's `Iterator` chain with `take(n)` limits consumption; OCaml's `Seq.take` does the same.
4. **`async` connection**: Rust's `async fn` desugars to a state machine similar to generators; `yield` and `await` have the same fundamental semantics.

## Exercises

1. Implement an infinite Fibonacci generator as `impl Iterator<Item = u64>`.
2. Write a `take_while` generator that yields values from another generator until a predicate fails.
3. Implement a `zip_generators` combinator that pairs elements from two generators, stopping when either is exhausted.
