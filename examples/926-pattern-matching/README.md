📖 **[View on hightechmind.io →](https://hightechmind.io/rust/926-pattern-matching)**

---

# 926-pattern-matching — Pattern Matching
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Pattern matching is the primary control flow mechanism of functional programming. Where imperative languages use `if/else` chains and switch statements, functional languages match on the structure of data: a tree is either a Leaf or a Node(left, value, right); a result is either Ok(x) or Err(e). OCaml and Rust both center pattern matching as a first-class language feature. Both compile match expressions to decision trees, check exhaustiveness at compile time (no unhandled case), and bind variables in each arm. This example uses algebraic shapes to compare the two languages' match syntax and capabilities.

## Learning Outcomes

- Write exhaustive `match` expressions on Rust enums
- Use guard clauses (`if condition`) inside match arms for refined patterns
- Use nested pattern destructuring to bind inner values
- Recognize that Rust's match is exhaustive — the compiler rejects missing cases
- Compare Rust's `match` with OCaml's `match` (nearly identical syntax)

## Rust Application

`area(shape: &Shape)` matches `Circle(r)`, `Rectangle(w, h)`, and `Triangle(a, b, c)` — each arm destructures the enum variant and computes the area. `describe` adds guard clauses: `Shape::Rectangle(w, h) if (w - h).abs() < f64::EPSILON` matches only squares. `Shape::Triangle(a, b, c) if (a - b).abs() < f64::EPSILON && (b - c).abs() < f64::EPSILON` matches equilateral triangles. The compiler rejects non-exhaustive matches — all variants must be handled.

## OCaml Approach

OCaml's `match shape with | Circle r -> pi *. r *. r | Rectangle(w, h) -> w *. h | Triangle(a, b, c) -> ...` is nearly identical. Guard clauses: `| Rectangle(w, h) when w = h -> ...`. OCaml's `when` keyword corresponds to Rust's `if` in match guards. Both compile to efficient decision trees. OCaml also has `function` keyword as shorthand for `fun x -> match x with`. The syntax is so similar that OCaml knowledge transfers almost directly to Rust pattern matching.

## Key Differences

1. **Syntax similarity**: OCaml `when` = Rust `if` in guards; OCaml `|` variant = Rust `,` for multiple patterns. Overall extremely similar.
2. **Reference patterns**: Rust requires `&Shape::Circle(r)` when matching `&Shape`; OCaml automatically dereferences values.
3. **Exhaustiveness**: Both enforce exhaustive matching; Rust uses `_` wildcard, OCaml uses `_` or named wildcards.
4. **Struct patterns**: Rust `struct` fields can be matched with `{ field_name: pattern }`; OCaml records use `{ field_name = pattern }`.

## Exercises

1. Add a `Ellipse(f64, f64)` variant for semi-major and semi-minor axes and add it to both the `area` and `describe` functions.
2. Implement `perimeter` using pattern matching, including Heron's formula for the triangle perimeter check (triangle inequality).
3. Write `classify_shape(shape: &Shape) -> &str` that returns "compact", "elongated", or "regular" based on the aspect ratio of each shape type.
