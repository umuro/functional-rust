📖 **[View on hightechmind.io →](https://hightechmind.io/rust/171-csv-parser)**

---

# CSV Parser
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

CSV (Comma-Separated Values) is the most universally used data interchange format — spreadsheets, database exports, data pipelines. Despite its apparent simplicity, correct CSV parsing is subtle: fields may be quoted (allowing commas inside), quotes inside quoted fields are doubled (`"She said ""hello"""`), and line endings may be `\n` or `\r\n`. This example builds a complete RFC 4180 compliant CSV parser from combinators.

## Learning Outcomes

- Build a complete CSV parser handling unquoted fields, quoted fields, and embedded commas
- Understand RFC 4180's quoting rules: `"` is escaped as `""` inside quoted fields
- See how `choice` selects between quoted and unquoted field parsers
- Appreciate CSV as a real-world example of a deceptively complex format

## Rust Application

Two field parsers: `unquoted_field` scans until `,` or newline; `quoted_field` parses `"..."` with `""` → `"` unescaping inside. The `field` parser is `choice([quoted_field, unquoted_field])`. A row is `separated_list1(char_parser(','), field)`. The full CSV parser applies `many0(row)` separated by newlines. Carriage returns are handled by stripping `\r` before `\n`. The result is `Vec<Vec<String>>` — rows of fields.

## OCaml Approach

OCaml's ecosystem provides `csv` (opam package) for production use. A hand-written CSV parser follows the same combinator structure: `field = quoted_field <|> unquoted_field`, `row = sep_by (char ',') field`. OCaml's `Buffer` type is used for efficient string accumulation in quoted fields, avoiding repeated string concatenation. The `Scanf` module provides an alternative imperative approach.

## Key Differences

1. **Quoting**: Both handle the `""` → `"` unescaping inside quoted fields — this is the trickiest part of CSV and requires careful state tracking.
2. **CRLF handling**: RFC 4180 requires `\r\n`; both parsers handle this, though implementations differ in detail.
3. **Performance**: Production CSV parsers (Rust's `csv` crate, OCaml's `csv` package) use streaming I/O; these examples parse the entire string in memory.
4. **Headers**: Neither example handles header rows specially; production code typically parses the first row as column names.

## Exercises

1. Add support for the `\r\n` line ending in addition to `\n`.
2. Implement a `csv_with_headers` parser that returns `Vec<HashMap<String, String>>` using the first row as column names.
3. Handle the empty string edge case: `""` in a CSV is a quoted empty field, not absent.
