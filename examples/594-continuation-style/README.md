📖 **[View on hightechmind.io →](https://hightechmind.io/rust/594-continuation-style)**

---

# Continuation-Passing Style (CPS)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Continuation-Passing Style (CPS) is a program transformation where instead of returning a value, a function passes its result to a callback (continuation). Every function takes an extra argument `k: impl FnOnce(T) -> R` and calls `k(result)` instead of returning. CPS has deep roots in compiler theory (CPS IR is used in LLVM, GHC, and OCaml's backend), enables explicit control flow manipulation, and is the foundation for implementing coroutines, async/await, generators, and exception handling. It also eliminates stack overflow in recursive functions.

## Learning Outcomes

- How direct-style functions are transformed to CPS using continuation arguments
- How `fact_k<R>(n, k: Box<dyn FnOnce(u64) -> R>) -> R` passes results to continuations
- How CPS makes the call stack explicit as a chain of closures
- How CPS is related to trampolining for stack-safe recursion
- Where CPS appears: compilers (CPS IR), async/await desugaring, effect systems

## Rust Application

`fact(n) -> u64` is direct style. `fact_k<R: 'static>(n, k: Box<dyn FnOnce(u64) -> R>) -> R` is CPS — it calls `k(1)` for the base case, or recursively calls itself with a continuation that multiplies and calls the original `k`. The continuation chain builds up as closures. `fib_k` shows CPS for a function with two recursive calls — the continuation captures the first result while computing the second.

Key patterns:
- `k: Box<dyn FnOnce(T) -> R>` — continuation argument
- `k(result)` — "return" by calling the continuation
- `fact_k(n-1, Box::new(move |r| k(n * r)))` — building continuation chains
- CPS transform: every call becomes a tail call

## OCaml Approach

OCaml's CPS is natural and the OCaml compiler uses CPS as its intermediate representation:

```ocaml
let rec fact_k n k = if n <= 1 then k 1 else fact_k (n-1) (fun r -> k (n * r))
(* Usage: *)
let result = fact_k 10 (fun x -> x)
```

OCaml's tail-call optimization ensures CPS functions do not stack overflow — a key advantage over Rust where CPS chains of `Box<dyn FnOnce>` still allocate on the heap.

## Key Differences

1. **TCO**: OCaml optimizes tail calls — CPS factorial in OCaml is O(1) stack; Rust does not guarantee TCO, so heap-boxed continuations are needed for large `n`.
2. **Continuation type**: OCaml continuations are plain function values; Rust requires `Box<dyn FnOnce(T) -> R>` to store them dynamically.
3. **Practical use**: OCaml's compiler backend genuinely uses CPS IR; Rust's async/await desugars to state machines rather than CPS.
4. **Allocation**: Rust's CPS with `Box` allocates one heap object per continuation frame; OCaml's GC manages continuation closures but also allocates.

## Exercises

1. **CPS identity**: Write a direct-style `identity(x: i32) -> i32` and its CPS version `identity_k<R>(x: i32, k: impl FnOnce(i32) -> R) -> R` — verify they produce the same results.
2. **CPS addition**: Transform `fn add(a: i32, b: i32) -> i32` to CPS and use it to build `fn sum_list_k(items: &[i32], k: impl FnOnce(i32)) -> ()` that passes the total to `k`.
3. **Defunctionalize**: Transform the CPS `fact_k` to use an explicit stack of `enum Cont { Identity, MultAndCall(u64, Box<Cont>) }` instead of closures — this is defunctionalization.
