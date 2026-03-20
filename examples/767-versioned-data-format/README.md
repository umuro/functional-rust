📖 **[View on hightechmind.io →](https://hightechmind.io/rust/767-versioned-data-format)**

---

# 767-versioned-data-format — Versioned Data Format
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Long-lived systems must evolve their data formats without breaking existing data. A V1 file written in 2020 must still be readable by a V3 application in 2025. This requires explicit version negotiation, backward-compatible additions (new optional fields with defaults), forward-compatible reading (ignoring unknown fields), and migration functions for format upgrades. Protocol Buffers, Avro, and Thrift all have sophisticated solutions; this example shows the principles in pure Rust.

## Learning Outcomes

- Model multiple format versions as distinct structs (`DataV1`, `DataV2`, `DataV3`)
- Implement migration functions: `DataV1 -> DataV2`, `DataV2 -> DataV3`
- Use a `Version` struct for compatibility checking (same major = compatible)
- Implement a unified `Data` enum that can hold any version
- Write tests that verify both backward compatibility and migration correctness

## Rust Application

`DataV1` has `name: String, value: i32`. `DataV2` adds `tags: Vec<String>`. `DataV3` changes `value` to `f64` and adds `metadata: HashMap<String, String>`. `Version(major, minor)` implements `is_compatible` as same-major check. Migration functions convert upward: `v1_to_v2(DataV1) -> DataV2` (empty tags default), `v2_to_v3(DataV2) -> DataV3` (value cast, empty metadata). `Data::upgrade()` applies all migrations to reach V3.

## OCaml Approach

OCaml's `Bin_prot` handles versioning through `Versioned` modules: each version has a `bin_read_t` and migrations are explicit functions. Jane Street uses this pervasively in their trading infrastructure. OCaml's `ppx_sexp_conv` generates S-expression serializers per version; custom deserialization reads the version field first and dispatches. `Protobuf` bindings for OCaml (`ocaml-protoc`) provide language-agnostic versioning.

## Key Differences

1. **Version field**: Both languages encode the version as an explicit field; Rust's `u8` pair vs. OCaml's `Int.t` are equivalent.
2. **Migration chain**: Rust's explicit `v1_to_v2`/`v2_to_v3` functions mirror OCaml's migration module pattern.
3. **Schema evolution**: Protocol Buffers and Avro handle versioning at the schema level; this example handles it in application code.
4. **Backward compatibility**: Both languages support optional fields with defaults, though Rust's `Option<T>` and OCaml's `option 't` require explicit handling.

## Exercises

1. Implement `DataV4` that adds a `priority: u8` field with a default of 0 and write a `v3_to_v4` migration.
2. Add binary serialization for each version and implement `read_any_version(bytes: &[u8]) -> Result<DataV3, Error>` that reads the version byte, deserializes, and migrates.
3. Write a compatibility matrix test that verifies: V1 data can migrate to V3, V2 data can migrate to V3, but V3 data cannot be downgraded to V1 (return an error).
