# 139: HList — Heterogeneous List

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

A type-safe list of mixed types where the full type of each element is preserved and accessible at compile time — unlike tuples (fixed structure), `Vec` (uniform type), or `Box<dyn Any>` (runtime type).

## The Problem This Solves

Tuples in Rust hold mixed types — `(i32, &str, f64)` — but they're structurally fixed. You can't write generic code that "iterates" a tuple, appends to it, or extracts the nth element when n is a runtime value. `Vec<T>` handles any length but requires all elements to be the same type. `Vec<Box<dyn Any>>` loses type information and requires unsafe downcasting.

What if you need a collection that is both heterogeneous (different element types) and type-safe (types fully preserved and accessible without casts)? Serialization frameworks that process fields of different types, function composition chains where each step changes the type, database row types — all of these need something that tuples can't express generically and Vec can't express at all.

An HList (heterogeneous list) is the answer. `HCons<i32, HCons<&str, HCons<f64, HNil>>>` is a type that carries exactly three elements of exactly those types. `head()` returns `&i32`. `tail().head()` returns `&&str`. The full type information is preserved, and you can write generic code over any HList using recursive trait impls.

## The Intuition

An HList is built from two constructors — just like a regular linked list, but in the type system:
- `HNil` — the empty list
- `HCons<H, T>` — an element `H` prepended to tail `T`

So `hlist!(42, "hello", true)` has type `HCons<i32, HCons<&str, HCons<bool, HNil>>>`. Every element's type is part of the overall type. The length is also encoded: `HNil` has length 0, `HCons<_, T>` has length 1 + len(T) — computable at compile time.

You access elements by traversing the type structure: `.head()` gives the first element, `.tail()` drops it, `.tail().head()` gives the second, and so on. The compiler tracks the type at each position — no casts.

## How It Works in Rust

```rust
// Two constructors — the entire HList type system
struct HNil;                    // empty list
struct HCons<H, T>(H, T);      // head :: tail

// Compute length at compile time via recursive trait
trait HLength { const LEN: usize; }
impl HLength for HNil { const LEN: usize = 0; }
impl<H, T: HLength> HLength for HCons<H, T> {
    const LEN: usize = 1 + T::LEN;  // recursion in the type system
}

// Access head — only on non-empty lists
trait Head {
    type Output;
    fn head(&self) -> &Self::Output;
}
impl<H, T> Head for HCons<H, T> {
    type Output = H;
    fn head(&self) -> &H { &self.0 }
}

// Access tail — only on non-empty lists
trait Tail {
    type Output;
    fn tail(&self) -> &Self::Output;
}
impl<H, T> Tail for HCons<H, T> {
    type Output = T;
    fn tail(&self) -> &T { &self.1 }
}

// Ergonomic construction macro
macro_rules! hlist {
    () => { HNil };
    ($head:expr $(, $tail:expr)*) => { HCons($head, hlist!($($tail),*)) }
}

// Type alias macro for cleaner signatures
macro_rules! hlist_type {
    () => { HNil };
    ($head:ty $(, $tail:ty)*) => { HCons<$head, hlist_type!($($tail),*)> }
}

// Debug-style display via recursive trait
trait HDisplay { fn h_display(&self) -> String; }
impl HDisplay for HNil { fn h_display(&self) -> String { String::new() } }
impl<H: std::fmt::Debug, T: HDisplay> HDisplay for HCons<H, T> {
    fn h_display(&self) -> String {
        let rest = self.1.h_display();
        if rest.is_empty() { format!("{:?}", self.0) }
        else { format!("{:?}, {}", self.0, rest) }
    }
}
```

Usage:
```rust
let list = hlist!(42, "hello", 3.14, true);

// Each access is type-safe — compiler knows the type at each position
let first: &i32  = list.head();           // 42
let second: &&str = list.tail().head();    // "hello"
let third: &f64  = list.tail().tail().head();  // 3.14

// Length is a compile-time constant
let len = <hlist_type!(i32, &str, f64, bool) as HLength>::LEN;  // 4

// Typed declarations
let typed: hlist_type!(i32, &str) = hlist!(1, "two");
println!("{}", typed.h_display());  // "1, \"two\""
```

## What This Unlocks

- **Typed record composition** — build function pipelines where each step adds a field to a typed record, with no HashMap and no dynamic dispatch.
- **Frunk / generic programming** — the `frunk` crate uses HList as the foundation for generic derive, type-safe lenses, and structural subtyping.
- **Variadic functions** — simulate functions with variable argument counts where each argument's type is tracked; useful for test frameworks, SQL query builders, and RPC frameworks.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| HList type | GADT: `type _ hlist = HNil : hnil hlist \| HCons : 'a * 'b hlist -> ('a,'b) hcons hlist` | Structs: `struct HNil; struct HCons<H, T>(H, T)` |
| Element access | Pattern match: `let HCons (x, _) = h` | Trait methods: `h.head()`, `h.tail()` |
| Generic algorithms | Type-level recursion via GADT | Recursive trait impls: one for `HNil`, one for `HCons<H, T>` |
| Construction | Explicit `HCons (42, HCons ("hello", HNil))` | `hlist!(42, "hello")` macro |
