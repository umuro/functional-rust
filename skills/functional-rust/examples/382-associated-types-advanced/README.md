# 382: Associated Types vs Type Parameters

**Difficulty:** 3  **Level:** Advanced

Choose associated types when there's one natural output type; use type parameters when the caller decides.

## The Problem This Solves

You're designing a trait that produces a value. Should the output type be a type parameter (`trait Convert<T>`) or an associated type (`trait Convert { type Output; }`)? This choice ripples through your API, your callers' type annotations, and what the compiler can infer.

The rule of thumb: if an implementor can only convert to *one* target type meaningfully, use an associated type. If an implementor might convert to *many* different types and the caller controls which, use a type parameter. Getting this wrong means either multiple conflicting `impl` blocks or verbose type annotations that fight the compiler.

This distinction matters deeply in trait design. `Iterator` uses `type Item` (associated) because each iterator produces one specific item type. A hypothetical `Into<T>` uses a type parameter because you can convert into many different types.

## The Intuition

Associated type: the implementor decides what type is produced. `impl Iterator for MyStruct { type Item = u32; }` — callers know `MyStruct::Item` is `u32` without specifying it. One impl per struct (can't implement `Iterator` twice).

Type parameter: the caller decides what type is produced. `impl Into<String> for MyStruct` and `impl Into<Vec<u8>> for MyStruct` — both can coexist. Callers must often annotate: `let x: String = val.into()`.

Associated types also clean up bounds. Compare `fn process<I: Iterator<Item = u32>>(i: I)` vs `fn process<I: Iterator>(i: I) where I::Item: Display` — the first is more common; the second shows how associated type projections work in where clauses.

## How It Works in Rust

```rust
// Associated type — implementor picks the output
trait Parse {
    type Output;
    type Error;
    fn parse(s: &str) -> Result<Self::Output, Self::Error>;
}

struct JsonParser;
impl Parse for JsonParser {
    type Output = serde_json::Value;
    type Error = serde_json::Error;
    fn parse(s: &str) -> Result<Self::Output, Self::Error> {
        serde_json::from_str(s)
    }
}

// Type parameter — caller picks the output
trait ConvertTo<T> {
    fn convert(&self) -> T;
}

struct Rgb(u8, u8, u8);
impl ConvertTo<String> for Rgb {
    fn convert(&self) -> String { format!("#{:02X}{:02X}{:02X}", self.0, self.1, self.2) }
}
impl ConvertTo<u32> for Rgb {
    fn convert(&self) -> u32 { ((self.0 as u32) << 16) | ((self.1 as u32) << 8) | self.2 as u32 }
}

// Using associated type projection in bounds
fn sum_items<I>(iter: I) -> i32
where
    I: Iterator<Item = i32>,  // clean: Item is projected
{
    iter.sum()
}
```

## What This Unlocks

- **Clean iterator traits** — `type Item` means callers write `Iterator<Item = T>` not `Iterator<T>`.
- **Multiple implementations** — type parameters allow one type to implement a trait for many output types.
- **GATs (Generic Associated Types)** — associated types can themselves be generic: `type Item<'a>`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Associated type | Module type signature: `type t` | `trait Foo { type Bar; }` |
| Type parameter in trait | Functor parameter | `trait Foo<T> { }` |
| Type projection | `M.t` (module type member) | `T::Item` (associated type projection) |
| Multiple impls | Functors with different modules | Type parameter enables multiple `impl Foo<X> for T` |
