# OCaml vs Rust: Generic Newtype Patterns

## Side-by-Side Code

### OCaml

```ocaml
(* Algebraic variant as newtype *)
type email = Email of string

let email_of_string s =
  if String.contains s '@' then Some (Email s) else None

let string_of_email (Email s) = s

(* Functor-based generic wrapper *)
module type VALIDATOR = sig
  type t
  val validate : t -> bool
end

module Validated (V : VALIDATOR) = struct
  type t = V.t
  let create x = if V.validate x then Some x else None
end
```

### Rust (idiomatic newtypes)

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);

impl Email {
    pub fn new(s: &str) -> Result<Self, &'static str> {
        if s.contains('@') { Ok(Email(s.to_owned())) }
        else { Err("invalid email: missing '@'") }
    }
    pub fn as_str(&self) -> &str { &self.0 }
}

// Typed IDs — same underlying u64, completely distinct types
pub struct UserId(pub u64);
pub struct ProductId(pub u64);
```

### Rust (generic validated wrapper — functor parallel)

```rust
pub trait Validator<T> {
    type Error: fmt::Debug + fmt::Display;
    fn validate(value: &T) -> Result<(), Self::Error>;
}

pub struct Validated<T, V>(T, PhantomData<V>);

impl<T, V: Validator<T>> Validated<T, V> {
    pub fn new(value: T) -> Result<Self, V::Error> {
        V::validate(&value)?;
        Ok(Validated(value, PhantomData))
    }
    pub fn inner(&self) -> &T { &self.0 }
}

pub struct Positive;
impl Validator<i64> for Positive {
    type Error = String;
    fn validate(v: &i64) -> Result<(), String> {
        if *v > 0 { Ok(()) } else { Err(format!("{v} is not positive")) }
    }
}

pub type PositiveInt = Validated<i64, Positive>;
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Newtype definition | `type email = Email of string` | `struct Email(String)` |
| Smart constructor | `val email_of_string : string -> email option` | `fn Email::new(s: &str) -> Result<Email, &'static str>` |
| Unwrap | `let string_of_email (Email s) = s` | `fn as_str(&self) -> &str { &self.0 }` |
| Generic wrapper | `module Validated (V : VALIDATOR)` | `struct Validated<T, V>(T, PhantomData<V>)` |
| Validator | `module type VALIDATOR` | `trait Validator<T>` |
| Transparent access | Pattern match or accessor | `impl Deref for T { type Target = Inner; }` |

## Key Insights

1. **Zero cost** — Rust newtypes (`struct Foo(Bar)`) are guaranteed by the compiler to have the
   same memory layout as `Bar`. The abstraction is purely compile-time; no heap allocation, no
   indirection, no vtable.

2. **Phantom types replace functors** — OCaml uses functors (parameterised modules) to produce
   specialised validated types. Rust achieves the same with generic structs and `PhantomData<V>`,
   where `V` is a zero-sized marker type carrying the validator logic as a trait impl.

3. **`Deref` for transparent delegation** — OCaml pattern-matches to extract the inner value.
   Rust's `Deref` trait lets the newtype behave as its inner type for read-only operations
   (method calls, slice indexing) while still being a distinct type for function signatures.

4. **`Result` over `option`** — OCaml smart constructors naturally return `'a option`. Rust
   prefers `Result<T, E>` so callers get a machine-readable error; the `?` operator then
   propagates it ergonomically through call stacks.

5. **Type-level ID safety** — Two `u64` fields become `UserId(u64)` and `ProductId(u64)`.
   Passing a `ProductId` where a `UserId` is expected is a compile error with zero runtime
   cost — a guarantee OCaml's type aliases (`type user_id = int`) cannot provide because
   aliases are transparent to the type checker.

## When to Use Each Style

**Use simple validated newtypes (`Email`, `Username`) when:** the invariant is specific to one
type and you want an ergonomic `new` constructor with `Display` / `Deref` built in.

**Use the generic `Validated<T, V>` wrapper when:** multiple types share the same validation
shape (e.g. `PositiveInt`, `NonEmptyStr`, `BoundedF64`) and you want to define validators once
and reuse them — the Rust analogue of an OCaml functor application.

**Use typed ID newtypes (`UserId`, `ProductId`) when:** preventing accidental substitution of
structurally identical primitive types is the primary goal and no validation logic is needed.
