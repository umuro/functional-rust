📖 **[View on hightechmind.io →](https://hightechmind.io/rust/765-csv-parsing-pattern)**

---

# 765-csv-parsing-pattern — CSV Parsing Pattern
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

CSV (Comma-Separated Values) is the most common format for tabular data exchange — spreadsheets, database exports, analytics pipelines. Despite its simplicity, CSV has edge cases that trip up naive implementations: quoted fields containing commas, escaped quotes inside quoted fields, variable column counts, and different line endings (CRLF vs LF). Building a correct CSV parser teaches state machine design and the importance of handling edge cases explicitly.

## Learning Outcomes

- Implement a CSV parser that handles quoted fields with embedded commas and newlines
- Detect and report inconsistent column counts across rows
- Parse rows as `Vec<String>` with proper unquoting and un-escaping
- Return typed `CsvError` variants for unterminated quotes and column count mismatches
- Understand why the `csv` crate exists and what it handles that this example omits (BOM, CRLF, etc.)

## Rust Application

`parse_csv` splits into rows and calls `parse_row` per line. `parse_row` implements a state machine that tracks whether it is inside quotes (`in_quote`) and handles `""` as an escaped quote. The column count of the first non-empty row is used as the expected count; subsequent rows that differ produce `InconsistentColumns`. `CsvError` has two variants: `UnterminatedQuote(line_num)` and `InconsistentColumns { expected, got, line }`.

## OCaml Approach

OCaml's `Csv` library handles CSV parsing with RFC 4180 compliance. The `csv-sxml` library converts parsed CSV to XML. `Csvfields` (Jane Street) generates typed record accessors from CSV headers. The parser pattern in OCaml uses `Buffer.t` for field accumulation and explicit state variables, structurally identical to the Rust implementation.

## Key Differences

1. **State machine**: Both languages implement CSV parsing as a character-by-character state machine with explicit quote tracking.
2. **Buffer accumulation**: Rust uses a `String` and `push` for field building; OCaml uses `Buffer.t` and `Buffer.add_char` — equivalent patterns.
3. **Error types**: Rust's typed `CsvError` enum; OCaml's `Csv` library raises exceptions with string messages.
4. **Streaming**: The `csv` crate supports streaming (iterator over rows); OCaml's `Csv.of_channel` does the same; this example parses the whole string at once.

## Exercises

1. Add support for Windows line endings (CRLF) by stripping trailing `\r` before processing each line.
2. Implement `parse_csv_with_headers` that treats the first row as column names and returns `Vec<HashMap<String, String>>` instead of `Vec<Vec<String>>`.
3. Write a streaming CSV parser using `Iterator` that yields one row at a time, suitable for processing files larger than available memory.
