📖 **[View on hightechmind.io →](https://hightechmind.io/rust/760-serde-derive-concept)**

---

# 760: Derive-Based Serialization: How derive(Serialize) Works

**Difficulty:** 3  **Level:** Intermediate

Understand what `#[derive(Serialize, Deserialize)]` generates by manually writing the equivalent code.

## The Problem This Solves

`#[derive(Serialize, Deserialize)]` from the `serde` crate is magic to many Rust developers — it "just works" for most structs, but when it doesn't (custom formats, enums with complex shapes, backwards compatibility), you need to implement the traits manually. Before you can do that, you need to understand what the derive macro generates.

This example manually implements the Serialize/Deserialize trait pattern — not using `serde`'s actual traits (which require the crate), but using a simplified analog that captures the same conceptual structure: a `Serialize` trait that emits fields into an output, and a `Deserialize` trait that reconstructs a value from a map of fields. Understanding this makes the real `serde` machinery approachable.

The key insight is that `#[derive(Serialize)]` generates a `Serialize` implementation that calls the serializer's `serialize_struct` method for each field by name. There's no runtime reflection — the field names and types are baked in at compile time by the proc macro.

## The Intuition

`Serialize` is a trait with one method: emit your fields. `Deserialize` is a trait with one method: reconstruct yourself from field data. `#[derive(Serialize)]` is a proc macro that reads your struct definition and writes the `impl Serialize` block that lists every field by name. At runtime, there's no reflection — just a sequence of `insert("field_name", value.to_string())` calls generated at compile time.

## How It Works in Rust

```rust
// The trait shape (simplified analog of serde::Serialize)
pub trait Serialize {
    fn serialize_fields(&self, out: &mut HashMap<String, String>);
}

// For struct Color { r: u8, g: u8, b: u8 }
// This is what #[derive(Serialize)] generates conceptually:
impl Serialize for Color {
    fn serialize_fields(&self, out: &mut HashMap<String, String>) {
        out.insert("r".to_string(), self.r.to_string());  // field name → stringified value
        out.insert("g".to_string(), self.g.to_string());
        out.insert("b".to_string(), self.b.to_string());
    }
}

// This is what #[derive(Deserialize)] generates:
impl Deserialize for Color {
    fn deserialize_fields(map: &HashMap<String, String>) -> Option<Self> {
        Some(Color {
            r: map.get("r")?.parse().ok()?,  // lookup by field name, parse to type
            g: map.get("g")?.parse().ok()?,
            b: map.get("b")?.parse().ok()?,
        })
    }
}

// Usage: identical to how serde works
let red = Color { r: 255, g: 0, b: 0 };
let serialized = red.serialize();            // "b=0|g=0|r=255" (sorted, deterministic)
let decoded = Color::deserialize(&serialized).unwrap();
assert_eq!(red, decoded);
```

Real `serde` uses a `Serializer` trait (not a `HashMap`) to support many output formats without re-implementing the `Serialize` trait. The proc macro generates the same `serialize_fields`-equivalent code — just against `serde`'s `Serializer` interface instead of a `HashMap`.

## What This Unlocks

- **Understanding derive macro output** — once you see that `#[derive(Serialize)]` just generates a field-by-field loop, manual `impl Serialize` becomes straightforward; you know exactly what to write when the macro falls short.
- **Custom serialization hooks** — `#[serde(rename = "field_name")]`, `#[serde(skip)]`, and `#[serde(with = "module")]` are all ways to customize what the generated code does; understanding the generated code makes these attributes intuitive.
- **Format independence** — the `Serializer` trait abstraction (in real serde) means your `Serialize` impl works for JSON, TOML, MessagePack, and custom binary formats without changing the struct code.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Serialization derive | `ppx_sexp_conv`, `ppx_yojson_conv` | `#[derive(Serialize, Deserialize)]` from `serde` |
| Proc macro output | Generated OCaml code | Generated Rust `impl Trait` blocks |
| Field name access | Runtime via `ppx`-generated code | Compile-time string literals in the generated impl |
| Format abstraction | Format-specific ppx per format | One `Serialize` trait, many `Serializer` implementations |
