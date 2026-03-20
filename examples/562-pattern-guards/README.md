📖 **[View on hightechmind.io →](https://hightechmind.io/rust/562-pattern-guards)**

---

# Pattern Guards

## Problem Statement

Patterns alone cannot express all matching conditions — sometimes you need to test an arbitrary Boolean expression in addition to the structural match. Pattern guards (`if condition` after a match arm pattern) fill this gap: they allow any expression as an additional condition, while keeping the pattern for structural decomposition. Guards are used heavily in parsers, compilers, and game logic where the same structural form can have different meanings depending on computed values.

## Learning Outcomes

- How `match n { x if x < 0 => ... }` combines a binding with a Boolean guard
- How guards can reference bound variables from the pattern
- How destructured values can be tested in guards: `(x, y) if x == y`
- Why guards interact with exhaustiveness checking: the compiler cannot verify guards cover all cases
- Where guards are common: number categorization, range checks, coordinate classification

## Rust Application

`categorize(n: i32)` uses `x if x < 0 => "negative"` — binds `x` and tests the condition. `check_range(n, min, max)` uses `x if x >= min && x <= max`. `process_point(point: (i32, i32))` uses `(x, y) if x == y => "diagonal"` — destructuring combined with guard. Guards can reference any in-scope variable, not just the bound pattern variables.

Key patterns:
- `arm_pattern if condition => expr` — guard in match arm
- `(x, y) if x == y` — guard testing destructured fields against each other
- Guards do not affect exhaustiveness — `_` arm still needed

## OCaml Approach

OCaml uses `when` as its guard keyword:

```ocaml
let categorize n = match n with
  | x when x < 0 -> "negative"
  | 0 -> "zero"
  | x when x < 10 -> "small positive"
  | _ -> "large positive"
```

The semantics are identical to Rust's `if` guards.

## Key Differences

1. **Keyword**: Rust uses `if` in guards (`arm if cond`); OCaml uses `when` (`arm when cond`).
2. **Exhaustiveness**: Both Rust and OCaml cannot statically verify guard coverage — compilers assume guards might fail and require a fallback arm.
3. **Variable scope**: In both languages, guard conditions can reference variables bound in the pattern; the guard sees the destructured values.
4. **NLL interaction**: Rust guards can borrow values from the matched expression — NLL ensures borrows in guards do not conflict with the arm body.

## Exercises

1. **FizzBuzz with guards**: Implement FizzBuzz using pattern guards: `match n { n if n % 15 == 0 => "FizzBuzz", ... }` without using `if/else`.
2. **Point classification**: Write `fn classify_point(x: f64, y: f64) -> &'static str` using nested match with guards to distinguish origin, axes, quadrants, and far points (distance > 10).
3. **Guard with enum**: Create a `Temperature` enum with `Celsius(f64)` and `Fahrenheit(f64)` and use guards to classify as "freezing", "cold", "warm", "hot" across both variants.
