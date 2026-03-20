📖 **[View on hightechmind.io →](https://hightechmind.io/rust/870-trait-objects)**

---

# 870-trait-objects — Trait Objects (Dynamic Dispatch)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

In object-oriented languages, polymorphism is the default: a method call on a base-class pointer dispatches to the concrete implementation at runtime. Functional languages like OCaml achieve the same through algebraic types with pattern matching (static) or object types (dynamic). Rust offers both mechanisms: generics with trait bounds (static/monomorphized dispatch, zero cost) and trait objects `dyn Trait` (dynamic dispatch via vtable, runtime polymorphism). The choice between them affects binary size, flexibility, and whether the concrete type must be known at compile time. This example shows both approaches using a `Shape` hierarchy.

## Learning Outcomes

- Understand the difference between static dispatch (`impl Trait`) and dynamic dispatch (`dyn Trait`)
- Implement a vtable-based polymorphic collection using `Box<dyn Trait>` or `&dyn Trait`
- Recognize when dynamic dispatch is necessary (heterogeneous collections, plugin systems)
- Compare Rust's trait objects with OCaml's object types and polymorphic variants
- Understand object safety requirements for traits used as trait objects

## Rust Application

The code defines a `Shape` trait with `area()` and `name()` methods. `Circle`, `Rectangle`, and `Triangle` implement it. `total_area_dyn` accepts `&[&dyn Shape]` — a slice of trait objects — enabling a heterogeneous collection of shapes without generics. The vtable is allocated once per concrete type; each `&dyn Shape` is a fat pointer (data pointer + vtable pointer). A generic version using `<T: Shape>` is also shown for comparison, demonstrating the zero-cost alternative when the type is uniform.

## OCaml Approach

OCaml provides two mechanisms. Object types (`class type shape = object method area: float ... end`) give structural subtyping and dynamic dispatch. Algebraic types with pattern matching (`type shape_adt = Circle of float | Rectangle of float * float | Triangle of float * float`) give static exhaustive dispatch. The OCaml algebraic approach is more idiomatic for closed hierarchies; object types are used when the hierarchy must be extensible by third parties. Both compile to efficient code; object dispatch uses a vtable like Rust's `dyn Trait`.

## Key Differences

1. **Fat pointers**: Rust `&dyn Trait` is two words (data + vtable); OCaml object values carry a vtable inline in the heap block header.
2. **Object safety**: Rust trait objects require object-safe traits (no generics in methods, no `Self` return); OCaml has no such restriction.
3. **Heterogeneous collections**: Both languages support `Vec<Box<dyn Shape>>`-style collections; OCaml does it with `shape list` using object types.
4. **Open vs closed**: Rust trait objects enable open extension (anyone can implement the trait); OCaml ADTs are closed unless extended with polymorphic variants.

## Exercises

1. Add a `Perimeter` trait and implement it for all shapes, then compute total perimeter of a `Vec<Box<dyn Shape>>`.
2. Implement a `describe_all` function that takes `&[Box<dyn Shape>]` and returns a formatted summary using `Display`.
3. Refactor `total_area_dyn` to use `impl Iterator<Item = &dyn Shape>` instead of a slice, and explain the difference.
