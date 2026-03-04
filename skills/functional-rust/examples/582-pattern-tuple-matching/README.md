# 582: Tuple Pattern Matching

**Difficulty:** 2  **Level:** Beginner

Match on multiple values simultaneously by wrapping them in a tuple.

## The Problem This Solves

Sometimes a decision depends on the combination of two or more values, not either one alone. FizzBuzz depends on divisibility by 3 *and* 5. A traffic light transition depends on the current light *and* whether it's an emergency. A comparison result depends on `a > b` *and* `a < b` together.

Writing this as nested `if/else` or nested `match` statements is verbose and error-prone. Tuple patterns solve it: combine the values into a tuple, then match on the tuple. Every combination of values can be a distinct arm. The compiler checks exhaustiveness across all combinations.

This is the Rust equivalent of matching on a product of cases — the same technique that makes FizzBuzz elegant in functional languages.

## The Intuition

A tuple is just multiple values grouped together. Matching on a tuple is matching on all of them simultaneously. `match (a % 3 == 0, a % 5 == 0)` creates a `(bool, bool)` tuple; the four arms cover all four combinations of `(true/false, true/false)`. Wildcards (`_`) let you collapse cases: `(_, true)` matches any light in emergency mode.

The compiler guarantees you've covered every combination. Add a new variant to one of the enums? The match breaks — the compiler tells you exactly which combinations are now missing.

## How It Works in Rust

1. **Boolean product** — `match (n % 3 == 0, n % 5 == 0) { (true, true) => ..., (true, false) => ..., ... }` — four exhaustive arms.
2. **Enum + bool** — `match (light, emergency) { (_, true) => ..., (Light::Red, false) => ..., ... }` — wildcard collapses "any light in emergency" into one arm.
3. **Comparison result** — `match (a > b, a < b) { (true, false) => "gt", (false, true) => "lt", _ => "eq" }` — cleaner than nested ifs.
4. **Exhaustiveness** — the compiler verifies all combinations are covered; missing a case is a compile error, not a runtime bug.
5. **Guards in tuple arms** — `(Light::Green, false) if timer_expired => Light::Yellow` — guards work inside tuple arms.

## What This Unlocks

- Express multi-condition dispatch as a clean, exhaustive table instead of nested conditionals.
- Catch missing cases at compile time when new enum variants are added.
- Write FizzBuzz, state machines, and protocol logic in a form that reads like a truth table.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Tuple pattern match | `match (a, b) with (true, true) -> ...` — identical | `match (a, b) { (true, true) => ... }` — same pattern |
| Exhaustiveness | Compiler warns on missing cases | Compile error on missing cases |
| Wildcard in tuple | `(_, true) -> ...` | `(_, true) => ...` |
| Multi-value dispatch | Product types matched naturally | Same; no extra syntax needed |
