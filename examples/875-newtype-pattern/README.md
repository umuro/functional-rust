📖 **[View on hightechmind.io →](https://hightechmind.io/rust/875-newtype-pattern)**

---

# 875-newtype-pattern — Newtype Pattern
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Type aliases create convenient shorthand but provide no safety guarantees: `type UserId = u64` means `UserId` and `u64` are interchangeable. The newtype pattern wraps a type in a single-field struct, creating a distinct type with identical runtime representation. Haskell's `newtype` keyword formalizes this with zero-cost guarantee. In Rust, tuple structs like `struct UserId(u64)` achieve the same: you cannot accidentally pass a `UserId` where an `OrderId` is expected, even though both wrap `u64`. This pattern is standard in domain-driven design for preventing primitive obsession and encoding domain invariants.

## Learning Outcomes

- Create newtypes using Rust tuple structs to prevent type confusion
- Implement validation-enforcing smart constructors that return `Result`
- Understand the difference between newtype (new type) and type alias (same type)
- Use `Display` and `Debug` implementations to give newtypes meaningful output
- Compare Rust tuple structs with OCaml's abstract module types for the same purpose

## Rust Application

`UserId(u64)` and `OrderId(u64)` are distinct types despite identical runtime layout. `process_order(order: OrderId, user: UserId)` cannot be called with the arguments reversed — the compiler rejects it. `Email(String)` demonstrates validation: `Email::new` returns `Result<Email, String>`, ensuring only valid emails exist. The `value()` accessor exposes the inner data for legitimate use. Implementing `Display` gives user-friendly output while `Debug` gives developer output.

## OCaml Approach

OCaml achieves the same effect through abstract module types. `module UserId: sig type t; val create: int -> t; val value: t -> int end = struct type t = int; ... end` makes `UserId.t` opaque outside the module. The inner `int` is inaccessible directly, and `UserId.t` and `OrderId.t` are incompatible types. OCaml's `with type` constraint can selectively expose the type alias when needed. The pattern is more verbose than Rust's tuple struct but achieves the same compile-time safety.

## Key Differences

1. **Zero-cost abstraction**: Both Rust tuple structs and OCaml abstract types have zero runtime overhead compared to the wrapped type.
2. **Syntax brevity**: Rust `struct Email(String)` is three words; OCaml needs a full module definition.
3. **Deriving**: Rust can `#[derive(Debug, Clone, Copy, PartialEq)]`; OCaml requires explicit functor application or manual implementation.
4. **Inner access**: Rust's `.0` field accessor breaks abstraction (unless private); OCaml abstract types are more strictly opaque without the module's `value` function.

## Exercises

1. Implement `struct Percentage(f64)` with a smart constructor that validates the value is between 0.0 and 100.0.
2. Add arithmetic for `Quantity<Unit>` (a newtype-over-f64) that only allows same-unit addition but returns a plain `f64` for division.
3. Implement `struct NonNegative(i64)` and derive `Add`, `Sub` (returning `Result` on underflow), and `Mul` for it.
