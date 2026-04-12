📖 **[View on hightechmind.io →](https://hightechmind.io/rust/195-continuation-passing)**

---

# Continuation-Passing Style
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Continuation-passing style (CPS) transforms functions so that instead of returning a value, they pass the result to a "continuation" callback. CPS makes control flow explicit — every function knows exactly what happens next. It is the basis for compilers (CPS intermediate representations in GHC, MLton), non-local exits, and cooperative scheduling. CPS transform eliminates the call stack: tail calls to continuations never grow the stack.

## Learning Outcomes

- Understand CPS as explicit control flow: every function receives a continuation parameter
- Learn to transform direct-style functions into CPS
- See how `callcc` (call-with-current-continuation) enables non-local exits and backtracking
- Understand how CPS relates to trampolining (example 197) for eliminating stack overflow

## Rust Application

Direct style: `fn add(a: i32, b: i32) -> i32 { a + b }`. CPS style: `fn add_k<K: FnOnce(i32)>(a: i32, b: i32, k: K) { k(a + b) }`. The continuation `k` is passed where the return value would have gone. For recursive functions: CPS-style factorial passes the accumulator and continuation, achieving true tail position — the continuation is called in tail position. Rust's `FnOnce` captures the continuation's environment without heap allocation for small closures.

## OCaml Approach

OCaml naturally supports CPS:
```ocaml
let add_k a b k = k (a + b)
let factorial_k n k =
  let rec go n acc k = if n = 0 then k acc else go (n-1) (n*acc) k in
  go n 1 k
```
OCaml has a `callcc` library via `setjmp`/`longjmp` (unsafe) or via effects (OCaml 5). The `delimcc` library provides delimited continuations. CPS is a common intermediate representation in OCaml compilers.

## Key Differences

1. **Tail call optimization**: OCaml guarantees TCO for direct tail calls and can optimize CPS; Rust does not guarantee TCO — CPS in Rust still risks stack overflow without trampolining.
2. **`callcc`**: OCaml's effects provide first-class continuations; Rust lacks `callcc` — non-local exits use `Result` and `?`, not continuations.
3. **Expressiveness**: Full `callcc` enables coroutines, backtracking, and non-local exits; Rust's CPS simulation provides only the direct sequencing benefit.
4. **Readability**: CPS code in both languages is hard to read; compilers use CPS as IR but programmers rarely write it directly.

## Exercises

1. Convert `sum_list(list: &[i32]) -> i32` to CPS and verify it produces identical results.
2. Implement `map_k<A, B, K: FnOnce(Vec<B>)>(list: Vec<A>, f: impl Fn(A, Box<dyn FnOnce(B)>), k: K)` — CPS map over a vector.
3. Show how CPS factorial becomes iterative when combined with trampolining.
