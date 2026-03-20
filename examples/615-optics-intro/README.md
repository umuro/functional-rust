📖 **[View on hightechmind.io →](https://hightechmind.io/rust/615-optics-intro)**

---

# Optics: optics intro

## Problem Statement

Optics are composable data accessors originating from Haskell's lens library (Edward Kmett, 2012). They solve the deeply-nested update problem in immutable data: updating a field three levels deep requires rebuilding all intermediate values. Optics compose — a lens into a struct field composed with a prism for an enum variant gives a combined accessor that can get, set, and modify deeply nested optional values. The optic hierarchy includes Lens (exactly one focus), Prism (zero or one focus on enum variants), Traversal (zero or more foci), and Iso (lossless bidirectional conversion).

## Learning Outcomes

- The specific optic demonstrated in this example and what it focuses on
- How to implement the optic manually using closures or structs in Rust
- How this optic composes with others in the hierarchy
- The laws the optic must satisfy for correct behavior
- Where optics are used: state management, config manipulation, nested data transformation

## Rust Application

The source implements the optic concept using Rust's closure system. Due to lack of higher-kinded types, Rust uses explicit struct wrappers with Box<dyn Fn> fields or monomorphized versions with generic parameters. The examples show: the core get/set/preview/review operations, composition of two optics, and the laws verified in tests.

Key patterns:
- Core optic struct with closure fields
- Composition: combining optics for deeper focus
- Laws verification: identity, roundtrip, idempotence
- Practical example: modifying nested struct/enum data

## OCaml Approach

OCaml optics use the same record-with-function approach:

```ocaml
type ('s, 'a) lens = { get: 's -> 'a; set: 's -> 'a -> 's }
let name_lens = { get = (fun u -> u.name); set = (fun u n -> { u with name = n }) }
let compose l1 l2 = { get = (fun s -> l2.get (l1.get s)); set = (fun s a -> l1.set s (l2.set (l1.get s) a)) }
```

## Key Differences

1. **HKT requirement**: Haskell's van Laarhoven encoding uses Functor/Applicative for optic unification requiring HKT; Rust uses explicit struct types per optic kind.
2. **Operator syntax**: Haskell uses `^.`, `.~`, `%~` for terse optic use; Rust uses method calls, more verbose but explicit.
3. **Derive macros**: `lens-rs` and similar crates provide derive macros for automatic lens generation; OCaml uses `ppx_lens` for the same.
4. **Performance**: Boxed closure implementations have runtime overhead; monomorphized generic versions compile to zero-cost abstractions.

## Exercises

1. **Lens laws**: Write tests for all three lens laws: get-set (get after set returns set value), set-get (set to current value is identity), set-set (second set wins).
2. **Prism laws**: Write tests for prism laws: preview after review returns Some, set via review then preview round-trips.
3. **Compose two levels**: Create a lens for a struct field and a prism for an enum variant in that field — compose them and modify the inner value when the variant is present.
