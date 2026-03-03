# Phantom Types — OCaml vs Rust Comparison

## Core Insight

Phantom types let you encode invariants in the type system with zero runtime cost. Both OCaml and Rust support them, but Rust requires explicit `PhantomData<T>` marker while OCaml allows unused type parameters directly. The result is the same: the compiler prevents you from adding meters to seconds.

## OCaml Approach

Declares abstract types (`type meters`, `type seconds`) with no constructors — they exist purely at the type level. The `'a quantity` type carries the phantom parameter in its type signature. OCaml allows unused type parameters without complaint, making the pattern lightweight.

## Rust Approach

Uses zero-sized marker structs (`struct Meters;`) and `PhantomData<U>` in the quantity struct. `PhantomData` is a zero-sized type that tells the compiler "I logically use U" without actually storing data. Implementing `Add` trait only for same-unit quantities enforces safety through the trait system.

## Comparison Table

| Aspect        | OCaml                           | Rust                                 |
|---------------|---------------------------------|--------------------------------------|
| **Memory**    | Same as `float`                 | Same as `f64` (PhantomData is ZST)   |
| **Null safety** | Not applicable               | Not applicable                       |
| **Errors**    | Type error at compile time      | Type error at compile time           |
| **Iteration** | N/A                             | N/A                                  |
| **Marker**    | Abstract `type meters`          | `struct Meters;` + `PhantomData`     |

## Things Rust Learners Should Notice

1. **`PhantomData<T>`** is zero-sized — it compiles away completely, `Quantity` is just an `f64`
2. **Marker structs** — `struct Meters;` (unit struct) carries no data, exists only for types
3. **Trait bounds on `Add`** — implementing `Add for Quantity<U>` ensures same-unit addition
4. **`Copy` + `Clone`** can be derived since all fields are Copy (including PhantomData)
5. **Compile-time guarantee** — `meters(1.0) + seconds(2.0)` literally cannot compile

## Further Reading

- [PhantomData](https://doc.rust-lang.org/std/marker/struct.PhantomData.html)
- [Rust by Example: Phantom types](https://doc.rust-lang.org/rust-by-example/generics/phantom.html)
- [Zero-Sized Types](https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts)
