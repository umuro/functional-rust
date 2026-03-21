📖 **[View on hightechmind.io →](https://hightechmind.io/rust/196-delimited-cont)**

---

# Delimited Continuations
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Full continuations (`callcc`) capture the entire remaining computation. Delimited continuations capture only the computation up to a "delimiter" — a prompt that marks the boundary. This makes them composable and less dangerous than full continuations. Delimited continuations are the foundation of effect handlers (OCaml 5), generators, and async/await. Understanding them reveals how all these features share a common computational model.

## Learning Outcomes

- Understand the difference between full and delimited continuations
- Learn `reset` (sets the delimiter) and `shift` (captures the delimited continuation)
- See how delimited continuations generalize generators, exceptions, and async
- Understand why OCaml 5's effect handlers are implemented via delimited continuations

## Rust Application

Rust cannot express delimited continuations natively without unsafe code. The simulation uses a `Result`-based encoding: `Shift(continuation)` represents suspension up to the nearest `reset`. `reset(f: impl FnOnce() -> Step<A>) -> A` provides the delimiter; `shift(f: impl FnOnce(Continuation<A, B>) -> Step<B>) -> Step<A>` captures the continuation up to the delimiter. This simulation is limited — it cannot express multi-shot continuations, which require the ability to call the captured continuation multiple times.

## OCaml Approach

OCaml 5 implements delimited continuations natively:
```ocaml
let () =
  match_with (fun () ->
    let k = perform (Shift (fun k -> k)) in  (* capture delimited cont *)
    Printf.printf "resumed\n";
    perform (Resume k)  (* resume it *)
  ) () { ... }
```
The `delimcc` library (for older OCaml) provides `new_prompt`, `push_prompt`, and `shift0` / `control0` as the full delimited continuation API.

## Key Differences

1. **Native support**: OCaml 5's effect system is built on delimited continuations; Rust requires simulation or `unsafe` code.
2. **Multi-shot**: OCaml's `continue k` can be called multiple times (non-determinism, backtracking); Rust's `FnOnce` and `Box<dyn FnOnce>` are single-shot only.
3. **Stack frames**: OCaml captures actual stack frames as continuations — zero-copy; Rust's simulation copies data into closures.
4. **Composability**: Delimited continuations in OCaml compose via nested `match_with`; Rust's simulation composes poorly beyond simple cases.

## Exercises

1. Implement a generator using the delimited continuation simulation: `yield_value` shifts, `next_value` resets.
2. Implement simple backtracking: `amb([1, 2, 3])` tries all options using multi-shot continuation semantics.
3. Write a `reset`/`shift` combination that implements `Result` propagation — `shift` throws an error, `reset` catches it.
