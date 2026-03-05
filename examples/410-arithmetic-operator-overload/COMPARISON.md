# OCaml vs Rust: Arithmetic Operator Overloading

## Side-by-Side Code

### OCaml — Custom infix operators
```ocaml
type vec2 = { x: float; y: float }

(* Define custom operators *)
let ( +^ ) a b = { x = a.x +. b.x; y = a.y +. b.y }
let ( -^ ) a b = { x = a.x -. b.x; y = a.y -. b.y }
let ( *^ ) s v = { x = s *. v.x; y = s *. v.y }

let () =
  let a = { x = 3.0; y = 4.0 } in
  let b = { x = 1.0; y = 2.0 } in
  let sum = a +^ b in
  let scaled = 2.0 *^ a in
  ()
```

### Rust — Traits for standard operators
```rust
use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
struct Vec2 { x: f64, y: f64 }

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f64) -> Vec2 {
        Vec2 { x: self.x * s, y: self.y * s }
    }
}

fn main() {
    let a = Vec2 { x: 3.0, y: 4.0 };
    let b = Vec2 { x: 1.0, y: 2.0 };
    let sum = a + b;       // Uses Add trait
    let scaled = a * 2.0;  // Uses Mul<f64>
}
```

---

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Operator style | Custom infix: `+^`, `*^` | Standard: `+`, `*` |
| Definition | `let ( +^ ) a b = ...` | `impl Add for Type` |
| Return type | Inferred | `type Output = ...` |
| Symmetric ops | Define both sides | `impl Mul<Vec2> for f64` |
| Compound ops | Not built-in | `AddAssign`, `MulAssign`, etc. |

---

## Operator Trait Mapping

| Operator | Trait | Method |
|----------|-------|--------|
| `+` | `Add` | `add(self, rhs)` |
| `-` | `Sub` | `sub(self, rhs)` |
| `*` | `Mul` | `mul(self, rhs)` |
| `/` | `Div` | `div(self, rhs)` |
| `%` | `Rem` | `rem(self, rhs)` |
| `-x` (unary) | `Neg` | `neg(self)` |
| `+=` | `AddAssign` | `add_assign(&mut self, rhs)` |

---

## Symmetric Operations

For `2.0 * vec`, you need to implement both directions:

```rust
// vec * scalar
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f64) -> Vec2 { ... }
}

// scalar * vec
impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Vec2 { ... }
}
```

---

## Compound Assignment

```rust
impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Vec2) {
        self.x += other.x;
        self.y += other.y;
    }
}

let mut v = Vec2::new(1.0, 2.0);
v += Vec2::new(3.0, 4.0);  // Uses AddAssign
```

---

## 5 Takeaways

1. **OCaml requires custom operator names; Rust uses standard `+`, `*`.**
   Rust's approach feels more natural for math types.

2. **Rust traits define both input and output types.**
   `Mul<f64>` with `type Output = Vec2` is explicit.

3. **Symmetric operations need two impls.**
   `Vec2 * f64` and `f64 * Vec2` are separate.

4. **Compound assignment is a separate trait.**
   `+=` uses `AddAssign`, not `Add`.

5. **Rust's Output associated type enables type transformations.**
   `Matrix * Vector = Vector` (different output type).
