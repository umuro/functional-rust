📖 **[View on hightechmind.io →](https://hightechmind.io/rust/563-pattern-struct-destructuring)**

---

# Struct Destructuring
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Extracting multiple fields from a struct simultaneously — without intermediate temporary variables — is a fundamental ergonomic feature of pattern matching languages. Struct destructuring allows binding multiple fields in one pattern, renaming them, ignoring others with `..`, and doing all of this directly in function parameters. This eliminates boilerplate field access chains (`p.x`, `p.y`) in favor of declarative extraction that reads like a specification of what data you need.

## Learning Outcomes

- How `let Point { x, y } = p;` binds struct fields to local names
- How `Point { name: n, age: a }` renames fields during destructuring
- How `..` ignores remaining fields in a destructuring pattern
- How destructuring works directly in function parameters: `fn f(Point { x, y }: &Point)`
- Where struct destructuring reduces boilerplate: geometric computation, configuration extraction

## Rust Application

`get_x(p: &Point)` uses `let Point { x, .. } = p;` — binds `x`, ignores `y`. `distance_from_origin` uses `let Point { x, y } = p;` — binds both. `describe_person` uses `Person { name: n, age: a }` — field rename. `add_points(Point(x1, y1): &Point, Point(x2, y2): &Point)` destructures in function parameters directly. `quadrant(p: &Point)` uses match arms with `Point { x, y }` patterns plus guards.

Key patterns:
- `let Struct { field1, field2 } = val;` — field binding
- `Struct { field: new_name }` — rename during destructuring
- `Struct { field1, .. }` — partial destructuring with ignore
- Parameter destructuring: `fn f(Struct { field }: Struct)`

## OCaml Approach

OCaml record destructuring uses the same syntax as pattern matching:

```ocaml
type point = { x: int; y: int }
let get_x { x; _ } = x
let distance_from_origin { x; y } = sqrt (float_of_int (x*x + y*y))
let f { x; y } { x = x2; y = y2 } = { x = x+x2; y = y+y2 }
```

## Key Differences

1. **`..` vs `_`**: Rust uses `..` to ignore remaining fields; OCaml uses `_` in the pattern for each unused field, or relies on the fact that not listing a field is an error without explicit ignore.
2. **Parameter destructuring**: Both Rust and OCaml support destructuring in function parameters — idiomatic in both languages.
3. **Field rename**: Rust `{ field: new_name }` renames; OCaml `{ field = new_name }` does the same with `=` instead of `:`.
4. **Match in let**: Rust allows struct destructuring in `let` bindings, `match` arms, and parameters; OCaml has the same flexibility.

## Exercises

1. **RGB to HSL**: Write `fn rgb_to_hsl(Color { r, g, b }: Color) -> (f64, f64, f64)` using parameter destructuring to extract the color components without field access notation.
2. **Config extractor**: Create a `struct Config { host: String, port: u16, max_connections: usize, timeout_ms: u64 }` and write a function that destructures it to build a connection string.
3. **Nested destructuring**: Write `fn summarize(Outer { inner: Inner { value } }: &Outer) -> i32` that extracts the inner value using nested struct destructuring in the parameter.
