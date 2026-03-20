📖 **[View on hightechmind.io →](https://hightechmind.io/rust/565-pattern-tuple-struct)**

---

# Tuple Struct Patterns

## Problem Statement

Tuple structs serve two roles: lightweight structs with positional fields, and newtypes — single-field wrappers that create type-distinct values. The newtype pattern (`struct Meters(f64)`) prevents accidentally passing `Seconds` where `Meters` is expected, even though both are `f64` internally. Pattern matching on tuple structs extracts fields directly, either in `match` arms, `let` bindings, or function parameters. This is common in unit systems (NASA Mars Climate Orbiter crashed from a meters/feet confusion), domain modeling, and type-safe ID types.

## Learning Outcomes

- How `let Point(x, y) = p;` destructures a tuple struct
- How `fn f(Point(x, y): &Point)` puts destructuring directly in function parameters
- How `Meters(m)` in a pattern extracts the inner value while discarding the wrapper
- How the newtype pattern (`struct Meters(f64)`) prevents type confusion at compile time
- Where tuple struct patterns appear: unit conversions, typed IDs, domain modeling

## Rust Application

`get_coords(p: &Point) -> (i32, i32)` uses `let Point(x, y) = p;`. `add_points` takes two `&Point` parameters with destructuring in the signature. `meters_to_feet(Meters(m): Meters) -> f64` extracts `m` in the parameter — the `Meters` wrapper prevents calling with a `Seconds` value. `describe_color(Color(r, g, b): &Color)` destructures all three components for the match.

Key patterns:
- `TupleStruct(field1, field2)` in `let` and `match`
- Parameter destructuring: `fn f(Wrapper(inner): Wrapper)`
- `_` for unused fields: `TupleStruct(first, _)` 
- Newtype unwrap: `Meters(m)` extracts `f64` from `Meters`

## OCaml Approach

OCaml achieves newtypes with abstract module signatures or single-constructor variants:

```ocaml
type meters = Meters of float
type seconds = Seconds of float
let meters_to_feet (Meters m) = m *. 3.28084
```

The `Meters of float` pattern is identical in concept to Rust's `struct Meters(f64)`.

## Key Differences

1. **Syntax**: Rust `struct Meters(f64)` defines a tuple struct; OCaml `type meters = Meters of float` defines a single-constructor variant — both serve the newtype purpose.
2. **Exhaustiveness**: OCaml `match` on a single-constructor variant is always exhaustive; Rust `let Meters(m) = val` is irrefutable (always succeeds).
3. **Type safety**: Both prevent accidentally mixing `Meters` and `Seconds` — a compile error in both languages.
4. **Transparency**: Rust tuple struct fields can be `pub` or private; OCaml's abstraction is controlled via module signatures.

## Exercises

1. **Temperature newtype**: Create `struct Celsius(f64)` and `struct Fahrenheit(f64)` with conversion functions that destructure in parameters; verify the compiler rejects passing `Celsius` to a `Fahrenheit` function.
2. **Typed ID**: Implement `struct UserId(u64)` and `struct PostId(u64)` and write a `get_post_for_user(uid: UserId, pid: PostId) -> String` function — verify `(pid, uid)` argument order fails at compile time.
3. **Two-field tuple struct**: Create `struct Range(i32, i32)` with methods `contains(&self, n: i32) -> bool` and `overlap(&self, Range(other_start, other_end): &Range) -> bool` using tuple struct destructuring.
