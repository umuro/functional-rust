📖 **[View on hightechmind.io →](https://hightechmind.io/rust/386-object-safe-traits)**

---

# 386: Object-Safe Traits

## Problem Statement

Not every Rust trait can be used as `dyn Trait`. Object safety rules exist because vtables have fixed layouts: every slot is a function pointer taking an erased `*mut ()` as `self`. Traits with generic methods (`fn map<B>(self, f: impl Fn(A) -> B)`) cannot appear in vtables because each monomorphization would need a separate slot. Similarly, methods returning `Self` or taking `Self` parameters break the type erasure required for vtables. Understanding these rules prevents compiler errors and guides API design.

Object safety is a prerequisite for plugin architectures, `Box<dyn Error>`, `Box<dyn Iterator>`, and any heterogeneous dispatch system in Rust.

## Learning Outcomes

- Understand which trait features make a trait non-object-safe (generic methods, `Self` returns, non-dispatchable methods)
- Learn how to design traits that remain object-safe while providing useful functionality
- See how `Drawable` with concrete return types and `&self` methods satisfies object safety
- Understand the `where Self: Sized` trick to include non-object-safe methods in object-safe traits
- Learn how the compiler error messages guide you when violating object safety

## Rust Application

In `src/lib.rs`, the `Drawable` trait has methods `draw(&self) -> String` and `area(&self) -> f64` — both object-safe because they use concrete types, take `&self`, and do not reference `Self`. The `total_area` function takes `&[Box<dyn Drawable>]`, proving the trait is object-safe. The `Circle` and `Rectangle` implementations work through vtable dispatch in the slice iteration.

## OCaml Approach

OCaml's object methods are always virtually dispatched — there are no object safety restrictions. A class with a polymorphic method `method map : 'a -> 'b` is valid. OCaml's approach trades the performance guarantees of monomorphization for flexibility. The equivalent of non-object-safe methods in OCaml is simply methods with polymorphic types, which are allowed.

## Key Differences

1. **Safety rules**: Rust has explicit object safety rules enforced at compile time; OCaml has no equivalent restriction on virtual methods.
2. **Performance model**: Rust makes monomorphized (static dispatch) and vtable (dynamic dispatch) costs explicit and separately controllable; OCaml always uses dynamic dispatch for objects.
3. **Workaround**: Rust's `where Self: Sized` allows adding non-object-safe methods to object-safe traits; OCaml has no need for this pattern.
4. **Error messages**: Rust produces detailed object safety violation errors explaining which method caused the problem; OCaml type errors are at the type unification level.

## Exercises

1. **Non-object-safe trait**: Write a trait with a generic method (`fn transform<B>(&self) -> B`) and attempt to use it as `dyn Trait`. Document the compiler error, then fix it using `where Self: Sized` to exclude the method from the vtable.
2. **Shape renderer**: Extend `Drawable` with a `bounding_box(&self) -> (f64, f64, f64, f64)` method. Build a renderer that takes `Vec<Box<dyn Drawable>>`, computes total area, and finds the largest bounding box using only `dyn` dispatch.
3. **Object-safe wrapper**: Take a non-object-safe trait `Clonable` (with `fn clone_box(&self) -> Box<dyn Clonable>`) and implement it for several types. This is the standard pattern for making `Clone` work with `dyn Trait`.
