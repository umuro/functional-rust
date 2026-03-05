📖 **[View on hightechmind.io →](https://hightechmind.io/rust/767-versioned-data-format)**

---

# 767: Versioned Serialization with Migration

**Difficulty:** 4  **Level:** Advanced

Handle schema evolution — read old data with new code by versioning your wire format and building a migration chain.

## The Problem This Solves

Your data format changes over time: you add fields, rename them, change types. Old data still exists on disk, in databases, in message queues. New code must be able to read old data without breaking. This is **schema evolution**, and it's one of the hardest problems in production systems.

The naive solution (add nullable fields to one struct and handle all versions in one place) leads to increasingly messy code with many `if version >= 2` branches. The clean solution is a version-tagged format with a migration chain: each version is a separate type, `From` conversions migrate between adjacent versions, and a `VersionedUser` enum acts as a parsing discriminant. Reading always deserializes into the appropriate versioned type, then migrates to the current version in one `.into_current()` call.

This pattern is used in database migration frameworks, Avro/Protobuf schema registries, event sourcing systems, and any long-lived storage format. The `serde` equivalent uses `#[serde(default)]` and `Option<T>` for additive changes, and explicit version tags for breaking changes.

## The Intuition

Version the serialized format with a `version=N` field. Parse into a `VersionedUser::V1`, `V2`, or `V3` enum variant based on that tag. Implement `From<V1> for V2`, `From<V2> for V3`, and `From<V1> for V3` (transitive chain). A single method `.into_current()` converts any version to the latest by following the chain. New fields get sensible defaults in `From` implementations.

## How It Works in Rust

```rust
// Each version is its own type — no shared mutable fields
struct UserV1 { name: String, age: u32 }
struct UserV2 { name: String, age: u32, email: String }    // added field
struct UserV3 { name: String, age: u32, email: String, active: bool }  // added field

// Migration: From<older> for newer with sensible defaults
impl From<UserV1> for UserV2 {
    fn from(u: UserV1) -> Self {
        UserV2 {
            email: format!("{}@example.com", u.name.to_lowercase()),
            name: u.name,
            age: u.age,
        }
    }
}

impl From<UserV2> for UserV3 {
    fn from(u: UserV2) -> Self {
        UserV3 { active: true, name: u.name, age: u.age, email: u.email }
    }
}

// Enum as parsing discriminant
enum VersionedUser { V1(UserV1), V2(UserV2), V3(UserV3) }

impl VersionedUser {
    fn into_current(self) -> UserV3 {
        match self {
            VersionedUser::V1(u) => UserV3::from(u),  // two migrations
            VersionedUser::V2(u) => UserV3::from(u),  // one migration
            VersionedUser::V3(u) => u,                // already current
        }
    }
}

// Deserializer reads version tag, dispatches to correct parser
fn deserialize(s: &str) -> Result<VersionedUser, DeError> {
    let map = fields(s);
    match map.get("version") {
        Some("1") => Ok(VersionedUser::V1(parse_v1(&map)?)),
        Some("2") => Ok(VersionedUser::V2(parse_v2(&map)?)),
        Some("3") => Ok(VersionedUser::V3(parse_v3(&map)?)),
        Some(v)   => Err(DeError::UnsupportedVersion(v.to_string())),
        None      => Err(DeError::MissingField("version")),
    }
}
```

The `into_current()` pattern means migration logic is in `From` implementations — easily testable, composable, and free of if/else chains.

## What This Unlocks

- **`From` as migration** — Rust's `From`/`Into` traits are the natural hook for data migration; the orphan rule means you can implement migration chains without modifying either type.
- **Additive vs. breaking changes** — additive changes (new optional fields) can use `Option<T>` or `#[serde(default)]`; breaking changes (renamed fields, type changes) require a new version number and migration.
- **Unknown version handling** — always return `Err(UnsupportedVersion)` for unknown version tags; fail loudly rather than silently producing garbage data.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Version enum | Variant per version | `enum VersionedUser { V1(...), V2(...), V3(...) }` |
| Migration chain | Function composition | `From<V1> for V2` + `From<V2> for V3` |
| Default field values | `Option.value ~default:` | `#[serde(default)]` or explicit `From` defaults |
| Unknown version | Exception | `Err(UnsupportedVersion(v.to_string()))` — typed error |
