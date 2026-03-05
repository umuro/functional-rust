# Example 988: String.map and String.init — Character-level Operations

**Difficulty:** ⭐
**Category:** stdlib-string
**OCaml Source:** OCaml Standard Library — `String.map`, `String.init`

## Problem Statement

Transform strings character by character using a mapping function (`String.map`), and build strings from scratch by generating each character from its index (`String.init`).

## Learning Outcomes

- How OCaml's `String.map` maps directly to Rust's `.chars().map(f).collect::<String>()`
- How OCaml's `String.init` maps to Rust's `(0..n).map(f).collect::<String>()`
- Using range-based iterator chains to construct strings without explicit loops
- Implementing a cipher (ROT-13) as a pure character transformation function

## OCaml Approach

OCaml provides `String.map : (char -> char) -> string -> string` and `String.init : int -> (int -> char) -> string` as first-class library functions. A function is passed as a value and applied to each character or index. Strings are immutable values; the result is always a fresh string.

## Rust Approach

Rust strings are UTF-8 and accessed through iterators. `.chars()` yields `char` values (Unicode scalar values), `.map(f)` applies the transformation, and `.collect::<String>()` assembles the result. For `String.init`, a range `(0..n)` serves as the index source — no special function is needed.

## Key Differences

1. **Character access:** OCaml has direct `String.map`; Rust uses `.chars().map().collect()` — explicit but composable.
2. **String construction:** OCaml's `String.init` is built-in; Rust uses a range iterator `(0..n).map(f).collect()`.
3. **Encoding:** OCaml strings are byte arrays; Rust strings are UTF-8, so `.chars()` yields full Unicode `char` values.
4. **Type annotation:** Rust needs `.collect::<String>()` or type inference to know the target collection type.
