📖 **[View on hightechmind.io →](https://hightechmind.io/rust/432-macro-enum-dispatch)**

---

# 432: enum_dispatch via Macros

**Difficulty:** 4  **Level:** Expert

Generate a closed-set enum that delegates trait method calls to its variants — the same performance as a hand-written `match`, with none of the boilerplate, and zero heap allocation.

## The Problem This Solves

When you have a trait and several types that implement it, the idiomatic way to store them heterogeneously is `Box<dyn Trait>` — a fat pointer with heap allocation and a virtual dispatch table. For most use cases that's fine. But in hot loops, plugin systems, or game ECS code, the heap allocation and cache-unfriendly vtable lookup matter. You want static dispatch but still need to store mixed types in a `Vec`.

The manual solution is to write an enum with one variant per concrete type and implement the trait on the enum by delegating each method to the inner value via `match`. This is fast — the compiler can inline the inner calls — but it's massive boilerplate. Adding a new type means adding a variant, an `impl From`, and touching every method arm.

The `enum_dispatch` macro automates this pattern: you declare the trait, the enum, and the types, and the macro generates the full delegation impl and all `From` conversions.

## The Intuition

The generated enum is an ordinary Rust enum — it lives on the stack, its size is the max of its variants. The macro writes what you'd write by hand: a `match` arm for each variant that calls the same method on the inner value.

This is the "closed world" trade-off: you give up the ability to add new types at runtime (unlike `dyn Trait`) in exchange for zero heap allocation, inlineable dispatch, and the compiler knowing the full set of possibilities (enabling exhaustiveness checks and optimisations).

## How It Works in Rust

```rust
// The macro generates: enum, trait impl with match delegation, From impls
macro_rules! enum_dispatch {
    (
        trait $trait_name:ident { $(fn $method:ident($($p:ident: $t:ty),*) -> $ret:ty;)* }
        enum $enum_name:ident { $($variant:ident($inner:ty)),* $(,)? }
    ) => {
        #[derive(Debug)]
        enum $enum_name { $($variant($inner),)* }

        impl $trait_name for $enum_name {
            $(fn $method(&self, $($p: $t),*) -> $ret {
                match self {
                    $($enum_name::$variant(inner) => inner.$method($($p),*),)*
                }
            })*
        }

        $(impl From<$inner> for $enum_name {
            fn from(x: $inner) -> Self { $enum_name::$variant(x) }
        })*
    };
}

trait Animal {
    fn speak(&self) -> String;
    fn speed(&self) -> f64;
}

struct Dog; struct Cat;
impl Animal for Dog { fn speak(&self) -> String { "Woof!".into() } fn speed(&self) -> f64 { 5.0 } }
impl Animal for Cat { fn speak(&self) -> String { "Meow!".into() } fn speed(&self) -> f64 { 8.0 } }

// Macro invocation — generates AnyAnimal enum + impl Animal for AnyAnimal
enum_dispatch! {
    trait Animal { fn speak() -> String; fn speed() -> f64; }
    enum AnyAnimal { Dog(Dog), Cat(Cat), }
}

// Usage: no Box, no heap, stored by value
let animals: Vec<AnyAnimal> = vec![Dog.into(), Cat.into()];
for a in &animals {
    println!("{} ({}m/s)", a.speak(), a.speed()); // static dispatch via match
}

// Size: the enum, not a pointer
println!("{}", std::mem::size_of::<AnyAnimal>());   // e.g. 24
println!("{}", std::mem::size_of::<Box<dyn Animal>>()); // 16 (pointer + vtable ptr)
```

**What the macro generates (expanded):**
```rust
enum AnyAnimal { Dog(Dog), Cat(Cat) }
impl Animal for AnyAnimal {
    fn speak(&self) -> String {
        match self {
            AnyAnimal::Dog(inner) => inner.speak(),
            AnyAnimal::Cat(inner) => inner.speak(),
        }
    }
    fn speed(&self) -> f64 {
        match self { AnyAnimal::Dog(i) => i.speed(), AnyAnimal::Cat(i) => i.speed() }
    }
}
impl From<Dog> for AnyAnimal { fn from(x: Dog) -> Self { AnyAnimal::Dog(x) } }
impl From<Cat> for AnyAnimal { fn from(x: Cat) -> Self { AnyAnimal::Cat(x) } }
```

## What This Unlocks

- **Zero-allocation heterogeneous collections** — store mixed concrete types in a `Vec<AnyAnimal>` with no heap per element.
- **Plugin systems with known plugin sets** — define all plugin types at compile time, generate the dispatch enum, get fast dispatch with type safety.
- **Hot path optimisation** — the compiler can inline the inner method calls through the `match` arms; `Box<dyn Trait>` cannot be inlined.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Heterogeneous collections | Polymorphic variants or GADT-based encoding | `dyn Trait` (heap) or enum dispatch (stack) |
| Dynamic dispatch | Objects / first-class modules | `Box<dyn Trait>` — heap + vtable |
| Static enum dispatch | Variant matching is the norm | Enum dispatch macro generates match-based delegation |
| Adding a new type | New constructor + match arm | New variant + macro regenerates everything |
| Heap allocation | GC-managed; implicit | Explicit; enum dispatch avoids it entirely |
