📖 **[View on hightechmind.io →](https://hightechmind.io/rust/874-phantom-types)**

---

# 874-phantom-types — Phantom Types
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Type safety is most powerful when it prevents entire classes of bugs at compile time. Phantom types are a technique where a type parameter appears in a struct definition but carries no data — it exists solely to encode information in the type system. The classic applications are units of measure (preventing meters from being added to seconds), state machines (preventing unlocked-door operations when the door is locked), and access levels (preventing unauthorized API calls). F#'s units of measure, Haskell's `phantom type` pattern, and OCaml's typed phantom parameters all implement this idea. Rust uses `PhantomData<T>` to hold the phantom type without adding runtime overhead.

## Learning Outcomes

- Use `PhantomData<T>` to encode compile-time invariants without runtime cost
- Implement units-of-measure type safety preventing invalid arithmetic
- Model a state machine (locked/unlocked door) where invalid transitions are compile errors
- Understand why `PhantomData` is needed in Rust (variance, drop check)
- Compare Rust phantom types with OCaml's phantom type parameters

## Rust Application

`Quantity<Unit>` holds a `f64` value and a `PhantomData<Unit>` marker. Type tags `Meters`, `Seconds`, and `MetersPerSecond` are empty structs. `speed(distance: Quantity<Meters>, time: Quantity<Seconds>) -> Quantity<MetersPerSecond>` encodes the physics invariant: passing `Quantity<Seconds>` as the distance argument is a compile error. The state machine `Door<State>` uses `Locked` and `Unlocked` phantom tags: `door.open()` is only available on `Door<Unlocked>`, so calling it on a `Door<Locked>` fails to compile. Zero runtime overhead — `PhantomData` is zero-sized.

## OCaml Approach

OCaml phantom types use type parameters that are never instantiated: `type 'unit quantity = { value: float }`. The type `meters quantity` and `seconds quantity` are distinct despite having the same runtime representation. OCaml's type checker enforces the distinction. State machines use `type unlocked door` and `type locked door` — different phantom instantiations of the same runtime struct. The OCaml approach is more concise but requires discipline since the compiler cannot guarantee exhaustiveness of state transitions.

## Key Differences

1. **PhantomData requirement**: Rust requires `PhantomData<Unit>` because unused type parameters are rejected; OCaml type parameters are allowed to be phantom implicitly.
2. **Zero-size guarantee**: Rust `PhantomData<T>` is zero-sized; OCaml phantom types also add zero runtime cost.
3. **State machine enforcement**: Rust can encode that only `Door<Unlocked>` has an `open` method via separate `impl` blocks; OCaml uses module signatures to hide invalid methods.
4. **Variance**: `PhantomData` also controls variance (covariant/contravariant) in Rust; OCaml handles variance through its type system automatically.

## Exercises

1. Add a `Kilograms` unit and implement `force(mass: Quantity<Kilograms>, acceleration: Quantity<MetersPerSecond>) -> Quantity<Newtons>`.
2. Add an `Ajar` state to the door state machine and define legal transitions: `Unlocked -> Ajar -> Locked`.
3. Implement a typed API key wrapper `ApiKey<Permission>` where `ReadOnly` and `ReadWrite` permissions grant different method sets.
