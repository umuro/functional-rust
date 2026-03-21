📖 **[View on hightechmind.io →](https://hightechmind.io/rust/408-clone-copy-traits)**

---

# 408: Clone and Copy Traits
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Rust's ownership model moves values by default — after `let b = a`, `a` is consumed. For small stack-allocated types (integers, `f32`, pairs of floats), this restriction is unnecessary overhead: the value is trivially duplicated by copying bits. For heap-allocated types (strings, vectors), copying must be explicit to avoid unexpected O(n) copies. Rust resolves this with two traits: `Copy` (implicit bitwise copy, opt-in) and `Clone` (explicit `.clone()`, potentially expensive). This distinction makes performance visible in code: an `.clone()` call signals potential allocation.

`Copy` is implemented by all primitive types (`i32`, `f64`, `bool`, `char`), tuples of `Copy` types, and small structs/enums where all fields are `Copy`.

## Learning Outcomes

- Understand the semantic difference: `Copy` = implicit bit copy; `Clone` = explicit potentially-expensive copy
- Learn which types can implement `Copy` (no heap allocation, no `Drop`, all fields `Copy`)
- See how `Vector2D: Copy` allows passing by value without move semantics
- Understand why `LabeledPoint` (contains `String`) can only be `Clone`, not `Copy`
- Learn how `#[derive(Clone, Copy)]` works and its requirements

## Rust Application

In `src/lib.rs`, `Vector2D` has `f32 x` and `f32 y` — both `Copy`, so the struct is `Copy`. After `let b = a`, both `a` and `b` are valid. Methods take `self` by value (not reference) because copying is cheap. `LabeledPoint` contains `String` which is `!Copy` (heap-allocated), so it can only derive `Clone`. Calling `labeled.clone()` explicitly duplicates the heap string. The test suite demonstrates that `Vector2D` can be used after being "moved" (it was actually copied).

## OCaml Approach

OCaml values are either unboxed (integers, small variants) or boxed (heap-allocated). All values can be copied in OCaml — the GC manages aliasing. There is no `Copy`/`Clone` distinction. `String` in OCaml is mutable and copied by `String.copy`. Structural sharing is safe because the GC tracks all references. This simplicity comes at the cost of implicit O(n) copies when you don't intend to share.

## Key Differences

1. **Implicit vs. explicit**: Rust `Copy` types copy implicitly on assignment; OCaml copies are always by reference (shared) unless explicitly duplicated.
2. **Heap safety**: Rust's `Copy` exclusion of heap types prevents accidental O(n) copies; OCaml relies on GC reference counting to manage copies safely.
3. **Drop incompatibility**: Rust types implementing `Drop` cannot implement `Copy` (drop semantics conflict with bitwise copy); OCaml's GC finalizers have no such restriction.
4. **Performance visibility**: Rust's `.clone()` calls are explicit in code, signaling potential cost; OCaml's mutations on shared data are equally implicit but have different implications.

## Exercises

1. **Complex number**: Implement `Complex { re: f64, im: f64 }` with `Copy + Clone + Add + Mul + Display`. Verify that both addition operands remain valid after the operation (due to `Copy`).
2. **Color type**: Create `RgbColor { r: u8, g: u8, b: u8 }` implementing `Copy` and `LabeledColor { color: RgbColor, name: String }` implementing only `Clone`. Write a function accepting `impl Into<RgbColor>` that shows `RgbColor` can be passed by value freely.
3. **Clone cost measurement**: Create a `BigStruct { data: Vec<u8> }` and clone it 1 million times. Then create a `SmallStruct { x: f64, y: f64 }` and copy it 1 million times. Measure and compare the time, explaining the difference.
