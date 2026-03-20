📖 **[View on hightechmind.io →](https://hightechmind.io/rust/066-phantom-types)**

---

# 066 — Phantom Types (Type-Safe Units)

## Problem Statement

Phantom types use type parameters that carry no runtime data — they exist only to prevent mixing incompatible values at compile time. `Quantity<Meters>` and `Quantity<Seconds>` have the same runtime representation (`f64`) but are different types. Accidentally adding meters to seconds is a compile-time error, not a runtime error.

This technique prevented the Mars Climate Orbiter crash (1999) — a $327M mission failed because one system output pound-force-seconds while another expected newton-seconds. Phantom types enforce unit correctness statically. They appear in type-safe API design (typed IDs, state machines), dimensional analysis libraries (`uom` crate), and authentication tokens (typed permissions).

## Learning Outcomes

- Use `PhantomData<U>` to attach a type parameter that carries no runtime data
- Create unit marker types as zero-sized structs: `struct Meters; struct Seconds;`
- Implement `Add` for `Quantity<U>` to allow adding same-unit quantities
- Understand that `PhantomData<U>` informs the compiler about type variance
- Recognize phantom types as zero-cost abstraction: no runtime overhead

## Rust Application

`Quantity<U>` stores a `f64` value and a `_unit: PhantomData<U>`. Unit markers `Meters` and `Seconds` are zero-sized structs. `Quantity::<Meters>::new(5.0)` and `Quantity::<Seconds>::new(3.0)` are different types. Implementing `Add<Self> for Quantity<U>` allows `q1 + q2` only when both are the same unit — trying to add meters to seconds fails at compile time.

## OCaml Approach

OCaml's phantom types use a type parameter that is never instantiated: `type 'a quantity = Quantity of float`. `type meters = Meters and type seconds = Seconds`. `let meters x : meters quantity = Quantity x` and `let seconds x : seconds quantity = Quantity x`. At the call site: `add (meters 5.0) (seconds 3.0)` fails because the type checker sees `meters quantity` vs `seconds quantity` as different types.

## Key Differences

1. **`PhantomData`**: Rust requires explicit `PhantomData<U>` to avoid "type parameter U is never used" errors. OCaml's phantom parameters are allowed without any placeholder — the type checker tracks the phantom directly.
2. **Zero-sized types**: Rust's `struct Meters;` is a zero-sized type (ZST) — no memory at runtime. OCaml's phantom parameter is completely absent at runtime.
3. **Variance**: `PhantomData<U>` makes `Quantity<U>` covariant in `U` (by default). Use `PhantomData<fn(U)>` for contravariance or `PhantomData<*mut U>` for invariance. OCaml's variance is inferred.
4. **`uom` crate**: Rust's `uom` crate (units of measure) uses phantom types extensively to implement dimension-safe arithmetic. This example is the conceptual foundation.

## Exercises

1. **Velocity**: Define a `Speed<Meters, Seconds>` phantom type and implement division: `Quantity<Meters> / Quantity<Seconds> -> Quantity<Speed>`. Use a type alias `type MetersPerSecond = Speed<Meters, Seconds>`.
2. **State machine**: Use phantom types to model a connection state: `Connection<Disconnected>` and `Connection<Connected>`. Only `Connection<Connected>` can have a `send()` method. This is the typestate pattern.
3. **Typed IDs**: Define `Id<User>`, `Id<Post>`, `Id<Comment>` as phantom-typed `u64` wrappers. Demonstrate that passing a `UserId` where a `PostId` is expected fails at compile time.
