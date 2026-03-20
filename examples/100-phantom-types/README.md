[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 100 — Phantom Types

## Problem Statement

Use `PhantomData<Unit>` to tag a `Quantity<Unit>` struct with a unit type, preventing addition of meters to seconds at compile time. The unit type `Meters` or `Seconds` carries no runtime data — it exists purely as a type-level marker. Compare with OCaml's abstract phantom types (`type meters`) and the simpler newtype alternative.

## Learning Outcomes

- Use `PhantomData<T>` to add a type parameter that exists only at compile time
- Understand that `PhantomData<T>` is zero-sized: no runtime cost
- Implement `Add<Quantity<U>> for Quantity<U>` to allow same-unit addition
- Understand why you cannot add `Quantity<Meters>` to `Quantity<Seconds>` — different `U`
- Map Rust's `PhantomData` to OCaml's abstract `type meters` phantom type
- Compare with the simpler newtype pattern for when phantom types are overkill

## Rust Application

`Quantity<Unit>` stores `value: f64` and `_unit: PhantomData<Unit>` (zero bytes at runtime). `Meters` and `Seconds` are empty marker structs. `Add for Quantity<U>` is generic over `U`, so adding two `Quantity<Meters>` works but adding `Quantity<Meters> + Quantity<Seconds>` is a compile error — the `U` type parameters don't match. `meters(v)` and `seconds(v)` are constructor functions that return the correctly tagged type. `PhantomData` also tells the compiler about variance and ownership, preventing unsound code.

## OCaml Approach

OCaml uses `type meters` (empty abstract type) and `type 'a quantity = Q of float`. `meters x : meters quantity` and `seconds x : seconds quantity` create tagged values. `add (Q a : 'a quantity) (Q b : 'a quantity)` enforces same-unit addition at the type level. The `'a` phantom parameter makes this work with the same elegance as Rust's `PhantomData`, but with less syntactic overhead.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Marker | `PhantomData<Unit>` field | `type meters` empty type |
| Tagged value | `Quantity<Meters>` | `meters quantity` |
| Unit-safe add | `impl Add for Quantity<U>` | `add : 'a quantity -> 'a quantity -> 'a quantity` |
| Runtime cost | Zero | Zero |
| Compile error | Type mismatch on `U` | Type mismatch on `'a` |
| Alternative | Newtype `struct MetersVal(f64)` | Same: single-constructor variant |

Phantom types are a power tool for encoding invariants in the type system. They are used in Rust for: typestate machines (`File<Open>` vs `File<Closed>`), brand types (preventing index confusion), and units of measure. When a simpler newtype suffices, prefer it.

## Exercises

1. Add a `MetersPerSecond` unit type and implement `fn speed(d: Quantity<Meters>, t: Quantity<Seconds>) -> Quantity<MetersPerSecond>`.
2. Implement `impl Mul<f64> for Quantity<U>` and `impl Div<Quantity<U>> for Quantity<U>` returning a dimensionless `f64`.
3. Extend to a `Quantity<Unit, Currency>` with two phantom parameters for monetary amounts in different currencies.
4. Implement a typestate builder: `Connection<Disconnected>` → `Connection<Connected>` that prevents calling `send` before `connect`.
5. In OCaml, implement the same typestate pattern using phantom types and GADTs (generalised algebraic data types).
