# LinkedIn Post: Pattern Matching Basics

🦀 **Functional Rust #003: Pattern Matching Basics**

Pattern matching is the soul of functional programming. Both OCaml and Rust enforce exhaustiveness, but with different ownership semantics.

**OCaml:**
```ocaml
type shape = Circle of float | Rectangle of float * float

let describe = function
  | Circle r when r > 10.0 -> "large circle"
  | Circle _ -> "small circle"
  | Rectangle (w, h) when w = h -> "square"
  | Rectangle _ -> "rectangle"
```

**Rust:**
```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn describe(shape: &Shape) -> &str {
    match shape {
        Shape::Circle(r) if *r > 10.0 => "large circle",
        Shape::Circle(_) => "small circle",
        Shape::Rectangle(w, h) if w == h => "square",
        Shape::Rectangle(_, _) => "rectangle",
    }
}
```

**Key differences:**

🔒 **Ownership** - Rust requires `&Shape` to avoid moving
✅ **Exhaustiveness** - Rust: compile error, OCaml: warning
🎯 **Guards** - `when` vs `if` keyword
📦 **Or patterns** - Both support `A | B | C`

**Rust bonus - matches! macro:**
```rust
if matches!(shape, Shape::Circle(_)) {
    // Quick boolean check
}
```

Pattern matching lets you design with data, not classes. The compiler guarantees you handle every case. No null checks, no instanceof chains - just clear, exhaustive logic.

Next: Option and Result types 🎁

#Rust #FunctionalProgramming #OCaml #PatternMatching #RustLang
