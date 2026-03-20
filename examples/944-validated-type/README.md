**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

[validated-type on hightechmind.io](https://hightechmind.io/posts/functional-rust/validated-type)

---

## Problem Statement

Use Rust's type system to enforce domain invariants at construction time via smart constructors. Implement `NonEmptyString` and `PositiveInt` as opaque newtypes whose inner fields are private. Consumers can only obtain values of these types by calling a validated constructor that returns `Result`, making invalid states unrepresentable at compile time.

## Learning Outcomes

- Implement opaque newtypes with private fields to prevent direct construction of invalid values
- Write smart constructors that return `Result<T, String>` and enforce invariants at the boundary
- Expose safe derived operations that preserve invariants without re-checking (e.g., `NonEmptyString::concat` is always non-empty)
- Implement `Display` for newtype wrappers
- Understand the OCaml module system's role in encapsulation vs Rust's module visibility (`pub` vs no modifier)

## Rust Application

```rust
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonEmptyString(String);  // private field

impl NonEmptyString {
    pub fn create(s: &str) -> Result<Self, String> {
        if !s.is_empty() {
            Ok(NonEmptyString(s.to_string()))
        } else {
            Err("string must be non-empty".to_string())
        }
    }

    pub fn value(&self) -> &str { &self.0 }
    pub fn len(&self) -> usize { self.0.len() }

    // concat is always non-empty — no validation needed
    pub fn concat(&self, other: &NonEmptyString) -> NonEmptyString {
        NonEmptyString(format!("{}{}", self.0, other.0))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositiveInt(i64);  // private field

impl PositiveInt {
    pub fn create(n: i64) -> Result<Self, String> {
        if n > 0 { Ok(PositiveInt(n)) } else { Err(format!("{} is not positive", n)) }
    }

    pub fn value(self) -> i64 { self.0 }
    pub fn add(self, other: Self) -> Self { PositiveInt(self.0 + other.0) }
    pub fn mul(self, other: Self) -> Self { PositiveInt(self.0 * other.0) }
}
```

The private field is the critical detail. Code outside the module cannot call `NonEmptyString("".to_string())` because the tuple struct constructor is not accessible. The only way in is through `create`, which runs the invariant check.

Derived operations like `add` and `mul` on `PositiveInt` skip validation because `positive + positive` and `positive * positive` are provably positive — the invariant is preserved structurally. `concat` on `NonEmptyString` is similarly always safe.

## OCaml Approach

```ocaml
(* Opaque type via module signature *)
module NonEmptyString : sig
  type t
  val create : string -> (t, string) result
  val value : t -> string
  val concat : t -> t -> t
end = struct
  type t = string
  let create s =
    if String.length s > 0 then Ok s
    else Error "string must be non-empty"
  let value s = s
  let concat a b = a ^ b
end

module PositiveInt : sig
  type t
  val create : int -> (t, string) result
  val value : t -> int
  val add : t -> t -> t
  val mul : t -> t -> t
end = struct
  type t = int
  let create n =
    if n > 0 then Ok n
    else Error (string_of_int n ^ " is not positive")
  let value n = n
  let add a b = a + b
  let mul a b = a * b
end
```

OCaml uses the module system's signature/implementation split to create opaque types. The signature exposes `type t` without revealing that `t = string`, so external code cannot construct or pattern-match on values of `t` directly.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Encapsulation | Private struct field (not `pub`) | Abstract type in module signature |
| Constructor gating | Smart constructor returning `Result` | Same pattern |
| Safe derived ops | Methods that skip re-validation | Same; compiler cannot verify but convention holds |
| `Copy` semantics | `#[derive(Copy)]` for cheap types | Value semantics throughout (no `Copy` concept) |
| Ordering | `#[derive(PartialOrd, Ord)]` — lexicographic by default | `compare` function or `module Ord` |

This pattern — make invalid states unrepresentable — is one of the most effective uses of static type systems in both languages. The cost is a single validation at the boundary; the benefit is zero-cost safety everywhere the type is used.

## Exercises

1. Implement `EmailAddress` as an opaque type whose `create` validates presence of `@` and at least one `.` after it.
2. Add `NonEmptyString::split_first() -> (char, &str)` — guaranteed safe because the string is non-empty.
3. Implement `BoundedInt { value: i64, min: i64, max: i64 }` with a constructor that validates range.
4. Add `serde` serialization that validates on deserialization (implement `Deserialize` with invariant check).
5. Write a function that takes two `PositiveInt`s and computes their ratio as `f64` — always safe (no division by zero).
