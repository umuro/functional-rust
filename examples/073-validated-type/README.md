📖 **[View on hightechmind.io →](https://hightechmind.io/rust/073-validated-type)**

---

# 073 — Validated Types (Parse, Don't Validate)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

"Parse, don't validate" (Lexi Lambda, 2019) is a design principle: instead of checking a precondition and continuing with the raw value, parse the input into a type that PROVES the precondition is satisfied. `NonEmptyString` cannot be empty by construction; `PositiveInt` cannot be negative. The type system enforces invariants, not runtime checks.

This pattern eliminates entire categories of defensive programming. If a function takes `NonEmptyString`, callers cannot accidentally pass an empty string — the type system prevents it. Applied in Rust's type system, `Email`, `PositiveInt`, `BoundedString<1, 50>` make invalid states unrepresentable. Used in `nutype`, `validator`, and custom domain types throughout production Rust code.

## Learning Outcomes

- Define newtypes with private fields so construction is controlled
- Implement fallible constructors returning `Option<T>` or `Result<T, E>`
- Understand that the type guarantees the invariant everywhere — no re-checking needed
- Apply the "parse, don't validate" principle: validate once at the boundary, then trust
- Use these validated types in function signatures to express preconditions

## Rust Application

`NonEmptyString(String)` has a private `0` field — only `NonEmptyString::new(s)` can construct it, and it returns `None` if `s.is_empty()`. Once you have a `NonEmptyString`, its length is always >= 1 — no further checks needed. `PositiveInt(u32)` works similarly: `new(n: i32) -> Option<Self>` validates and the type guarantees positivity. Functions taking `NonEmptyString` or `PositiveInt` express their preconditions in the type signature.

## OCaml Approach

OCaml's approach uses abstract types in modules: `module NonEmptyString : sig type t val of_string : string -> t option val to_string : t -> string end = struct type t = string let of_string s = if s = "" then None else Some s let to_string s = s end`. The `sig` hides the internal representation, preventing direct construction of `t`.

## Key Differences

1. **Private fields**: Rust uses a tuple struct with no `pub` on the field: `struct NonEmptyString(String)`. OCaml uses abstract types in a module signature to hide the representation.
2. **Newtype vs abstract type**: Rust's newtype pattern is the typical approach. OCaml's module-based abstract types provide the same guarantee but through the module system.
3. **`nutype` crate**: Rust's `nutype` crate generates validated newtypes via derive macros. OCaml has no equivalent — modules are written manually.
4. **Zero-cost**: Both approaches are zero-cost — no runtime overhead for the wrapper. Rust's tuple struct has the same layout as the inner type.

## Exercises

1. **Email type**: Define `Email(String)` with `Email::new(s: &str) -> Option<Email>` that validates the string contains exactly one `@` and a non-empty domain. Use it in a `send_email(to: Email, subject: NonEmptyString, body: &str)` function.
2. **Bounded string**: Write `BoundedString<const MIN: usize, const MAX: usize>(String)` using const generics. `new` validates `MIN <= s.len() <= MAX`.
3. **Range value**: Write `RangeValue<const MIN: i32, const MAX: i32>(i32)` with const generics. Implement `Add<RangeValue<MIN, MAX>> for RangeValue<MIN, MAX>` that saturates at the bounds.
