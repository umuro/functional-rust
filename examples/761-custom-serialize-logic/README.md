# 761: Custom Serialization for Complex Types

**Difficulty:** 3  **Level:** Intermediate

Implement `Serialize` manually for enums with payloads, `Option<T>`, and `Vec<T>` — shapes that derive macros handle awkwardly.

## The Problem This Solves

`#[derive(Serialize)]` works perfectly for simple structs. It struggles with enums that have complex payloads you want serialized in a specific wire format, newtypes wrapping collections, and cases where you need length-prefixed encoding or custom delimiters. When you need to control exactly how bytes land on the wire — for network protocols, compact binary formats, or compatibility with legacy systems — you implement `Serialize` manually.

This is the pattern behind custom `serde::Serialize` impls in production: Tokio's codec types, custom binary protocol handlers, and any library that needs a specific wire format rather than `serde_json`'s default choices. Once you understand how to hand-roll a serializer for an enum, the `serde` docs on custom serializers become clear.

The key challenge with enums is that each variant has a different shape. You need a tag (to identify the variant on deserialization) and then variant-specific payload serialization. Length-prefixed strings (`"5:hello"`) are a common technique for variable-length fields in binary protocols.

## The Intuition

Serialization is a function `value → bytes/string`. For enums, precede each variant's data with a tag character (`'C'` for Circle, `'R'` for Rectangle, `'P'` for Point). For strings, prefix with the byte count so the deserializer knows where the string ends. For `Option<T>`, use `'N'` for None and `'S' + payload` for Some. This self-describing approach lets the deserializer parse without knowing the type in advance.

## How It Works in Rust

```rust
pub trait Serialize {
    fn serialize(&self, out: &mut String);
}

// Enum with tagged payloads
impl Serialize for Shape {
    fn serialize(&self, out: &mut String) {
        match self {
            Shape::Circle(r) => {
                out.push_str("C|");    // tag: Circle
                r.serialize(out);      // delegate to f64's Serialize
            }
            Shape::Rectangle { width, height } => {
                out.push_str("R|");
                width.serialize(out);
                out.push('|');
                height.serialize(out);
            }
            Shape::Point => out.push('P'),   // no payload needed
        }
    }
}

// Length-prefixed string encoding: "5:hello"
impl Serialize for String {
    fn serialize(&self, out: &mut String) {
        write!(out, "{}:{}", self.len(), self).unwrap();  // length:data
    }
}

// Option<T>: N for None, S + payload for Some
impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, out: &mut String) {
        match self {
            None    => out.push('N'),
            Some(v) => { out.push('S'); v.serialize(out); }
        }
    }
}

// Vec<Shape> with count header for framing
fn serialize_shapes(shapes: &[Shape]) -> String {
    let mut out = String::new();
    write!(out, "{}\n", shapes.len()).unwrap();  // count header
    for s in shapes { s.serialize(&mut out); out.push('\n'); }
    out
}
```

Passing `&mut String` is idiomatic for push-based serialization — avoids allocation per field, builds the output incrementally. The `Deserialize` counterpart uses a cursor (`&str` that gets consumed as tokens are parsed) to mirror the format.

## What This Unlocks

- **Manual `serde::Serialize` impl** — once you understand tagged-variant encoding, implementing `serde::Serializer` calls for enum variants is straightforward; the pattern maps directly.
- **Custom wire formats** — length-prefixed encoding, delimited formats, and tagged-union protocols are all expressible with this pattern; you control every byte.
- **Generic `Option<T>` and `Vec<T>` impls** — the trait system lets you write one impl that works for any serializable type, paralleling how `serde` derives work transitively through generic types.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Enum serialization | Pattern match + write tag | Same — `match` on variant, push tag + payload |
| Length-prefixed strings | `Printf.sprintf "%d:%s"` | `write!(out, "{}:{}", len, s)` — same idea |
| Generic impls | Functors or polymorphic variants | `impl<T: Serialize> Serialize for Option<T>` |
| Output buffer | `Buffer.t` | `&mut String` — zero-copy append via `push_str` |
