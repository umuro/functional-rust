📖 **[View on hightechmind.io →](https://hightechmind.io/rust/128-type-level-bool)**

---

# Example 128: Type-Level Booleans

**Difficulty:** ⭐⭐⭐  
**Category:** Type System | Phantom Types | Compile-Time Safety  
**OCaml Source:** Real World OCaml — Chapter on Phantom Types

## Problem Statement

Encode `true`/`false` as compile-time *types* rather than runtime values, so the compiler can enforce logical constraints (e.g., "both validated AND logging must be enabled before calling `execute()`") without any runtime checks or panics.

## Learning Outcomes

- How zero-sized marker structs (`struct True; struct False;`) carry type-level information with zero runtime cost
- How `PhantomData<T>` lets a generic struct hold a type parameter that has no corresponding field
- How implementing a method only on a specific type instantiation (`impl Config<True, True>`) turns missing setup steps into compile errors
- How associated types in traits (`trait Not { type Output: Bool }`) encode type-level logic that the compiler evaluates statically

## OCaml Approach

OCaml uses *phantom type parameters* — a type variable that appears in the type signature but not in the data representation. `type 'b flag = { _phantom : unit }` is a record whose field carries no information; only the type parameter `'b` distinguishes `true_t flag` from `false_t flag`. Module signatures hide constructors so callers cannot forge an invalid state.

## Rust Approach

Rust uses empty structs (`struct True;` and `struct False;`) as type-level labels and `PhantomData<V>` to include a phantom type parameter in a struct without storing data. Methods are gated by writing `impl Config<True, True>` — only the fully-setup instantiation exposes `execute()`. Any attempt to call it prematurely is a compile-time type error, not a runtime panic.

## Key Differences

1. **Representation:** OCaml phantom types use a single record with an ignored field; Rust uses `PhantomData<T>` which compiles to nothing.
2. **Type-level logic:** OCaml reaches for GADTs or module functors for type-level AND/OR; Rust uses traits with associated types (`trait And<B> { type Output: Bool }`).
3. **Enforcement mechanism:** OCaml hides constructors via module signatures; Rust simply doesn't define the method on the wrong type instantiation.
4. **Safety guarantees:** Both approaches make invalid states *unrepresentable*, but Rust's error messages point directly to the missing method call, while OCaml's point to a type mismatch.

## Exercises

1. Implement type-level `And`, `Or`, and `Not` operations on your `True`/`False` types and write tests that verify them at compile time using trait bounds.
2. Build a type-level natural number system (Peano encoding) that allows expressing `Succ<Succ<Zero>>` for two, and implement type-level `Add` that resolves to the correct type at compile time.
3. Use type-level booleans to implement a permission system: define `CanRead`, `CanWrite` marker types and restrict API methods to only compile when the phantom type parameter satisfies the required capability trait.
