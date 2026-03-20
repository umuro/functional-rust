📖 **[View on hightechmind.io →](https://hightechmind.io/rust/773-serde-attributes-concept)**

---

# 773-serde-attributes-concept — Serde Attributes Concept

## Problem Statement

`serde`'s attribute system — `#[serde(rename = "...")]`, `#[serde(skip)]`, `#[serde(default)]`, `#[serde(flatten)]` — transforms how types map to their serialized representation. Understanding these attributes is essential for working with external JSON APIs, legacy formats, and versioned protocols. This example demystifies serde attributes by showing what behavior they encode without the actual serde crate, making the mental model clear.

## Learning Outcomes

- Understand `#[serde(rename = "user_name")]` — changes the JSON key without changing the Rust field name
- Understand `#[serde(skip)]` — omits a field from serialization (useful for derived/sensitive data)
- Understand `#[serde(default)]` — uses `Default::default()` when the field is absent during deserialization
- Understand `#[serde(flatten)]` — inlines a nested struct's fields into the parent object
- Map each attribute to the code it conceptually generates

## Rust Application

`User` has fields with simulated attribute configurations stored in `FieldConfig` structs. `field_configs()` returns a `HashMap` mapping field names to their configuration. A serializer interprets these configs: `rename` maps to an alternate JSON key name, `skip` causes the field to be omitted, `default` causes missing fields to use `Default::default()`. The `serialize_user` function demonstrates how a derive macro would use the configs to produce the correct JSON key mapping.

## OCaml Approach

OCaml's `ppx_sexp_conv` attributes: `[@sexp.opaque]` hides a field, `[@sexp_list]` treats a field as a list, `[@default 0]` provides a default. `ppx_yojson_conv` uses `[@yojson.option]`, `[@key "name"]`, and `[@yojson.drop_default]`. Unlike serde's unified attribute system, OCaml ppx attributes are ppx-specific and must be duplicated per serialization format.

## Key Differences

1. **Unified vs per-format**: Serde's attributes work across all formats (JSON, TOML, YAML, MessagePack); OCaml requires separate attribute sets per ppx.
2. **Runtime simulation**: This example simulates attribute effects at runtime via `FieldConfig`; real serde processes attributes at compile time via proc macros.
3. **flatten**: Serde's `#[serde(flatten)]` is notoriously complex to implement; OCaml's ppx_yojson_conv doesn't support it natively.
4. **Default values**: Serde's `#[serde(default = "fn_name")]` calls a custom function; OCaml's `[@default expr]` evaluates an expression at type definition time.

## Exercises

1. Implement a `serialize_with_rename(config: &FieldConfig, field_name: &str) -> &str` helper that returns the wire name — using `config.rename` if set, otherwise `field_name`.
2. Write a `deserialize_user(json: &HashMap<String, String>) -> User` that applies `default` and `rename` configs during deserialization.
3. Simulate `#[serde(skip_serializing_if = "Option::is_none")]` by adding a `skip_if_none: bool` field to `FieldConfig` and implementing the conditional skip logic.
