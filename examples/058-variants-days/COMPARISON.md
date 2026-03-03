# Comparison: Variants — Days of the Week — OCaml vs Rust

## OCaml

```ocaml
type day = Sun | Mon | Tue | Wed | Thu | Fri | Sat

let day_name = function
  | Sun -> "Sunday" | Mon -> "Monday" | Tue -> "Tuesday"
  | Wed -> "Wednesday" | Thu -> "Thursday" | Fri -> "Friday"
  | Sat -> "Saturday"

let is_weekend = function
  | Sun | Sat -> true
  | _         -> false

let next_day = function
  | Sun -> Mon | Mon -> Tue | Tue -> Wed | Wed -> Thu
  | Thu -> Fri | Fri -> Sat | Sat -> Sun
```

## Rust — Idiomatic (impl methods)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Day { Sun, Mon, Tue, Wed, Thu, Fri, Sat }

impl Day {
    pub fn name(self) -> &'static str {
        match self {
            Day::Sun => "Sunday", Day::Mon => "Monday", /* ... */
        }
    }

    pub fn is_weekend(self) -> bool {
        matches!(self, Day::Sun | Day::Sat)
    }

    pub fn next(self) -> Day {
        match self {
            Day::Sun => Day::Mon, /* ... */ Day::Sat => Day::Sun,
        }
    }
}
```

## Rust — Numeric (discriminant arithmetic)

```rust
impl Day {
    pub fn from_index(i: u8) -> Option<Day> { /* 0..=6 → variant */ }
    pub fn to_index(self) -> u8 { self as u8 }
    pub fn next_arithmetic(self) -> Day {
        Day::from_index((self.to_index() + 1) % 7).unwrap()
    }
}
```

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Declaration | `type day = Sun \| Mon \| ...` | `enum Day { Sun, Mon, ... }` |
| Equality | Built-in (structural) | Requires `#[derive(PartialEq, Eq)]` |
| Copying | Implicit (all values copyable) | Requires `#[derive(Clone, Copy)]` |
| Debug printing | Via `ppx_deriving` or manual | `#[derive(Debug)]` |
| Pattern syntax | `function \| Sun -> ...` | `match self { Day::Sun => ... }` |
| Or-patterns | `\| Sun \| Sat -> true` | `matches!(self, Day::Sun \| Day::Sat)` |
| Namespace | Flat (just `Sun`) | Prefixed (`Day::Sun`) unless `use Day::*` |
| Method attachment | No (free functions only) | `impl Day { fn name(self) ... }` |

## Type Signatures Explained

**OCaml:** `val day_name : day -> string` — simple function from variant to string
**Rust:** `fn name(self) -> &'static str` — method taking `self` by copy (since `Day: Copy`), returning a string slice with `'static` lifetime (string literals live forever)

## Takeaways

1. **Near-identical concept:** Both are sum types with exhaustive matching — the core idea transfers perfectly
2. **Rust requires opt-in traits:** `derive` macros replace OCaml's built-in structural equality and copy
3. **Namespace difference:** Rust variants are namespaced (`Day::Sun`); OCaml's are module-level (`Sun`)
4. **Methods vs functions:** Rust encourages `impl` blocks; OCaml keeps everything as standalone functions
5. **`matches!` macro** is Rust's ergonomic equivalent of OCaml's multi-arm pattern returning `bool`
