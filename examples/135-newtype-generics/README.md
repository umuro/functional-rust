📖 **[View on hightechmind.io →](https://hightechmind.io/rust/135-newtype-generics)**

---

# Generic Newtype Patterns
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Primitive types like `String` or `u32` carry no domain meaning — a user ID and a product ID are both `u32`, but passing one where the other is expected is a bug the type checker won't catch. Newtypes wrap primitives in named types that are distinct at the type level, add invariants via smart constructors, and enable implementing external traits on otherwise opaque types. This is one of the most practical patterns in production Rust code.

## Learning Outcomes

- Understand the newtype pattern: a single-field tuple struct wrapping another type
- Learn validated newtypes that enforce invariants at construction time
- See the generic `Validated<T, V>` pattern parameterized by a validator trait
- Practice transparent newtypes via `Deref` to expose the inner API without boilerplate

## Rust Application

`Email(String)` and `Username(String)` are private newtypes with smart constructors that validate their invariants. Once constructed, callers can trust the invariant without rechecking. `Validated<T, V>` is a generic newtype parameterized by a `Validator` trait — the equivalent of an OCaml functor. `LoggedVec<T>` implements `Deref<Target = Vec<T>>` to expose all `Vec` methods while intercepting construction.

## OCaml Approach

OCaml uses modules for validated newtypes:
```ocaml
module Email : sig
  type t
  val make : string -> t option
  val to_string : t -> string
end = struct
  type t = string
  let make s = if String.contains s '@' then Some s else None
  let to_string s = s
end
```
The module signature hides the concrete type, preventing direct construction outside the module. This achieves the same invariant-enforcement as Rust's private tuple fields, but at the module granularity rather than the type granularity.

## Key Differences

1. **Abstraction boundary**: OCaml's module system is the mechanism for newtype abstraction (opaque module types); Rust uses private fields in tuple structs.
2. **Deref transparency**: Rust's `Deref` lets newtypes participate in auto-dereferencing for ergonomic access; OCaml has no equivalent — wrapper modules require explicit delegation.
3. **Trait orphan rules**: Rust newtypes can implement external traits (e.g., `Display` for `Vec<T>`); OCaml modules can implement signatures freely with no orphan restrictions.
4. **Generic validators**: Rust's `Validated<T, V: Validator>` is parameterized at the type level; OCaml achieves this via parameterized module functors.

## Exercises

1. Create a `NonEmptyString` newtype that rejects empty strings at construction and implements `Display` and `Deref<Target = str>`.
2. Implement `Validated<u8, RangeValidator>` where `RangeValidator` checks that a number falls within [0, 100].
3. Write a `SortedVec<T: Ord>` newtype that wraps `Vec<T>` and guarantees sorted order after each insertion.
