📖 **[View on hightechmind.io →](https://hightechmind.io/rust/571-pattern-dotdot-wildcard)**

---

# .. and _ Wildcards

## Problem Statement

Large structs and long tuples often have many fields, but a given function cares about only one or two. Without wildcards, every pattern must mention every field. `_` ignores a single element; `..` ignores zero or more elements in struct patterns and slice patterns. Together they enable precise, readable patterns that name exactly the data you need. This is particularly important for forward compatibility — using `..` in struct patterns means adding new fields to the struct does not break existing match arms.

## Learning Outcomes

- How `_` ignores a single field or variable binding
- How `..` ignores remaining struct fields or middle slice elements
- How `(a, _, _, _)` extracts the first element of a 4-tuple
- How `Point { x, .. }` extracts just `x` from a struct with many fields
- Why `..` in struct patterns is important for API evolution and forward compatibility

## Rust Application

`get_x(p: &Point)` uses `Point { x, y: _, z: _ }` — verbose form of explicit ignoring. `get_x_short` uses `let Point { x, .. } = p;` — concise `..` form. `first_of_four((a, _, _, _): (i32, i32, i32, i32)) -> i32` ignores three tuple elements. Slice patterns: `[first, .., last]` ignores the middle elements. `_name` variables (underscore-prefixed) suppress unused variable warnings while still binding.

Key patterns:
- `Struct { field, .. }` — extract `field`, ignore rest
- `(first, ..)` — extract first, ignore rest of tuple (nightly only; use `(first, _, _)`)
- `[head, ..]` — first element of slice
- `_` in any position — explicitly ignored single element

## OCaml Approach

OCaml's `_` and `_field` work identically, and record patterns also support partial matching:

```ocaml
let get_x { x; _ } = x   (* _ ignores other fields *)
let first_of_four (a, _, _, _) = a
```

OCaml requires listing all non-ignored record fields explicitly by default (compiler warning for partial patterns).

## Key Differences

1. **`..` vs `_`**: Rust's `..` in struct patterns ignores all unlisted fields; OCaml uses `_` for each field individually or relies on a compiler flag to suppress partial-pattern warnings.
2. **Forward compat**: Rust struct patterns without `..` cause compile errors when new fields are added to the struct; OCaml partial record patterns cause warnings.
3. **Tuple `..`**: Rust tuple struct `..` ignores trailing fields; OCaml requires explicit `_` for each ignored position.
4. **Slice middle**: Rust `[first, .., last]` skips middle elements; OCaml arrays require explicit indexing for this pattern.

## Exercises

1. **Add field safely**: Add a `w: i32` field to `Point3D { x, y, z }` and verify that patterns using `..` continue compiling without modification, while patterns without `..` report an error.
2. **Deep ignore**: Write `fn extract_leaf(tree: &Tree) -> Option<i32>` where `Tree` is a nested struct with many fields — use `..` to ignore fields at each level and extract only the leaf value.
3. **Tuple third**: Write `fn third_of_five<T: Copy>((_, _, x, _, _): (T, T, T, T, T)) -> T` using `_` for each ignored position — compare readability with an equivalent index-based version.
