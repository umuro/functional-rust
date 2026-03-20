[csv-writer on hightechmind.io](https://hightechmind.io/posts/functional-rust/csv-writer)

---

## Problem Statement

Implement a CSV writer that correctly escapes fields: any field containing a comma, double-quote, newline, or carriage return must be wrapped in double quotes, with internal double-quotes doubled (`"` → `""`). Implement field escaping, single-row serialization, and multi-row document serialization.

## Learning Outcomes

- Implement `needs_quoting(s: &str) -> bool` to detect when a field requires quoting
- Implement `escape_field`: wrap in quotes and double internal `"` characters
- Use `fields.iter().map(escape_field).collect::<Vec<_>>().join(",")` for row serialization
- Handle both `&str` and `String` input variants
- Understand the RFC 4180 CSV quoting rules: quote if and only if the field contains `,`, `"`, `\n`, or `\r`

## Rust Application

```rust
pub fn needs_quoting(s: &str) -> bool {
    s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r')
}

pub fn escape_field(s: &str) -> String {
    if needs_quoting(s) {
        let mut out = String::with_capacity(s.len() + 2);
        out.push('"');
        for c in s.chars() {
            if c == '"' { out.push('"'); }  // double the quote
            out.push(c);
        }
        out.push('"');
        out
    } else {
        s.to_string()
    }
}

pub fn write_row(fields: &[&str]) -> String {
    fields
        .iter()
        .map(|f| escape_field(f))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn write_csv(rows: &[Vec<String>]) -> String {
    rows.iter()
        .map(|row| write_row_owned(row))
        .collect::<Vec<_>>()
        .join("\n")
}
```

`String::with_capacity(s.len() + 2)` pre-allocates for the content plus the two surrounding quotes, avoiding reallocations for the common case where the content has no additional `"` characters to double.

The doubling rule: for each `"` in the original, emit `""`. The outer quotes are added once at the start and end. So `say "hello"` becomes `"say ""hello"""`.

## OCaml Approach

```ocaml
let needs_quoting s =
  String.contains s ',' || String.contains s '"' ||
  String.contains s '\n' || String.contains s '\r'

let escape_field s =
  if not (needs_quoting s) then s
  else begin
    let buf = Buffer.create (String.length s + 2) in
    Buffer.add_char buf '"';
    String.iter (fun c ->
      if c = '"' then Buffer.add_char buf '"';
      Buffer.add_char buf c
    ) s;
    Buffer.add_char buf '"';
    Buffer.contents buf
  end

let write_row fields =
  String.concat "," (List.map escape_field fields)

let write_csv rows =
  String.concat "\n" (List.map write_row rows)
```

OCaml's `String.contains` provides a built-in single-character search, slightly cleaner than Rust's `.contains(char)`. The `Buffer`-based approach avoids intermediate string allocations during escape processing.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Quote detection | `.contains('"')` etc. | `String.contains s '"'` etc. |
| String builder | `String::with_capacity` + `push` | `Buffer` + `add_char` |
| Row joining | `.join(",")` | `String.concat ","` |
| Multi-row | `.join("\n")` | `String.concat "\n"` |
| Slice vs list | `&[&str]` — zero-copy borrowed slice | `string list` — linked list |

The writer and parser (958) form a pair that should round-trip: any `Vec<Vec<String>>` serialized with `write_csv` and parsed with `parse_csv_line` row-by-row should recover the original data exactly.

## Exercises

1. Verify round-trip correctness: write a property test with random strings (including commas and quotes) and confirm `parse(write(data)) == data`.
2. Add an `append_row` variant that writes to a `Write` trait implementation (e.g., `std::io::stdout()`) instead of returning a `String`.
3. Handle the BOM (byte-order mark) that Excel inserts in CSV files: add an optional `write_with_bom` variant.
4. Implement `write_tsv` (tab-separated values) by parameterizing the delimiter.
5. Extend `needs_quoting` to also quote fields that start or end with whitespace.
