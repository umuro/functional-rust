📖 **[View on hightechmind.io →](https://hightechmind.io/rust/576-pattern-matches-macro)**

---

# matches! Macro

## Problem Statement

Boolean checks against patterns appear frequently in filters, validations, and conditions. Before `matches!`, Rust code needed full `match val { Pattern => true, _ => false }` expressions for this — verbose for a single-line predicate. The `matches!` macro (stabilized in Rust 1.42) collapses this to `matches!(val, Pattern)`, supporting or-patterns, guards, and destructuring. It is the most concise way to test whether a value matches a pattern without extracting data.

## Learning Outcomes

- How `matches!(val, Pattern)` returns `bool` without a full `match` expression
- How `matches!(val, A | B | C)` tests multiple alternatives
- How `matches!(val, Pattern if condition)` adds a guard
- When to use `matches!` vs `if let` vs explicit `match`
- Where `matches!` is most useful: `filter`, `all`, `any`, predicate functions

## Rust Application

`is_active(status: &Status) -> bool` uses `matches!(status, Status::Active)`. `is_usable` uses `matches!(status, Status::Active | Status::Pending)`. With guards: `matches!(n, x if x > 0 && x < 100)`. In iterator context: `statuses.iter().filter(|s| matches!(s, Status::Active)).count()`. The macro expands to a `match` expression at compile time — zero runtime overhead.

Key patterns:
- `matches!(val, Pattern)` — boolean pattern test
- `matches!(val, P1 | P2)` — or-pattern test
- `matches!(val, Pattern if guard)` — pattern with guard
- In `filter`: `.filter(|x| matches!(x, Target))`

## OCaml Approach

OCaml achieves the same with a function:

```ocaml
let is_active = function Status.Active -> true | _ -> false
let is_usable = function Active | Pending -> true | _ -> false
(* or inline: *)
let count_active statuses = List.length (List.filter (function Active -> true | _ -> false) statuses)
```

## Key Differences

1. **Macro vs function**: Rust `matches!` is a procedural macro; OCaml uses higher-order functions or `function` shorthand.
2. **Guard support**: `matches!(val, P if cond)` includes a guard; OCaml's `function P when cond -> true | _ -> false` is the equivalent.
3. **Or-patterns**: `matches!(val, A | B)` uses Rust or-pattern syntax; OCaml's `function A | B -> true` is identical in concept.
4. **Expansion**: `matches!` expands to `match val { Pattern => true, _ => false }` — identical performance to writing it manually.

## Exercises

1. **HTTP filter**: Write a predicate `fn is_success(code: u16) -> bool` using `matches!(code, 200..=299)` and use it to filter a list of response codes.
2. **Complex guard**: Implement `fn is_valid_move(event: &Event) -> bool` using `matches!(event, Event::Move { x, y } if *x >= 0 && *y >= 0)`.
3. **All/any combo**: Write `fn all_active(statuses: &[Status]) -> bool` and `fn any_banned(statuses: &[Status]) -> bool` using `matches!` inside `iter().all()` and `iter().any()`.
