📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1007-result-combinators)**

---

# 1007-result-combinators — Result Combinators
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Chaining fallible operations without deeply nested pattern matches is a core challenge in error-handling design. In C, every call site checks a return code, creating pyramids of conditionals. Haskell's `Either` monad and OCaml's `Result` type both solve this by providing combinators that thread success values through a pipeline while propagating errors automatically.

Rust's `Result<T, E>` ships with a rich set of combinators: `map`, `and_then`, `map_err`, `or_else`, and `unwrap_or_else`. These methods let you compose fallible computations as cleanly as iterator chains, desugaring to the same match logic you would write by hand.

## Learning Outcomes

- Understand how `and_then` implements monadic bind (flatmap) for `Result`
- Apply `map` to transform success values without unwrapping
- Use `map_err` to convert or annotate error types
- Chain `or_else` to substitute a fallback `Result` on failure
- Replace explicit `match` blocks with expressive combinator pipelines

## Rust Application

`src/lib.rs` demonstrates three composition styles. `process_chain` uses `and_then` to pass the parsed integer into `double_if_positive`, then `map` to convert the final `i64` to a `String`. `process_with_fallback` appends `unwrap_or_else` so the pipeline always resolves to a `String`. `process_or_else` uses `or_else(|_| Ok(0))` to recover from any error with a sentinel value.

The combinator pattern avoids re-binding intermediate results and keeps error propagation implicit, matching the ergonomics of `?` but without early return.

## OCaml Approach

OCaml's `Result` module provides `Result.map`, `Result.bind` (equivalent to `and_then`), and `Result.map_error`. The `|>` pipeline operator makes chaining natural:

```ocaml
let process s =
  parse_int s
  |> Result.bind double_if_positive
  |> Result.map string_of_int
```

OCaml expresses combinators as regular functions passed via `|>`, while Rust uses method chaining on the `Result` value.

## Key Differences

1. **Method vs function syntax**: Rust uses `result.and_then(f)` method chaining; OCaml uses `Result.bind f result` piped with `|>`.
2. **Error type unification**: Rust requires both branches of `and_then` to share the same `E` type; OCaml is structurally typed and more flexible.
3. **`From` conversion**: Rust's `?` auto-converts error types via `From`; OCaml combinators require explicit `Result.map_error` for type adaptation.
4. **Ownership**: Rust combinators consume the `Result` value by move; OCaml passes values through the GC with no ownership concern.

## Exercises

1. Add a `clamp_to_range(n: i64, min: i64, max: i64) -> Result<i64, String>` function and insert it into `process_chain` between parsing and doubling.
2. Rewrite `process_with_fallback` using the `?` operator inside a helper function instead of combinator chaining. Verify all tests still pass.
3. Implement a `map2` function that takes two `Result<T, E>` values and a combining function, returning `Result<U, E>`. Use it to add two parsed integers.
