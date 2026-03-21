📖 **[View on hightechmind.io →](https://hightechmind.io/rust/761-custom-serialize-logic)**

---

# 761-custom-serialize-logic — Custom Serialize Logic
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

`#[derive(Serialize)]` handles the common case, but sometimes you need custom serialization: dates formatted as ISO strings rather than struct fields, passwords skipped entirely, amounts rounded before serialization, or opaque identifiers encoded as base64. Custom serialization implements `Serialize` (or `serde::Serialize` in production) manually, giving complete control over the wire format.

## Learning Outcomes

- Implement custom serialization for a `Date` type as ISO 8601 string vs. compact integer
- Serialize `Option` fields as null/value with custom null representation
- Skip sensitive fields (`password_hash`) during serialization
- Implement custom deserialization that parses ISO strings back to structured `Date` values
- See how custom serialization enables format versioning and data migration

## Rust Application

`Date { year, month, day }` implements two serialization formats: `to_iso_string()` produces `"2024-01-15"` and `to_compact()` produces `20240115`. `from_iso_string` and `from_compact` provide round-trip deserialization. A custom `User` serializer skips `password_hash` and renders `display_name: Option<String>` as either a string or `null`. The example builds a mini JSON serializer that respects these custom behaviors.

## OCaml Approach

OCaml's `ppx_sexp_conv` allows custom `sexp_of_t` implementations that override the generated one. For JSON, `ppx_yojson_conv` supports `[@yojson.option]` and `[@yojson.key "name"]` attributes. Completely custom serialization replaces the generated function with a hand-written one in the same module. `Bin_prot` similarly allows custom `bin_write_t`/`bin_read_t` implementations.

## Key Differences

1. **Serde attributes**: Rust's `#[serde(skip)]`, `#[serde(rename = "...")]`, `#[serde(with = "...")]` provide common custom behaviors without full manual implementation; OCaml's equivalents are `[@yojson.key]`, `[@sexp.opaque]`.
2. **Format independence**: Rust's custom `Serialize` impl works across all serde formats; OCaml's custom functions are typically format-specific.
3. **with module**: Rust's `#[serde(with = "timestamp_seconds")]` delegates to a module with `serialize`/`deserialize` functions; OCaml has no direct equivalent.
4. **Versioning**: Custom serialization enables format evolution (V1 → V2 migration); both languages use this for long-lived protocols.

## Exercises

1. Implement a `Money { amount_cents: i64, currency: &str }` type with custom serialization as `"100.00 USD"` and deserialization that parses that string format.
2. Write a custom serializer for `IpAddr` that serializes IPv4 as a 4-byte array and IPv6 as a 16-byte array in binary formats.
3. Implement `serialize_with_version` that adds a `"_version": 2` field to any struct's JSON output, enabling format evolution detection during deserialization.
