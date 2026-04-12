# OCaml vs Rust: Leap Year

## Side-by-Side Code

### OCaml
```ocaml
let is_leap_year year =
  (year mod 400 = 0) ||
    (year mod 4 = 0 && year mod 100 <> 0)

let () =
  assert (is_leap_year 2000 = true);
  assert (is_leap_year 1900 = false);
  assert (is_leap_year 2024 = true);
  print_endline "ok"
```

### Rust (idiomatic)
```rust
pub fn is_leap_year(year: i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}
```

### Rust (explicit decomposition)
```rust
pub fn is_leap_year_explicit(year: i32) -> bool {
    let divisible_by_4   = year % 4   == 0;
    let divisible_by_100 = year % 100 == 0;
    let divisible_by_400 = year % 400 == 0;
    divisible_by_400 || (divisible_by_4 && !divisible_by_100)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Function signature | `val is_leap_year : int -> bool` | `fn is_leap_year(year: i32) -> bool` |
| Integer type | `int` (63-bit on 64-bit systems) | `i32` (explicit 32-bit signed) |
| Boolean operators | `&&`, `\|\|`, `not` | `&&`, `\|\|`, `!` |
| Modulus operator | `mod` (keyword) | `%` (operator) |
| Inequality | `<>` | `!=` |

## Key Insights

1. **Operator vs. keyword**: OCaml uses `mod` as an infix keyword; Rust uses `%` as an operator — both compute the remainder.
2. **Operator precedence**: The parenthesization is the same in both languages: the OR has lower precedence than AND; both require explicit parentheses to clarify the three-part rule.
3. **Naming booleans explicitly**: The "explicit" Rust version names each condition (`divisible_by_4`, etc.) — OCaml supports this too via `let` bindings, but the one-liner is idiomatic in both.
4. **Type width**: OCaml's `int` is machine-width (63-bit on 64-bit); Rust forces you to choose `i32` or `i64` explicitly, which matters for domain modeling.
5. **Pure function**: Both implementations are pure — the same input always gives the same output, with no side effects. This enables easy unit testing and reasoning.

## When to Use Each Style

**Use the one-liner Rust version when:** the logic is clear in a single line and readability is not compromised; idiomatic for boolean predicates.
**Use the explicit decomposition when:** the rule has three or more named parts and clarity is paramount; especially useful in code review or educational contexts.
