# 773: Serde Attributes: rename, skip, flatten Patterns

**Difficulty:** 3  **Level:** Intermediate

Manually implement what `#[serde(rename)]`, `#[serde(skip)]`, and `#[serde(flatten)]` do — then understand why the real `serde` attributes are so powerful.

## The Problem This Solves

When you write `#[serde(rename = "id")]` on a field, you're telling `serde`'s derive macro to generate serialization code that uses `"id"` in JSON instead of the Rust field name `user_id`. But if this is just generated code, what does that code actually look like? And when the generated code does something unexpected, how do you reason about it?

This example implements `rename`, `skip`, and `flatten` *manually* using a custom `Serialize`/`Deserialize` trait pair. Seeing the explicit code makes the attribute semantics concrete: `rename` is a key substitution, `skip` is an `if` branch that omits a field, and `flatten` is merging one struct's fields into the parent map.

Understanding the manual version makes `#[serde(default)]`, `#[serde(with = "...")]`, and field-level attribute combinations easy to reason about.

## The Intuition

Serialization is "write struct fields into a map." `rename` means write under a different key. `skip` means don't write at all. `flatten` means write the nested struct's fields *directly into the same map* rather than as a nested object.

Deserialization reverses each: `rename` reads from the renamed key, `skip` uses a default value (the field wasn't in the map), `flatten` reads the nested struct's fields from the same map level.

## How It Works in Rust

**The domain types:**
```rust
struct User {
    user_id:  u32,    // rename → "id"
    name:     String,
    password: String, // skip — never written
    address:  Address,// flatten — fields merged into User's map
}
```

**`rename` in serialize** — write under the new key:
```rust
out.insert("id".into(), self.user_id.to_string());  // not "user_id"
```

**`skip` in serialize** — simply don't insert:
```rust
// password field: nothing written
// name field:
out.insert("name".into(), self.name.clone());
```

**`flatten` in serialize** — delegate to the nested struct's serializer:
```rust
self.address.serialize_to(out);  // merges street, city directly into `out`
```
`Address::serialize_to` writes `"street"` and `"city"` keys into the same `HashMap` as `User`'s fields.

**`rename` in deserialize** — read from the new key:
```rust
let user_id = map.get("id")?.parse().ok()?;  // read "id", not "user_id"
```

**`skip` / `default` in deserialize** — use a default value:
```rust
let password = String::new();  // #[serde(default)] — field absent → empty string
```

**`flatten` in deserialize** — read the nested struct from the same map:
```rust
let address = Address::deserialize_from(map)?;  // reads "street", "city" from User's map
```

**The resulting wire format:**
```
city=Berlin|id=42|name=Alice|street=Main St 1
// No "password", no "address" wrapper — flattened!
```

## What This Unlocks

- **Debugging `serde` derive output** — when `#[serde(flatten)]` produces unexpected JSON, you know exactly what code it generates.
- **Custom serialization without proc-macros** — for types where derive doesn't fit, implement `Serialize`/`Deserialize` manually using this pattern.
- **Understanding attribute combinations** — `#[serde(rename = "id", skip_serializing_if = "Option::is_none")]` is just two of these transformations composed.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Serialization | `ppx_sexp_conv`, `ppx_yojson_conv` | `serde` derive macros |
| Field rename | `[@key "id"]` (ppx attribute) | `#[serde(rename = "id")]` |
| Skip field | `[@sexp.option]` / custom | `#[serde(skip)]` |
| Flatten nested struct | Not standard; manual | `#[serde(flatten)]` — merges into parent object |
| Default on missing | `[@default ...]` | `#[serde(default)]` — calls `Default::default()` |
