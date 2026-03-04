# 232: Sum Types

**Difficulty:** ⭐⭐  **Level:** Type System / Functional Patterns

A sum type contains EXACTLY ONE of its variants at any time — it is the categorical coproduct, and every Rust enum is one.

## The Problem This Solves

Programs need to represent alternatives: a shape is either a circle, a rectangle, or a triangle — never more than one at once. Traditional OOP handles this with inheritance and runtime type checks (`instanceof`). The problem is that the compiler can't verify you've handled all cases, and adding a new variant silently breaks every existing switch statement.

Sum types — Rust's `enum` — solve both problems. The compiler knows all variants at compile time and enforces that you handle every one. Pattern matching is *exhaustive*: add a variant without updating every `match`, and the code won't compile. This is the strongest guarantee you can get for variant-based dispatch.

Understanding sum types categorically connects them to product types: together, products ("AND") and sums ("OR") form the complete vocabulary for data. Any data structure you can describe is built from products and sums.

## The Intuition

A **sum type** holds exactly ONE of its variants. An `enum Shape { Circle(f64), Rectangle(f64, f64), Triangle(f64, f64, f64) }` is always *either* a circle, *or* a rectangle, *or* a triangle — never a circle-rectangle hybrid, never undetermined.

The name "sum type" comes from counting: if `Circle` has 1 shape of data and `Rectangle` has 2 values of data, the total number of *distinct possible values* of `Shape` is the sum of variants' possibilities. Product types multiply; sum types add.

The categorical view: the *coproduct* of `A` and `B` is `Either<A, B>` with two *injections* `inl: A -> Either<A, B>` and `inr: B -> Either<A, B>`. The universal property: for any type `C` with functions `f: A -> C` and `g: B -> C`, there's a unique `h: Either<A, B> -> C` — this is exactly `match { Left(a) => f(a), Right(b) => g(b) }`.

**Exhaustive pattern matching** is the universal property made concrete: `match` forces you to handle every injection.

## How It Works in Rust

```rust
// Sum type: exactly one variant at a time
#[derive(Debug, Clone, PartialEq)]
enum Shape {
    Circle(f64),           // one piece of data
    Rectangle(f64, f64),   // two pieces of data (product inside sum!)
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r)           => std::f64::consts::PI * r * r,
            Shape::Rectangle(w, h)     => w * h,
            Shape::Triangle(a, b, c)   => {
                let s = (a + b + c) / 2.0;
                (s * (s-a) * (s-b) * (s-c)).sqrt()
            }
        }
        // Compiler error if any variant is missing — exhaustiveness enforced
    }

    fn name(&self) -> &'static str {
        match self {
            Shape::Circle(_)         => "circle",
            Shape::Rectangle(_, _)   => "rectangle",
            Shape::Triangle(_, _, _) => "triangle",
        }
    }
}
```

The coproduct injections are the enum constructors themselves:
```rust
// inl: A -> Either<A, B>   ≡   Shape::Circle  (inject into Circle variant)
let c = Shape::Circle(3.0);

// Universal property: eliminate Either<A,B> -> C via case analysis
// This is exactly what match does
fn dispatch(shape: &Shape) -> String {
    match shape {
        Shape::Circle(r)       => format!("circle r={}", r),
        Shape::Rectangle(w, h) => format!("rect {}x{}", w, h),
        Shape::Triangle(a,b,c) => format!("tri {},{},{}", a, b, c),
    }
}
```

`Option<T>` is `enum Option { None, Some(T) }` — sum of "nothing" and "something".  
`Result<T, E>` is `enum Result { Ok(T), Err(E) }` — sum of "success" and "failure".

## What This Unlocks

- **Compiler-enforced exhaustiveness** — add a new variant and every incomplete `match` becomes a compile error. No silent failures when extending the type.
- **No null, no magic sentinel values** — `Option<T>` is a sum type that makes absence explicit in the type system. The compiler prevents you from forgetting to handle the `None` case.
- **Products inside sums, sums inside products** — `Rectangle(f64, f64)` is a product (two floats) inside a sum (Shape). Nesting gives you the full vocabulary for any data model.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Declaration | `type shape = Circle of float \| Rectangle of float * float` | `enum Shape { Circle(f64), Rectangle(f64, f64) }` |
| Pattern matching | `match s with \| Circle r -> ...` | `match s { Shape::Circle(r) => ... }` |
| Exhaustiveness | Compiler-enforced | Compiler-enforced |
| `Option` | Built-in `'a option` (`None` / `Some`) | Built-in `Option<T>` (`None` / `Some`) |
| Injection syntax | Constructor name (lowercase in pattern) | `Variant::Constructor(...)` |
