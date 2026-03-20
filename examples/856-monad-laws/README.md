📖 **[View on hightechmind.io →](https://hightechmind.io/rust/856-monad-laws)**

---

# Monad Laws
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Like functor laws, monad laws are algebraic contracts that ensure `bind` (and_then) behaves predictably. The three laws — left identity, right identity, and associativity — guarantee that `return` is a neutral element for bind and that bind is associative (the order of parenthesizing a chain doesn't matter). Code that refactors monadic chains relies on these laws: splitting a long chain into helper functions, extracting common subexpressions, and equational reasoning in tests all depend on them. Violating monad laws leads to subtle bugs: computations that behave differently depending on how they were composed, breaking algebraic reasoning.

## Learning Outcomes

- State the left identity law: `return(a).and_then(f) == f(a)` — wrapping then binding is just applying
- State the right identity law: `m.and_then(return) == m` — binding with wrap is identity
- State the associativity law: `(m.and_then(f)).and_then(g) == m.and_then(|x| f(x).and_then(g))`
- Verify all three laws for `Option` and `Result` with concrete examples
- Understand why violations matter: they break equational reasoning and refactoring safety

## Rust Application

```rust
fn double(x: i32) -> Option<i32> { Some(x * 2) }
fn is_even(x: i32) -> Option<i32> { if x % 2 == 0 { Some(x) } else { None } }

// Left identity: Some(a).and_then(f) == f(a)
assert_eq!(Some(3).and_then(double), double(3)); // Some(6) == Some(6)

// Right identity: m.and_then(Some) == m
assert_eq!(Some(3).and_then(Some), Some(3));     // Some(3) == Some(3)
assert_eq!(None::<i32>.and_then(Some), None);    // None == None

// Associativity: (m >>= f) >>= g == m >>= (|x| f(x) >>= g)
let left  = Some(3).and_then(double).and_then(is_even);
let right = Some(3).and_then(|x| double(x).and_then(is_even));
assert_eq!(left, right); // Both Some(6)
```

The three assertions are directly executable proofs of the laws for concrete values. Testing with both `Some` and `None` inputs covers both cases. The associativity law shows that nested `and_then` can be flattened, which is the law enabling `let*` do-notation refactoring. Rust's `assert_eq!` makes law verification testable rather than just theoretical.

## OCaml Approach

OCaml verifies monad laws with `assert`: `assert (Option.bind (Some a) f = f a)` for left identity. The right identity: `assert (Option.bind m Option.some = m)`. Associativity: `assert (Option.bind (Option.bind m f) g = Option.bind m (fun x -> Option.bind (f x) g))`. OCaml's `QCheck` property tester generates random monadic values and functions, verifying laws across many inputs automatically. The `>>= ` operator makes the laws more readable.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Return/pure | `Some(x)` or `Ok(x)` | `Option.some x` or `Result.ok x` |
| Bind | `and_then` | `Option.bind` |
| Law notation | `assert_eq!` tests | `assert` or `QCheck` |
| Associativity | Parenthesization independence | Same |
| Verification | Concrete examples or proptest | `QCheck` preferred |
| Law importance | Enables safe refactoring | Same |

## Exercises

1. Write a `proptest` property test that verifies the associativity law for `Option<i32>` with random functions and values.
2. Construct a "broken" monad implementation that violates the right identity law — change `and_then` to add a side effect — and observe the violation.
3. Verify all three laws for `Result<i32, String>` with concrete examples covering both `Ok` and `Err` cases.
4. Show that the associativity law justifies extracting a sub-chain into a helper function without changing semantics.
5. Implement `do_notation` macro for `Option` using Rust macros that expands to `and_then` chains, enabled by the monad laws.
