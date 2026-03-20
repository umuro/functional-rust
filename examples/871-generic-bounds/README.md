📖 **[View on hightechmind.io →](https://hightechmind.io/rust/871-generic-bounds)**

---

# 871-generic-bounds — Generic Bounds

## Problem Statement

Generic programming allows a single function to operate over many types, but only those types that satisfy certain contracts. In C++, this was implicit (duck typing) leading to cryptic template errors. Rust and Haskell require explicit constraints: `T: PartialOrd + Display` means "any type T that supports comparison and formatting." OCaml uses module signatures and functors to express similar constraints. Generic bounds are the foundation of type-safe abstractions: they enable writing `find_max` once that works for integers, floats, strings, or any custom comparable type.

## Learning Outcomes

- Express single and multiple trait bounds with the `<T: Trait1 + Trait2>` syntax
- Use the `Display` and `PartialOrd` standard library traits as common bounds
- Implement a custom trait that inherits from another trait as a supertrait
- Understand how bounds enable generic algorithms without sacrificing type safety
- Compare Rust trait bounds with OCaml's module type constraints

## Rust Application

The code implements `find_max<T: PartialOrd>` and `find_min<T: PartialOrd>` over slices, then `print_max<T: PartialOrd + Display>` adding the formatting requirement. A custom `Summarize: Display` supertrait is defined, and `Stats` implements both `Display` and `Summarize`. The `print_summaries<T: Summarize>` function demonstrates how a supertrait bound implies all parent trait methods are available. `clamp<T: PartialOrd>` shows bounds used for conditional logic without any output formatting.

## OCaml Approach

OCaml uses module types as the constraint mechanism: `module type Comparable = sig type t; val compare: t -> t -> int end`. A functor `MakeProcessor(M: Comparable)` is parameterized over any module satisfying the signature. For simple cases, OCaml uses its built-in polymorphic `compare` function (which works on any type), avoiding explicit constraints. The explicit comparison function parameter pattern `find_max_by (cmp: 'a -> 'a -> int)` mirrors Rust's trait bound but passed as a value rather than resolved at compile time.

## Key Differences

1. **Resolution time**: Rust bounds are resolved at compile time via monomorphization; OCaml's polymorphic functions use runtime representation with the universal compare.
2. **Supertrait inheritance**: Rust `trait Summarize: Display` enforces Display must also be implemented; OCaml module types compose by inclusion in the signature.
3. **Multiple bounds**: Rust uses `+` syntax (`T: A + B`); OCaml composes module types with `include` or functor composition.
4. **Error messages**: Rust bound violations produce targeted errors ("T doesn't implement Display"); OCaml module mismatches can produce verbose signature errors.

## Exercises

1. Add a `Bounded` trait with `min_value()` and `max_value()` associated functions, and implement it for `i32` and `f64`.
2. Write a generic `statistics<T: PartialOrd + Clone>` function that returns `(min, max, first, last)` in one pass over a slice.
3. Implement a `sorted_unique<T: Ord + Clone>` function that deduplicates a sorted slice, requiring both ordering and cloning bounds.
