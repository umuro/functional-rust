📖 **[View on hightechmind.io →](https://hightechmind.io/rust/741-parse-dont-validate)**

---

# 741-parse-dont-validate — Parse Don't Validate
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

"Validate then use" is the traditional approach: accept raw input, check it, and then use the raw value downstream, relying on programmers to remember to validate first. Parse-don't-validate flips this: you can only construct a typed value by successfully parsing it, making invalid states unrepresentable. Coined by Alexis King in 2019, this principle is used in Haskell's `text` library, Rust's `std::net::IpAddr`, and almost every well-designed API that accepts structured input.

## Learning Outcomes

- Create types with private fields that can only be constructed via a parsing function
- Model a `NonEmptyString`, `Email`, and `BoundedInt` that are always valid once constructed
- Return `Result<ValidType, ParseError>` instead of `Result<String, Error>` from parse functions
- Understand why private fields are essential — they prevent bypassing validation
- See how composed types (`UserProfile`) inherit validity from their components

## Rust Application

`NonEmptyString(String)` has a private inner field. The only constructor is `NonEmptyString::parse(s: &str) -> Result<Self, ParseError>`, which checks for emptiness before wrapping. `Email(String)` requires an `@` symbol and a non-empty domain. `BoundedInt<const LO: i64, const HI: i64>` uses const generics to encode the valid range in the type. Once constructed, all accessors are infallible — no further validation needed downstream.

## OCaml Approach

OCaml uses abstract types in modules to enforce the same invariant. A module `Email : sig type t val parse : string -> (t, error) result val to_string : t -> string end` ensures only `parse` can create an `Email.t`. Jane Street's `Validated` module and `Validated_sexp` follow this exact pattern. OCaml's module system makes it natural — the implementation type is hidden behind the signature.

## Key Differences

1. **Mechanism**: Rust uses private struct fields within a crate; OCaml uses abstract module types to hide the representation.
2. **Const generics**: Rust's `BoundedInt<LO, HI>` encodes bounds in the type itself; OCaml requires runtime bounds stored in the value or a functor argument.
3. **Error accumulation**: Rust's `?` operator short-circuits on first error; OCaml's `Applicative` validation pattern accumulates all errors before returning.
4. **Ecosystem**: Rust's `garde`, `validator`, and `nutype` crates generate parse-don't-validate types from derive macros; OCaml has `ppx_validate`.

## Exercises

1. Implement a `PhoneNumber` type that only accepts E.164 format (`+` followed by 7–15 digits) via a `parse` function returning `Result<PhoneNumber, ParseError>`.
2. Create a `Url` newtype that validates scheme, host, and optional port, exposing typed accessors for each component.
3. Write a `UserProfile::parse(name: &str, email: &str, age: i64) -> Result<UserProfile, Vec<ParseError>>` that accumulates all validation errors instead of returning on the first failure.
