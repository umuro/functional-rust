📖 **[View on hightechmind.io →](https://hightechmind.io/rust/197-trampoline)**

---

# Trampoline

## Problem Statement

Rust (and OCaml without tail-call optimization) stack-overflows on deeply recursive functions. A trampoline converts stack recursion into heap iteration: instead of calling the next step recursively, return a thunk (a suspended computation). The trampoline loop executes thunks iteratively on the heap, never growing the call stack. This enables "infinite" recursion and mutual recursion on arbitrary inputs without stack overflow.

## Learning Outcomes

- Understand why stack overflow occurs in recursive functions
- Learn the trampoline pattern: return `Done(value)` or `More(thunk)` instead of recursing
- See how trampolining converts stack depth into heap allocation
- Understand the trade-off: heap allocation per step vs. stack frame per step

## Rust Application

`enum Trampoline<T> { Done(T), More(Box<dyn FnOnce() -> Trampoline<T>>) }`. The trampoline loop: `while let More(f) = t { t = f(); }`. CPS-style factorial becomes trampolinable: instead of `go(n-1, n*acc)`, return `Trampoline::More(Box::new(move || go(n-1, n*acc)))`. The loop unrolls this into O(n) heap allocations. For very large `n`, this is slower than iterative code but never stack-overflows.

## OCaml Approach

OCaml guarantees tail call optimization for direct tail calls:
```ocaml
let rec factorial n acc = if n = 0 then acc else factorial (n-1) (n*acc)
```
This is already stack-safe in OCaml. Trampolining is only needed in OCaml for mutually recursive functions that the compiler cannot prove are tail-recursive. OCaml's `tailcall` ppx annotation forces TCO and warns if it fails.

## Key Differences

1. **TCO guarantee**: OCaml guarantees TCO for direct tail calls; Rust does not — trampolining is always required for stack-safe deep recursion in Rust.
2. **Mutual recursion**: Neither language's standard TCO handles mutual tail recursion (between two different functions); trampolining handles this in both.
3. **Heap cost**: Each trampoline step allocates a `Box<dyn FnOnce>`; OCaml's TCO reuses the stack frame — zero allocation.
4. **Readability**: Trampolined code is harder to read than direct recursion; the transformation is mechanical and can be automated with a macro.

## Exercises

1. Implement trampolined mutual recursion: `is_even(n)` and `is_odd(n)` using `Trampoline` with no stack growth.
2. Write a macro `trampoline!` that transforms a recursive function into its trampolined equivalent automatically.
3. Measure the performance difference between trampolined vs. iterative Fibonacci for n=100,000.
