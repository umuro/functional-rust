📖 **[View on hightechmind.io →](https://hightechmind.io/rust/869-continuation-monad)**

---

# 869-continuation-monad — Continuation Monad

## Problem Statement

Continuation-passing style (CPS) is a program transformation where every function, instead of returning a value, accepts an extra "continuation" argument — a callback representing the rest of the computation. This transform makes control flow explicit and enables features that are otherwise impossible in a direct-style language: early exit, coroutines, and delimited continuations. Scheme pioneered first-class continuations with `call/cc`. The continuation monad wraps this pattern as a composable abstraction with the type `Cont r a = (a -> r) -> r`, fundamental to compiler intermediate representations and effect-system implementations.

## Learning Outcomes

- Understand continuation-passing style and how it makes control flow explicit
- Implement the `Cont` monad type in Rust using `Box<dyn Fn>`
- Recognize CPS factorial as the canonical tail-recursive transformation
- Understand how `callcc` enables early exit from nested computations
- Compare Rust's heap-allocated closures with OCaml's uniform closure representation

## Rust Application

The code defines `Cont<R, A>` wrapping `Box<dyn Fn(Box<dyn Fn(A) -> R>) -> R>`. `cont_return` lifts a pure value, and `run_cont` applies a final continuation. The tests demonstrate basic CPS addition, CPS factorial (which is naturally tail-recursive in CPS form), and the `Cont` monad wrapping a pure value and mapping over it. The OCaml `callcc` (capture current continuation) is shown conceptually; Rust cannot implement it in full generality without `unsafe` or a runtime.

## OCaml Approach

OCaml represents `Cont` as a GADT-free algebraic type `type ('a, 'r) cont = Cont of (('a -> 'r) -> 'r)`. The `bind` function composes two continuations, and `callcc` captures the current continuation, enabling early exit from `List.fold_left`. OCaml's `let*` syntax makes monadic chains readable. The OCaml version shows `find_first_negative` using `callcc` to exit the fold immediately when a negative is found — a pattern impossible with plain `fold`.

## Key Differences

1. **Closure representation**: Rust requires `Box<dyn Fn>` for heap-allocated dynamic closures; OCaml closures are uniformly heap-allocated values.
2. **callcc**: Full `callcc` is not expressible in safe Rust; OCaml (and Scheme) support it natively via the runtime stack.
3. **Lifetime of continuations**: Rust must carefully manage `'static` bounds on captured values; OCaml's GC handles lifetime automatically.
4. **Composition**: OCaml's `>>=` operator chains continuations cleanly; Rust requires explicit method calls or helper functions.

## Exercises

1. Implement `cont_bind` that sequences two `Cont<R, A>` values (monadic bind / `>>=`).
2. Implement a CPS Fibonacci that is stack-safe by expressing it as a continuation chain.
3. Use the continuation monad to implement a depth-limited tree search that exits early when the depth limit is reached.
