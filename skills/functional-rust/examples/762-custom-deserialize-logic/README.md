# 762: Custom Deserialization with Visitor Pattern

**Difficulty:** 4  **Level:** Advanced

Implement the serde Visitor pattern from scratch — understand how `Deserialize` and type-driven dispatch work.

## The Problem This Solves

Deserialization is harder than serialization: you're converting untyped wire data into a strongly-typed value, and different types expect different input shapes. A `Person` expects a map; a `u32` expects an integer; a `bool` expects true/false. The **Visitor pattern** is how `serde` solves this: the type being deserialized describes what it expects, and the deserializer drives it through a type-specific visitor.

When `#[derive(Deserialize)]` falls short — non-standard wire formats, complex validation during parsing, types from other crates you can't annotate — you implement `Deserialize` manually. Understanding the visitor pattern is the key to doing this correctly. It also explains why serde's error messages mention "expected a map" or "expected a string": that's the `Visitor::expecting()` method at work.

This example implements the core of serde's machinery from scratch: a `Token` type (what the deserializer emits), a `Visitor` trait (what the type being deserialized expects), a `SimpleDeserializer` (drives the visitor), and a concrete `PersonVisitor` that consumes a map token and constructs a `Person`.

## The Intuition

The visitor pattern inverts the usual control flow. Instead of the caller saying "give me a map", the deserializer says "I have a map — call `visit_map`". The visitor implements only the methods for the types it can accept, returning `Err(InvalidType)` for everything else. This lets the same `Deserialize` impl work across different deserializers (JSON, TOML, binary) — they all call the same visitor methods with different underlying data.

## How It Works in Rust

```rust
// The Visitor trait: a type says what it expects
pub trait Visitor<'de>: Sized {
    type Value;
    fn expecting(&self) -> &'static str;  // human-readable type description

    // Default: return InvalidType error — override the ones you accept
    fn visit_str(self, v: &'de str) -> Result<Self::Value, DeError> {
        Err(DeError::InvalidType { got: "str", expected: self.expecting() })
    }
    fn visit_map(self, m: Vec<(&'de str, &'de str)>) -> Result<Self::Value, DeError> {
        Err(DeError::InvalidType { got: "map", expected: self.expecting() })
    }
    // ... visit_i64, visit_f64, visit_bool ...
}

// Concrete visitor for Person: accepts only a map
pub struct PersonVisitor;
impl<'de> Visitor<'de> for PersonVisitor {
    type Value = Person;
    fn expecting(&self) -> &'static str { "a map with name and age" }

    fn visit_map(self, m: Vec<(&'de str, &'de str)>) -> Result<Person, DeError> {
        let name = m.iter().find(|(k, _)| *k == "name")
            .map(|(_, v)| v.to_string())
            .ok_or(DeError::MissingField("name"))?;
        let age = m.iter().find(|(k, _)| *k == "age")
            .and_then(|(_, v)| v.parse().ok())
            .ok_or(DeError::MissingField("age"))?;
        Ok(Person { name, age })
    }
}

// The Deserialize trait delegates to the visitor
impl<'de> Deserialize<'de> for Person {
    fn deserialize(de: SimpleDeserializer<'de>) -> Result<Self, DeError> {
        de.deserialize_any(PersonVisitor)  // "drive this visitor with my data"
    }
}
```

The `'de` lifetime is the key: it ties the visitor's output lifetime to the input data, enabling zero-copy deserialization (borrowed `&str` slices from the wire data without allocation). When the visitor returns owned `String` values, `'de` is still needed to satisfy the trait bounds.

## What This Unlocks

- **Manual `serde::Deserialize` impl** — the real serde `Deserialize` trait has the same shape: `fn deserialize<D: Deserializer<'de>>(de: D) -> Result<Self, D::Error>`; you implement it by calling `de.deserialize_map(YourVisitor)`.
- **`'de` lifetime** — the borrowed-data lifetime in serde is not magic; it's the lifetime that lets deserializers return `&'de str` slices pointing into the original input bytes, avoiding copies.
- **Informative error messages** — `Visitor::expecting()` is exactly the string that appears in serde's "invalid type: expected X, got Y" error messages; implementing it well makes your types easy to debug.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Visitor pattern | Typically via first-class modules | Trait with default methods for each token type |
| Lifetime in deserialization | Garbage collected — no lifetime tracking | `'de` ties output borrows to input data lifetime |
| Error propagation | Exceptions or `Result` | `?` operator throughout — clean early-return style |
| Type-driven dispatch | GADT or polymorphic variants | Trait objects / monomorphization via generics |
