📖 **[View on hightechmind.io →](https://hightechmind.io/rust/056-monad-result)**

---

# 056 — Result as a Monad
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

`Result` is a monad: it satisfies the three monad laws (left identity, right identity, associativity) and provides `return` (wrapping in `Ok`) and `bind` (`and_then`). Recognizing `Result` as a monad explains why `and_then` chains and `?` feel so clean: they are a principled application of monadic sequencing, the same structure as `IO` in Haskell, `async/await` in most languages, and parser combinators.

The monad structure is what makes railway-oriented programming work: the "happy path" is the `Ok` track, errors get routed to the `Err` track, and `and_then` is the switch. Understanding this gives you the vocabulary to recognize and use the same pattern across different types (Option, Future, Iterator).

## Learning Outcomes

- Recognize `Result` as a monad with `Ok` as `return` and `and_then` as `bind`
- Build multi-step fallible pipelines using `and_then` (monadic bind)
- Compare `and_then` chaining with the `?` operator — both express the same computation
- Understand why `Result` is a monad but `Validation` (example 054) is not
- Use `map` (functor) and `and_then` (monad) as the complete Result transformation toolkit

- Sequence fallible operations with `Result::and_then` — the monadic bind that propagates errors automatically
- Verify monad laws: left identity `Ok(x).and_then(f) == f(x)`, right identity `r.and_then(Ok) == r`

## Rust Application

`compute_bind(s1, s2)` uses `parse_int(s1).and_then(|a| parse_int(s2).and_then(|b| safe_div(a, b)))` — explicit monadic bind. `compute_question(s1, s2)` rewrites as `let a = parse_int(s1)?; let b = parse_int(s2)?; safe_div(a, b)` — `?` desugars to the same computation. `pipeline(s)` chains `and_then` with `map`: parse, divide, add 1, multiply — each step may fail or transform.

## OCaml Approach

OCaml's result monad: `let ( >>= ) r f = Result.bind r f`. Then: `parse_int s1 >>= fun a -> parse_int s2 >>= fun b -> safe_div a b`. With `let*` (ppx_let): `let* a = parse_int s1 in let* b = parse_int s2 in safe_div a b`. Both forms are equivalent. OCaml's `>>=` operator for result is not in stdlib but is easily defined and widely used.

## Key Differences

1. **`>>=` vs `and_then`**: Haskell uses `>>=` as the bind operator. OCaml defines it by convention. Rust uses the method name `and_then`. All three are the same monad operation.
2. **Monad laws**: Left identity: `Ok(a).and_then(f) == f(a)`. Right identity: `r.and_then(Ok) == r`. Associativity: `r.and_then(f).and_then(g) == r.and_then(|x| f(x).and_then(g))`. Verify these in tests.
3. **`?` ergonomics**: Rust's `?` makes the monad pattern syntactically cheap — writing monadic code feels like imperative code. OCaml's `let*` achieves the same goal.
4. **Error type consistency**: Monadic `and_then` chains require a consistent error type `E`. Use `map_err` to normalize before chaining, or use `Box<dyn Error>` for heterogeneous chains.

1. **`Result` as a monad:** `Ok(x).and_then(f)` = `f(x)` and `Err(e).and_then(f)` = `Err(e)`. These are the monad laws for `Result` (return + bind). Verifying monad laws in tests builds confidence in error handling correctness.
2. **`?` as do-notation:** Haskell's `do { x <- action; ... }` is equivalent to Rust's `let x = action?`. Both are syntactic sugar for monadic bind.
3. **Sequencing fallible operations:** The power of the `Result` monad is sequencing: each step feeds its output to the next, and a single failure short-circuits the chain. No explicit error checking between steps.
4. **OCaml's `let*`:** With `ppx_let`, OCaml supports `let* x = fallible () in next x` as syntactic sugar for `Result.bind (fallible ()) (fun x -> next x)`. Equivalent to Rust's `?`.

## Exercises

1. **Monad laws test**: Write tests verifying the three monad laws for `Result<i32, String>`. For each law, construct a concrete case and assert equality.
2. **State monad**: Implement a `State<S, T>` type wrapping `impl Fn(S) -> (T, S)`. Implement `and_then` for it. Show how it enables stateful computation in a pure functional style.
3. **Continuation monad**: Implement `type Cont<R, T> = Box<dyn FnOnce(Box<dyn FnOnce(T) -> R>) -> R>`. Define `bind` and use it to express error handling in continuation-passing style (connects to example 099).

4. **Kleisli composition**: Implement `kleisli_compose<A, B, C, E>(f: impl Fn(A) -> Result<B, E>, g: impl Fn(B) -> Result<C, E>) -> impl Fn(A) -> Result<C, E>` — the "fish operator" `>=>` from Haskell.
5. **MonadPlus**: Implement `or_else_result<T, E>(r: Result<T, E>, alternative: impl FnOnce(E) -> Result<T, E>) -> Result<T, E>` — the `MonadPlus` recovery operation for `Result`.
