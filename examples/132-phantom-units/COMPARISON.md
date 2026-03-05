# OCaml vs Rust: Phantom Units of Measure

## Side-by-Side Code

### OCaml
```ocaml
(* Abstract type tags — phantom; no constructors exposed *)
type meters
type seconds
type kilograms

(* The phantom type parameter 'unit is never stored *)
type 'unit quantity = { value : float }

let meters  v : meters  quantity = { value = v }
let seconds v : seconds quantity = { value = v }

let add (a : 'u quantity) (b : 'u quantity) : 'u quantity =
  { value = a.value +. b.value }

(* distance / time → speed — no separate speed type in simple version *)
let speed_of dist time : float =
  dist.value /. time.value
```

### Rust (idiomatic — operator overloading)
```rust
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul};

pub struct Meters;
pub struct Seconds;
pub struct MetersPerSecond;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<U> Quantity<U> {
    pub fn new(value: f64) -> Self {
        Quantity { value, _unit: PhantomData }
    }
    pub fn value(self) -> f64 { self.value }
}

// Same-unit addition — only compiles when both sides match
impl<U> Add for Quantity<U> {
    type Output = Quantity<U>;
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.value + rhs.value)
    }
}

// Dimensional analysis: Meters / Seconds = MetersPerSecond
impl Div<Quantity<Seconds>> for Quantity<Meters> {
    type Output = Quantity<MetersPerSecond>;
    fn div(self, rhs: Quantity<Seconds>) -> Self::Output {
        Quantity::new(self.value / rhs.value)
    }
}
```

### Rust (functional style — explicit helper functions)
```rust
pub fn meters(v: f64)  -> Quantity<Meters>  { Quantity::new(v) }
pub fn seconds(v: f64) -> Quantity<Seconds> { Quantity::new(v) }

// Explicit conversion — no implicit coercion
pub fn feet_to_meters(q: Quantity<Feet>) -> Quantity<Meters> {
    Quantity::new(q.value() * 0.3048)
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Phantom type wrapper | `type 'unit quantity = { value: float }` | `struct Quantity<Unit> { value: f64, _unit: PhantomData<Unit> }` |
| Unit tag | Abstract type: `type meters` | Zero-sized struct: `struct Meters;` |
| Same-unit addition | `val add : 'u quantity -> 'u quantity -> 'u quantity` | `impl<U> Add for Quantity<U>` |
| Dimensional product | Requires separate type annotation | `impl Div<Quantity<Seconds>> for Quantity<Meters>` → `Quantity<MetersPerSecond>` |
| Runtime cost | `{ value: float }` — one float | `struct Quantity<Unit> { value: f64, _unit: PhantomData<Unit> }` — one f64 (PhantomData is zero bytes) |

## Key Insights

1. **Phantom means "in the type, not in the data"**: In OCaml the phantom parameter `'unit` never appears in the record fields. In Rust, `PhantomData<Unit>` is the explicit carrier — it is a zero-sized type that satisfies the compiler's "every type parameter must be used" rule without adding any runtime storage.

2. **Dimensional analysis via `impl` blocks**: OCaml's simple version just returns `float` for mixed-unit operations, losing the unit information. Rust lets you express `Meters / Seconds → MetersPerSecond` as a concrete `Div` impl that the compiler enforces — wrong-unit division is a compile error.

3. **No implicit coercions**: In both languages, `Quantity<Meters>` and `Quantity<Feet>` are structurally identical at runtime yet statically incompatible. Conversion between them must be explicit (`feet_to_meters`). The compiler refuses to accept one where the other is expected — exactly the safety guarantee that could have saved the Mars Climate Orbiter.

4. **Zero runtime overhead**: Both OCaml and Rust compile phantom types away entirely. `Quantity<Meters>` and `Quantity<Feet>` are both just a single `float`/`f64` in memory. The unit tags exist only during type-checking.

5. **Operator overloading extends the safety net**: Rust's trait system lets `+` remain intuitive while being unit-safe — `meters(3.0) + meters(4.0)` compiles, but `meters(3.0) + feet(4.0)` does not. OCaml achieves the same via the type of `add`, but without operator syntax.

## When to Use Each Style

**Use idiomatic Rust (operator overloading)** when building a library that end users interact with through natural arithmetic expressions — the `+`, `/`, `*` syntax keeps calling code readable while the type system enforces correctness invisibly.

**Use explicit constructor functions** (`meters()`, `seconds()`) at API boundaries to make the unit annotation visible in the source code, reducing the chance of a caller accidentally constructing a value with the wrong unit.
