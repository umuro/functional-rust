📖 **[View on hightechmind.io →](https://hightechmind.io/rust/763-json-format-from-scratch)**

---

# 763-json-format-from-scratch — JSON Format From Scratch
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

JSON (JavaScript Object Notation) was introduced in 2001 and is now the universal data interchange format. Building a JSON serializer from scratch teaches you about recursive data structures, string escaping, number formatting, and the performance trade-offs in text-based formats. Understanding JSON's structure also helps when working with serde's JSON support and when debugging serialization issues in production systems.

## Learning Outcomes

- Represent JSON as a recursive `JsonValue` enum: Null, Bool, Number, String, Array, Object
- Implement `to_json(&self) -> String` for compact output
- Implement `to_json_pretty(&self, indent: usize) -> String` with configurable indentation
- Handle string escaping: `"`, `\`, newlines, control characters
- Understand JSON number formatting: integers vs. floats, scientific notation edge cases

## Rust Application

`JsonValue` is an enum with six variants. `to_json` recursively serializes: null/bool/number directly, strings with `escape_json_string`, arrays with comma-joined items, objects with `"key": value` pairs. `to_json_pretty` uses `to_json_indent(level, indent)` to track nesting depth. `escape_json_string` handles `"`, `\`, newlines, tabs, and Unicode escapes. Number formatting avoids scientific notation for integers under 1e15.

## OCaml Approach

OCaml's `Yojson` library represents JSON as a variant type similar to `JsonValue`. Encoding uses `Yojson.Safe.to_string` and `Yojson.Safe.pretty_to_string`. Custom serialization uses the `to_basic` function to convert from library types. OCaml's `Jsonaf` (Jane Street) provides a high-performance alternative. Both OCaml JSON libraries handle Unicode and number formatting edge cases that this from-scratch example omits.

## Key Differences

1. **Recursion**: Both represent JSON as recursive algebraic data types (enums in Rust, variants in OCaml); the recursive `to_string` pattern is identical.
2. **Object ordering**: Rust's example uses `Vec<(String, JsonValue)>` to preserve insertion order; OCaml's `Yojson` uses an association list with the same property.
3. **Number handling**: JSON numbers are IEEE 754 doubles; both languages face the same `1.0 vs 1` formatting challenge.
4. **Performance**: Production JSON libraries (Rust's `simd-json`, OCaml's `jsonaf`) use SIMD for bulk scanning; this from-scratch version prioritizes clarity.

## Exercises

1. Implement `from_json(s: &str) -> Result<JsonValue, ParseError>` — a simple recursive descent JSON parser for the types in the `JsonValue` enum.
2. Add `merge_objects` that combines two `JsonValue::Object` values, with the second's values overriding the first's for duplicate keys.
3. Implement `json_path_get(root: &JsonValue, path: &str) -> Option<&JsonValue>` that navigates a dot-separated path like `"users.0.name"` through a JSON tree.
