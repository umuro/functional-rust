📖 **[View on hightechmind.io →](https://hightechmind.io/rust/739-phantom-units-of-measure)**

---

# 739-phantom-units-of-measure — Phantom Units of Measure
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The Mars Climate Orbiter was lost in 1999 because one module computed thrust in pound-force seconds and another expected newton-seconds. Unit confusion errors cost lives and billions of dollars. F# pioneered Units of Measure as a language feature; Rust and OCaml achieve the same protection via phantom types. A `Length<Meters>` cannot be added to `Length<Feet>` without an explicit conversion, catching unit errors at compile time with zero runtime overhead.

## Learning Outcomes

- Represent physical units as zero-sized phantom marker types (`Meters`, `Feet`, `Kilograms`)
- Implement arithmetic that preserves or derives units: `Length<M> + Length<M> -> Length<M>`
- Prevent mixing incompatible units at compile time (`Length<Meters>` + `Length<Feet>` fails)
- Implement explicit unit conversion functions that change the phantom parameter
- See how the pattern extends to derived units (velocity = length / time)

## Rust Application

The example implements `Length<Unit>` and `Mass<Unit>` wrappers with phantom `PhantomData<Unit>`. Addition is implemented only for matching unit parameters using `impl<U> Add<Length<U>> for Length<U>`. A `convert` function takes a conversion factor and returns the value in a different unit type. Multiplication of `Length<Meters>` by `Length<Meters>` returns `Area<MetersSquared>` — the type system tracks derived units.

## OCaml Approach

F# has native units-of-measure syntax (`[<Measure>] type m` and `float<m>`). OCaml lacks this but achieves it via phantom types: `type 'u length = Length of float`. Jane Street's `Validated` and `units-sexp` libraries provide similar functionality. The OCaml community often uses `Gg` library for 3D geometry with typed vectors. GADTs allow encoding unit arithmetic relationships directly.

## Key Differences

1. **Language support**: F# has first-class unit support; Rust and OCaml both use phantom types as a library-level approximation.
2. **Derived units**: Rust can encode `Meters * Meters = MetersSquared` via trait implementations; OCaml requires more verbose GADT indices to express multiplication of units.
3. **Conversion safety**: Both languages require explicit conversion functions; neither can infer unit equivalences automatically.
4. **Ecosystem**: Rust has `uom` (units of measurement) and `dimensioned` crates with full SI unit coverage; OCaml has no equivalent widely-used crate.

## Exercises

1. Add `Time<Seconds>` and implement `Velocity<MetersPerSecond>` as the result of dividing `Length<Meters>` by `Time<Seconds>`.
2. Implement a `Celsius` to `Fahrenheit` conversion that changes the phantom type: `fn to_fahrenheit(t: Temperature<Celsius>) -> Temperature<Fahrenheit>`.
3. Create a `force` function that multiplies `Mass<Kilograms>` by `Acceleration<MetersPerSecondSquared>` and returns `Force<Newtons>`.
