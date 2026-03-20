📖 **[View on hightechmind.io →](https://hightechmind.io/rust/175-json-parser)**

---

# Complete JSON Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

JSON is the universal data interchange format — REST APIs, configuration files, log streams, and inter-service communication all use it. Parsing JSON correctly requires handling all six value types (null, boolean, number, string, array, object), Unicode escape sequences in strings, arbitrary nesting, and proper whitespace handling. This capstone example applies every parser combinator technique from the series to build a complete, correct JSON parser.

## Learning Outcomes

- Build a complete JSON parser using all previously learned combinator techniques
- Handle JSON string escape sequences: `\"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, `\t`, `\uXXXX`
- See how recursive `array` and `object` parsers combine with all primitive parsers
- Understand JSON as a real-world benchmark for parser combinator expressiveness

## Rust Application

The `Json` enum has six variants matching the JSON spec. Parsing hierarchy: `null` and booleans use `tag`; `number` uses the float parser from example 164; `string` handles escape sequences character by character; `array` is `delimited('[', separated_list0(',', json_value), ']')` with whitespace; `object` is key-value pairs with `separated_list0`. The top-level `json_value` uses `choice` over all six in an order that avoids ambiguity.

## OCaml Approach

OCaml's ecosystem provides `yojson` (opam) for production JSON parsing. A hand-written JSON parser in OCaml follows the identical structure. OCaml's `Buffer.t` efficiently accumulates string characters during escape sequence handling. The `Uchar` module handles `\uXXXX` Unicode escapes. `angstrom`'s streaming API handles JSON streams without loading the entire document into memory.

## Key Differences

1. **Unicode escapes**: `\uXXXX` requires converting to UTF-8 bytes; both languages need explicit handling beyond simple character matching.
2. **Number format**: JSON numbers are a subset of IEEE 754 — no `NaN`, no `Infinity`, no leading zeros for integers; both parsers should enforce these restrictions.
3. **Streaming**: Rust's `serde_json` supports streaming via `Deserializer`; OCaml's `yojson` has a streaming API; these examples parse complete strings.
4. **Strictness**: Production JSON parsers must reject trailing commas, duplicate keys, and other common extensions; these examples may be lenient.

## Exercises

1. Add `\uXXXX` Unicode escape sequence handling in string parsing, producing correct UTF-8 bytes.
2. Make the number parser reject leading zeros (e.g., `"01"` is invalid JSON).
3. Implement `json_to_string(json: &Json) -> String` that serializes a `Json` value back to a valid JSON string.
