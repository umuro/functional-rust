# CSV Writer — Comparison

## Core Insight
CSV writing is the inverse of parsing: turn structured data back into escaped text. Both languages use the same algorithm (check if quoting is needed, double embedded quotes, wrap in outer quotes). OCaml's `Buffer` is the mutable accumulator equivalent to Rust's `String::with_capacity`.

## OCaml Approach
- `String.contains s ','` — check if quoting is needed
- `Buffer.create` + `Buffer.add_char` + `Buffer.contents` for efficient building
- `String.iter` to iterate characters
- `String.concat "," (List.map escape_field fields)` — functional pipeline for rows
- `String.concat "\n" (List.map write_row rows)` — functional pipeline for CSV

## Rust Approach
- `s.contains(',')` — idiomatic contains check
- `String::with_capacity` pre-allocates for efficiency
- `for c in s.chars()` iterates characters (Unicode-aware)
- `.map(|f| escape_field(f)).collect::<Vec<_>>().join(",")` — functional pipeline
- `rows.iter().map(write_row).collect::<Vec<_>>().join("\n")`

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Contains check | `String.contains s ','` | `s.contains(',')` |
| String building | `Buffer.create` + `add_char` | `String::with_capacity` + `push` |
| Char iteration | `String.iter` | `s.chars()` |
| Row join | `String.concat "," list` | `vec.join(",")` |
| Map + join pattern | `String.concat sep (List.map f list)` | `list.iter().map(f).collect::<Vec<_>>().join(sep)` |
| Empty field | `""` → no quoting | `""` → no quoting (same) |
