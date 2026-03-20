📖 **[View on hightechmind.io →](https://hightechmind.io/rust/210-iso-basics)**

---

# Iso Basics — Isomorphisms
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

An isomorphism is a lossless, reversible transformation between two types: `Celsius <-> Fahrenheit`, `String <-> Vec<u8>`, `(A, B) <-> (B, A)`. An Iso optic captures this bidirectional mapping as a first-class value. Unlike a lens (asymmetric: `get` and `set` are different operations) or prism (partial), an iso is fully symmetric: `get` and `reverse_get` are inverses of each other. Isos are the most restrictive optic — they appear at the top of the optics hierarchy.

## Learning Outcomes

- Understand isos as lossless bidirectional transformations
- Implement `Iso<S, A>` with `get` and `reverse_get`
- Verify the iso laws: `get(reverse_get(a)) == a` and `reverse_get(get(s)) == s`
- See how isos fit into the optics hierarchy as the most specific optic

## Rust Application

`Iso<S, A>` has `get: Box<dyn Fn(&S) -> A>` and `reverse_get: Box<dyn Fn(&A) -> S>`. Methods: `flip()` reverses the direction returning `Iso<A, S>`. `then(other: Iso<A, B>)` composes two isos into `Iso<S, B>`. Examples: `string_bytes_iso` between `String` and `Vec<u8>` (via `into_bytes`/`from_utf8`). `celsius_fahrenheit_iso` between temperature scales. The `flip` operation demonstrates that isos are symmetric — unlike lenses or prisms.

## OCaml Approach

OCaml's iso pattern:
```ocaml
type ('s, 'a) iso = {
  get : 's -> 'a;
  reverse_get : 'a -> 's;
}
let flip iso = { get = iso.reverse_get; reverse_get = iso.get }
let ( >> ) i1 i2 = { get = (fun s -> i2.get (i1.get s));
                     reverse_get = (fun a -> i1.reverse_get (i2.reverse_get a)) }
```
The `(>>)` composition operator chains isos naturally. OCaml's infix operators make composition more readable.

## Key Differences

1. **Symmetry**: Isos are symmetric — `flip` produces a valid iso; lenses and prisms have no `flip` operation.
2. **Laws**: Iso laws are stronger than lens laws — both directions must be inverses; lens laws only require round-trip consistency in one direction.
3. **Hierarchy**: Every iso is a lens, prism, and traversal; going from iso to lens drops the `reverse_get` — a narrowing of capabilities.
4. **Practical use**: Isos arise when two representations are fully equivalent: `Celsius`/`Fahrenheit`, `Unix timestamp`/`DateTime`, `RGB`/`HSL`.

## Exercises

1. Implement `km_miles_iso` between kilometers and miles and verify both law directions.
2. Compose `celsius_fahrenheit_iso` with `fahrenheit_kelvin_iso` to produce `celsius_kelvin_iso`.
3. Implement `swap_iso: Iso<(A, B), (B, A)>` that swaps the components of a pair.
