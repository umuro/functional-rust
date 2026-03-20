📖 **[View on hightechmind.io →](https://hightechmind.io/rust/132-phantom-units)**

---

# Phantom Units of Measure

## Problem Statement

The Mars Climate Orbiter was lost in 1999 because one team used metric units and another used imperial — the mismatch went undetected until the spacecraft disintegrated. Unit confusion bugs kill software. Phantom types solve this: tag numeric values with their unit at the type level so `Quantity<Meters>` and `Quantity<Feet>` are distinct, incompatible types. Adding meters to feet is a compile error, not a runtime surprise. This pattern is used in aerospace, physics simulations, and financial systems.

## Learning Outcomes

- Understand how `PhantomData<Unit>` attaches type-level information without runtime cost
- Learn to implement unit-safe arithmetic via operator overloading (`Add`, `Mul`)
- See how unit multiplication produces derived units (e.g., `Meters / Seconds = MetersPerSecond`)
- Appreciate zero-overhead type safety: `Quantity<U>` has identical memory layout to `f64`

## Rust Application

`Quantity<Unit>` wraps `f64` with `PhantomData<Unit>`. `Add<Rhs = Quantity<U>>` is implemented with `Output = Quantity<U>` — adding two quantities of the same unit returns the same unit, while adding incompatible units fails to compile (no `Add` impl exists). `Mul` implementations produce derived units: `Quantity<Meters> * Quantity<Seconds>` can be given `Output = Quantity<MeterSeconds>`. The `new` constructor is the only way to create a `Quantity`, enforcing proper unit tagging at construction.

## OCaml Approach

OCaml can simulate phantom units using polymorphic types:
```ocaml
type meters
type feet
type 'unit quantity = float
let meters (x: float) : meters quantity = x
let feet (x: float) : feet quantity = x
(* let _ = meters 5.0 +. feet 3.0 (* type error *) *)
```
This is lighter syntactically but provides less safety — arithmetic operators on `float` still work regardless of the phantom tag because OCaml's type aliases are transparent to the operator implementations.

## Key Differences

1. **Operator restriction**: Rust's trait system only allows `+` between `Quantity<U>` and `Quantity<U>` — cross-unit addition is a compile error; OCaml's float operators work on all `_ quantity` values regardless of tag.
2. **Derived units**: Rust can encode `Meters * Seconds = MeterSeconds` via `Mul` trait bounds; OCaml requires explicit conversion functions.
3. **Zero overhead**: `size_of::<Quantity<Meters>>() == size_of::<f64>()` in Rust — the unit tag is pure type information; same in OCaml (transparent type alias).
4. **Conversion safety**: Rust requires explicit `from_feet_to_meters` conversion functions; OCaml similarly requires them, but the type system provides weaker enforcement.

## Exercises

1. Add a `Quantity<Kilograms>` type and implement `Mul<Quantity<MetersPerSecond>>` yielding `Quantity<NewtonSeconds>` (impulse = mass × velocity).
2. Write a `convert_feet_to_meters(q: Quantity<Feet>) -> Quantity<Meters>` function and verify that the result cannot be mistakenly added to a `Quantity<Feet>`.
3. Implement `PartialOrd` for `Quantity<U>` and write a `min_distance` function that takes two `Quantity<Meters>` values.
