📖 **[View on hightechmind.io →](https://hightechmind.io/rust/577-pattern-irrefutable)**

---

# Irrefutable vs Refutable Patterns

## Problem Statement

Rust distinguishes between patterns that always succeed (irrefutable) and those that might fail (refutable). `let` bindings, function parameters, and `for` loops require irrefutable patterns — patterns that cannot fail to match. `if let`, `while let`, and `match` arms accept refutable patterns. This distinction is enforced at compile time: using a refutable pattern in a `let` binding is an error. Understanding this rule explains compiler errors and guides which construct to use for which kind of pattern.

## Learning Outcomes

- What irrefutable means: the pattern always matches any value of the type
- What refutable means: the pattern might not match (e.g., `Some(x)` on `Option<T>`)
- Why `let x = 5;`, `let (a, b) = pair;` are irrefutable
- Why `let Some(x) = opt;` is a compile error (refutable in `let`)
- How `let-else`, `if let`, and `match` are the correct contexts for refutable patterns

## Rust Application

Irrefutable: `let x = 5;`, `let (a, b) = (1, 2);`, `let Point { x, y } = point;` — these always match. Refutable: `Some(x)`, `Ok(v)`, `[a, ..]`, `1 | 2 | 3` — these might not match every value. `let Some(x) = opt;` is allowed only with `let-else` or as `let-else`. Function parameters must be irrefutable: `fn f((a, b): (i32, i32))` works; `fn f(Some(x): Option<i32>)` does not.

Key patterns:
- Irrefutable: variable, `_`, tuple `(a, b)`, struct `Point { x, y }`, tuple struct `Pair(a, b)`
- Refutable: `Some(x)`, `Ok(v)`, `Err(e)`, `[first, ..]`, integer literals, enum variants
- Irrefutable required in: `let`, `for`, function parameters
- Refutable allowed in: `if let`, `while let`, `match`, `let-else`

## OCaml Approach

OCaml has the same distinction but does not enforce it at the syntax level — partial `let` patterns generate a warning rather than a compile error:

```ocaml
let Some x = Some 5 in x  (* warning: this is partial *)
let (a, b) = (1, 2) in a + b  (* fine — always matches *)
```

## Key Differences

1. **Enforcement**: Rust makes using refutable patterns in `let` a hard compile error; OCaml issues a warning for partial `let` patterns but allows them.
2. **`let-else` for refutable**: Rust provides `let-else` as the sanctioned way to use refutable patterns in `let` position with a mandatory fallback; OCaml has no equivalent — you use `match`.
3. **Function parameters**: Rust function parameters require irrefutable patterns; OCaml function parameters can be partial patterns (with a warning).
4. **Comprehension**: Understanding irrefutable vs refutable helps predict which construct to use; in OCaml, the distinction is stylistic rather than structural.

## Exercises

1. **Classify patterns**: For each pattern below, state whether it is irrefutable or refutable: `(a, b)`, `Some(x)`, `x`, `_`, `[h, ..]`, `Point { x, y }`, `Ok(v)`, `1 | 2 | 3`.
2. **Fix the error**: Take `let Some(x) = maybe_value;` and fix it using both `let-else` and `if let` — explain the semantic difference between the two fixes.
3. **Irrefutable tuple**: Write `fn sum_pair((a, b): (i32, i32)) -> i32 { a + b }` and `fn sum_triple((a, b, c): (i32, i32, i32)) -> i32` — verify parameter destructuring works for tuples.
