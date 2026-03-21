📖 **[View on hightechmind.io →](https://hightechmind.io/rust/582-pattern-tuple-matching)**

---

# Tuple Pattern Matching
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Many decisions depend on the combination of multiple conditions. FizzBuzz is the canonical example: the output depends on two independent Boolean conditions. Without tuple matching, you need nested `if` statements. Matching on a tuple `(cond1, cond2)` expresses the decision matrix declaratively: each arm covers exactly one combination. This pattern is used in state transition tables, game logic, protocol state machines, and any logic where multiple independent conditions determine the outcome.

## Learning Outcomes

- How `match (a, b) { (true, true) => ... }` matches all combinations of two conditions
- How tuple patterns scale to three or more conditions
- How to combine tuples with other patterns: `match (opt_a, opt_b) { (Some(a), Some(b)) => ... }`
- How `_` in tuple positions allows partial matching: `(true, _) => ...`
- Where tuple matching replaces nested if/else: FizzBuzz, state transitions, validation matrices

## Rust Application

`fizzbuzz(n: u32)` matches `(n % 3 == 0, n % 5 == 0)` — all four Boolean combinations cleanly expressed. `fizzbuzz_if` shows the equivalent `if/else` chain — more verbose and harder to verify complete coverage. Tuple matching on `Option` pairs: `match (opt_a, opt_b) { (Some(a), Some(b)) => ..., (Some(a), None) => ..., (None, Some(b)) => ..., (None, None) => ... }`. The tuple is created inline — no temporary variable needed.

Key patterns:
- `match (a, b) { (true, true) => ... }` — two-condition matrix
- `match (opt_a, opt_b)` — Option combination dispatch
- `(val, _)` — partial match ignoring one dimension
- Three-way: `match (a, b, c)` — 8-combination matrix (use guards to collapse)

## OCaml Approach

OCaml tuple pattern matching is identical:

```ocaml
let fizzbuzz n = match (n mod 3 = 0, n mod 5 = 0) with
  | (true, true) -> "FizzBuzz"
  | (true, false) -> "Fizz"
  | (false, true) -> "Buzz"
  | (false, false) -> string_of_int n
```

This is one of the most natural examples of OCaml pattern matching — the code reads exactly like a truth table.

## Key Differences

1. **Syntax**: Rust `match (a, b)` and OCaml `match (a, b) with` are nearly identical — the pattern is universal across ML-family languages.
2. **Decision matrix readability**: Both languages make the decision matrix explicit and self-documenting; imperative `if/else` chains obscure the structure.
3. **Exhaustiveness**: Both compilers verify all `(bool, bool)` combinations are covered — adding a case for `(_, _)` or relying on exhaustiveness is clear.
4. **Tuple creation cost**: Rust tuples are stack-allocated — the `(a, b)` expression has zero heap overhead; OCaml tuples are heap-allocated GC values.

## Exercises

1. **Three-way FizzBuzz**: Extend FizzBuzz to also handle multiples of 7 as "Bazz" — match on `(n%3==0, n%5==0, n%7==0)` and use `_` to collapse irrelevant combinations.
2. **State + event matrix**: Implement `fn transition(state: State, event: Event) -> State` for a traffic light using tuple matching on `(state, event)`.
3. **Option matrix**: Write `fn combine(a: Option<i32>, b: Option<i32>) -> Option<i32>` that returns `Some(a + b)` if both are `Some`, `Some(a)` if only `a`, `Some(b)` if only `b`, `None` otherwise.
