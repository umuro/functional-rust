📖 **[View on hightechmind.io →](https://hightechmind.io/rust/436-macro-newtype-derive)**

---

# 436: Deriving Traits for Newtypes

**Difficulty:** 3  **Level:** Advanced

Propagate trait implementations through newtype wrappers automatically — so your `UserId(u64)` behaves like a `u64` where you want it to, while staying distinct where you don't.

## The Problem This Solves

The newtype pattern — `struct Meters(f64)` — gives you type safety: you can't accidentally pass `Seconds` where `Meters` is expected. But it comes with a cost: all the traits the inner type implements (`Display`, `Add`, `Hash`, `Ord`, `PartialEq`) are now missing from the wrapper. You have to re-implement or delegate each one manually. For a simple newtype with five or six useful traits, that's fifty lines of boilerplate.

Macro-generated delegation solves this. A derive macro (or `macro_rules!`) inspects the newtype's inner type and generates forwarding implementations: `impl Display for Meters` that calls `self.0.fmt(f)`, `impl Add for Meters` that calls `self.0 + other.0`, and so on. The `derive_more` crate in the ecosystem provides `#[derive(Add, Display, From, Into)]` exactly for this purpose.

Understanding how to build this yourself teaches you both the newtype pattern's ergonomics and the macro technique for generating delegating impls.

## The Intuition

A newtype derive macro generates `impl Trait for Wrapper` that delegates directly to the inner field — so your newtype gets the trait without the boilerplate.

## How It Works in Rust

```rust
// Manual newtype delegation — what macros generate
struct Meters(f64);

impl std::fmt::Display for Meters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}m", self.0)  // delegate to inner, add context
    }
}

impl std::ops::Add for Meters {
    type Output = Meters;
    fn add(self, other: Meters) -> Meters { Meters(self.0 + other.0) }
}

impl From<f64> for Meters {
    fn from(v: f64) -> Self { Meters(v) }
}

// With derive_more crate (what the proc macro does):
use derive_more::{Add, Display, From, Into};

#[derive(Debug, Clone, Copy, Add, Display, From, Into)]
#[display(fmt = "{}m", _0)]
struct Meters(f64);

// Now:
let a = Meters(1.5);
let b = Meters(2.0);
let c = a + b;          // Add works
println!("{}", c);      // Display works: "3.5m"
let raw: f64 = c.into();// Into<f64> works

// DIY macro for delegation
macro_rules! newtype_display {
    ($name:ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}

newtype_display!(Meters);
```

1. Identify which traits you want to delegate to the inner type.
2. Each delegation is `self.0.method()` — just forwarding through the wrapper.
3. Use `derive_more` for common traits (`Add`, `Display`, `From`, `Into`, `Deref`).
4. Write custom `macro_rules!` or proc macros for project-specific delegation patterns.

## What This Unlocks

- **Zero-cost type safety**: `Meters` and `Seconds` are distinct types; arithmetic between them is a compile error.
- **Full trait ecosystem**: Newtypes participate in `Hash` maps, `Display` formatting, operator overloading — without boilerplate.
- **Selective exposure**: Derive `Add` but not `Mul` — control exactly which operations make semantic sense.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Newtype pattern | `type meters = Meters of float` | `struct Meters(f64)` |
| Automatic delegation | Module sharing / functor application | `derive_more`, manual `impl`, or custom macro |
| Type alias (no safety) | `type meters = float` | `type Meters = f64` (alias, not newtype) |
| Operator overloading | Not supported | `impl Add for Meters` |
| Display formatting | `Printf` format functions | `impl Display` — `{}` in format strings |
