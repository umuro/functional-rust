📖 **[View on hightechmind.io →](https://hightechmind.io/rust/759-manual-serialize-trait)**

---

# 759: Manual Serialize/Deserialize Trait Implementation

**Difficulty:** 3  **Level:** Intermediate

Implement `Serialize` and `Deserialize` traits by hand — understand what `serde` generates, build a custom wire format, and see the visitor pattern in action.

## The Problem This Solves

Most Rust projects use `serde` with `#[derive(Serialize, Deserialize)]` — which is excellent for JSON, TOML, bincode, and dozens of other formats. But there are situations where you need a custom wire format: a binary protocol, a legacy text format, a space-constrained embedded system, or a format that serde doesn't support.

Even when you use serde, understanding what it *does* makes you a better user of it. The `Serialize` trait, the `Serializer` interface, and the visitor pattern are the same whether the framework generates them or you write them. When serde's derive macro doesn't do what you need, you implement the trait manually — and this example shows exactly how.

There's also a practical need for lightweight serialization that avoids `serde`'s compilation overhead. If you're writing a library with minimal dependencies, rolling a simple key=value format can be the right call.

## The Intuition

Think of Python's `__repr__` and custom `json.JSONEncoder` — you define how your type serializes to a string, and a matching parser that reconstructs it. In Rust, you formalize this as a pair of traits with a wire format contract between them.

The key insight is the *round-trip property*: `deserialize(serialize(x)) == x` must hold for all valid values. This is what you test. Once the round-trip holds — including for edge cases like strings containing the delimiter — you can trust the format.

Escape handling is where custom formats usually fail. If your delimiter is `|` and your string can contain `|`, you need escaping. This example shows a complete escape/unescape cycle.

## How It Works in Rust

```rust
// The trait pair — serialize to String, deserialize from &str
pub trait Serialize {
    fn serialize(&self) -> String;
}

pub trait Deserialize: Sized {
    fn deserialize(input: &str) -> Result<Self, SerError>;
}

// Implement for your domain type
#[derive(Debug, PartialEq)]
pub struct Person { pub name: String, pub age: u32, pub active: bool }

impl Serialize for Person {
    fn serialize(&self) -> String {
        // name=Alice|Wonder → escape | in name as \|
        format!("name={}|age={}|active={}",
            escape(&self.name),
            self.age,
            self.active)
    }
}

impl Deserialize for Person {
    fn deserialize(input: &str) -> Result<Self, SerError> {
        let fields = parse_fields(input);  // splits on unescaped |, then on =

        let name = fields.get("name")
            .ok_or_else(|| SerError::MissingField("name".into()))?
            .clone();

        let age = fields.get("age")
            .ok_or_else(|| SerError::MissingField("age".into()))?
            .parse::<u32>()
            .map_err(|e| SerError::ParseError(e.to_string()))?;

        // ... active field similarly
        Ok(Person { name, age, active })
    }
}

// Generic round-trip — works for any Serialize + Deserialize type
fn round_trip<T: Serialize + Deserialize + Debug>(value: &T) -> Result<T, SerError> {
    T::deserialize(&value.serialize())
}

// Test: special characters in string fields survive the round-trip
#[test]
fn round_trip_special_chars() {
    let p = Person { name: "Pi|pe".to_string(), age: 1, active: false };
    let decoded = round_trip(&p).unwrap();
    assert_eq!(p, decoded);  // "|" in name must survive escape/unescape
}
```

Key points:
- Design the wire format first; implement escape/unescape before serialization
- The `parse_fields` helper returns a `HashMap<&str, String>` — look up each field, report missing as a specific error
- `SerError` has distinct variants for `MissingField` vs `ParseError` — callers know whether data was absent or malformed
- The `Deserialize: Sized` bound is needed because you're returning `Self` by value

## What This Unlocks

- **Custom protocols**: implement any text or binary wire format — legacy systems, compact embedded formats, domain-specific protocols
- **Understand serde**: after writing this by hand, `#[derive(Serialize, Deserialize)]` stops being magic and starts being familiar
- **No-dependency serialization**: embed a simple format in a library without pulling in `serde`, `serde_json`, and their transitive dependencies

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Serialization | `yojson`, `marshal`, or custom ppx | `serde` with `#[derive]` or manual trait impl |
| Trait pair | Module signature with `encode`/`decode` | `Serialize` + `Deserialize` traits |
| Missing field error | Exception or `Not_found` | `SerError::MissingField(field_name)` |
| Parse error | `Failure` exception | `SerError::ParseError(msg)` |
| Round-trip test | Custom equality check | `assert_eq!(original, round_trip(&original).unwrap())` |
| Escape handling | Manual string manipulation | Same — no magic, must be explicit |
