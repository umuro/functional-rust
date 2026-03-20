📖 **[View on hightechmind.io →](https://hightechmind.io/rust/931-records)**

---

# 931-records — Records and Functional Update

## Problem Statement

Functional programming favors immutable data structures with "functional update": instead of modifying a record in place, you create a new record with the changed field and the rest copied from the original. This preserves the original value, enables easy undo/redo, and is safe across threads. OCaml's `{ r with field = new_value }` syntax makes this concise. Rust provides the identical idiom with struct update syntax: `Struct { field: new_value, ..old }`. Both syntaxes copy unchanged fields from the original struct, creating a new value without mutation.

## Learning Outcomes

- Define structs with named fields as Rust's equivalent of OCaml records
- Use struct update syntax `Struct { field: value, ..old }` for functional update
- Destructure structs in function parameters (pattern matching on structs)
- Implement immutable operations that return new values rather than mutating
- Compare Rust's `..old` update syntax with OCaml's `{ r with field = value }`

## Rust Application

`Point { x, y }` and `Rect { origin: Point, width: f64, height: f64 }` are defined as `Copy` structs. `area(r: &Rect)` and `perimeter(r: &Rect)` borrow the rect. `translate(dx, dy, r: &Rect) -> Rect` uses the update syntax: `Rect { origin: Point { x: r.origin.x + dx, y: r.origin.y + dy }, ..*r }` — creates a new `Rect` with a new origin, copying `width` and `height` from `*r`. `contains_point` uses destructuring-style field access.

## OCaml Approach

OCaml records: `type rect = { origin: point; width: float; height: float }`. Functional update: `{ r with origin = { r.origin with x = r.origin.x +. dx; y = r.origin.y +. dy } }`. Pattern matching on records: `let area { width; height; _ } = width *. height`. OCaml's syntax is slightly more concise for nested updates because of the nested `with` syntax. OCaml record fields are mutable by declaring `mutable field: type` — Rust uses separate `let mut` binding for mutation.

## Key Differences

1. **Syntax**: OCaml `{ r with field = v }` vs Rust `Struct { field: v, ..r }`. Both copy unchanged fields; Rust lists fields before `..old`, OCaml uses `with`.
2. **Copy semantics**: Rust structs must implement `Copy` for the `..` update to work without moving; OCaml records are always shareable via GC.
3. **Nested updates**: OCaml's nested `with` is more concise for deeply nested updates; Rust requires explicit reconstruction at each nesting level.
4. **Mutable fields**: OCaml has explicit `mutable` field modifiers; Rust uses `let mut binding` — mutation is binding-level, not field-level.

## Exercises

1. Add a `scale(factor: f64, r: &Rect) -> Rect` function that scales width and height while keeping the origin fixed.
2. Implement `expand(dx: f64, dy: f64, r: &Rect) -> Rect` that grows the rect symmetrically around its center.
3. Create a `merge_rects(a: &Rect, b: &Rect) -> Rect` that returns the smallest bounding rect containing both input rects.
