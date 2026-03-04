# 410: Arithmetic Operator Overloading

**Difficulty:** 2  **Level:** Intermediate

Implement `Add`, `Sub`, `Mul`, and friends so your types work with `+`, `-`, `*`, `+=`, and `-`.

## The Problem This Solves

You've built a `Vec2`, `Matrix`, `Complex`, or `Money` type. Without operator overloading, every calculation looks like `Vec2::add(Vec2::mul(a, 2.0), b)` — verbose, hard to read, easy to make mistakes in. The math becomes obscured by the API.

Rust lets you implement the standard arithmetic traits (`Add`, `Sub`, `Mul`, `Neg`, `AddAssign`, etc.) so that `a + b`, `2.0 * a`, `-a`, and `a += b` work naturally on your types. The compiler desugars these operators to trait method calls — there's no runtime overhead beyond the method call itself, which is typically inlined.

This matters most for mathematical types, but also for custom numeric wrappers (`Meters`, `Dollars`), newtype patterns where you want arithmetic to work, and DSL types that benefit from operator syntax.

## The Intuition

Each operator maps to a trait. The trait has an associated type `Output` — what the expression evaluates to. The method takes `self` by value (for `Copy` types, this is a bitwise copy). The compiler transforms `a + b` into `Add::add(a, b)`.

You can implement `Add<Vec2>` for `Vec2` (vector + vector) AND `Mul<f64>` for `Vec2` (vector × scalar) AND `Mul<Vec2>` for `f64` (scalar × vector) — different left-hand-side types, different right-hand-side types. The full generality lets you express natural mathematical notation.

## How It Works in Rust

```rust
use std::ops::{Add, Sub, Mul, Neg, AddAssign};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2 { x: f64, y: f64 }

impl Vec2 {
    fn new(x: f64, y: f64) -> Self { Vec2 { x, y } }
    fn magnitude(self) -> f64 { (self.x * self.x + self.y * self.y).sqrt() }
    fn dot(self, other: Vec2) -> f64 { self.x * other.x + self.y * other.y }
}

// Vec2 + Vec2 → Vec2
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 { Vec2::new(self.x + o.x, self.y + o.y) }
}

// Vec2 - Vec2 → Vec2
impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 { Vec2::new(self.x - o.x, self.y - o.y) }
}

// Vec2 * f64 → Vec2 (right-scalar multiply)
impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, s: f64) -> Vec2 { Vec2::new(self.x * s, self.y * s) }
}

// f64 * Vec2 → Vec2 (left-scalar multiply: 2.0 * v)
impl Mul<Vec2> for f64 {
    type Output = Vec2;
    fn mul(self, v: Vec2) -> Vec2 { Vec2::new(self * v.x, self * v.y) }
}

// -Vec2
impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 { Vec2::new(-self.x, -self.y) }
}

// Vec2 += Vec2 (compound assignment — separate trait)
impl AddAssign for Vec2 {
    fn add_assign(&mut self, o: Vec2) { self.x += o.x; self.y += o.y; }
}

fn main() {
    let a = Vec2::new(3.0, 4.0);
    let b = Vec2::new(1.0, 2.0);

    println!("a + b = {:?}", a + b);      // uses Add
    println!("a - b = {:?}", a - b);      // uses Sub
    println!("2.0 * a = {:?}", 2.0 * a); // uses Mul<Vec2> for f64
    println!("|a| = {:.2}", a.magnitude());
    println!("-a = {:?}", -a);             // uses Neg

    let mut c = a;
    c += b;   // uses AddAssign
    println!("a += b: {:?}", c);
}
```

**Important:** `Add` takes `self` by value, consuming it. For `Copy` types like `Vec2`, this is fine — they're bitwise-copied. For non-`Copy` types (like `Matrix` with a `Vec`), you typically implement `Add for &Matrix` to avoid cloning.

## What This Unlocks

- **Mathematical types** — `Point + Point`, `Matrix * Matrix`, `Quaternion * Quaternion` — all write and read naturally.
- **Newtype arithmetic** — `Meters + Meters`, `Dollars * f64` — domain types with enforced units and natural syntax.
- **Generic math algorithms** — `fn dot_product<T: Mul<Output=T> + Add<Output=T>>(...)` — write once, works for any numeric type.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Custom operators | `let ( +^ ) a b = ...` — define any infix operator with `( op )` syntax | Implement `Add`/`Sub`/`Mul` etc. — fixed set of overloadable operators |
| Flexibility | Can define `( +++ )` or any symbol | Only standard operators (`+`, `-`, `*`, `/`, `%`, `&`, `|`, `^`, `<<`, `>>`, `!`, `-` unary) |
| Left-hand type | Function `( +^ ) a b` — `a` is first arg | `impl Add<Rhs> for Lhs` — `Lhs` is the left side; different impls for different `Lhs` |
| Compound assignment | No `+=` operator in OCaml | `AddAssign` trait — separate from `Add` |
