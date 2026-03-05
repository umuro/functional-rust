# OCaml vs Rust: Align Columns

## Side-by-Side Code

### OCaml
```ocaml
#load "str.cma"
open Str

let input = "Given$a$text\nfile$of$many"

let () =
  let lines = split (regexp_string "\n") input in
  let fields_l = List.map (split (regexp_string "$")) lines in
  let fields_l = List.map Array.of_list fields_l in
  let n =
    List.fold_left
      (fun n fields -> max n (Array.length fields))
      0 fields_l
  in
  let pads = Array.make n 0 in
  List.iter (
    Array.iteri
      (fun i word -> pads.(i) <- max pads.(i) (String.length word))
  ) fields_l;
  (* left-align *)
  List.iter (fun fields ->
    Array.iteri (fun i word ->
      let spaces = String.make (pads.(i) - String.length word) ' ' in
      Printf.printf "%s%s " word spaces
    ) fields;
    print_newline ()
  ) fields_l
```

### Rust (idiomatic)
```rust
#[derive(Clone, Copy)]
pub enum Alignment { Left, Right, Center }

pub fn align_columns(text: &str, delimiter: char, alignment: Alignment) -> String {
    let rows: Vec<Vec<&str>> = text.lines()
        .map(|line| line.split(delimiter).collect())
        .collect();
    let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
    let widths: Vec<usize> = (0..num_cols)
        .map(|col| rows.iter()
            .filter_map(|row| row.get(col))
            .map(|w| w.len())
            .max()
            .unwrap_or(0))
        .collect();
    rows.iter()
        .map(|row| row.iter().enumerate()
            .map(|(i, word)| {
                let w = widths.get(i).copied().unwrap_or(word.len());
                match alignment {
                    Alignment::Left   => format!("{word:<w$} "),
                    Alignment::Right  => format!("{word:>w$} "),
                    Alignment::Center => format!("{word:^w$} "),
                }
            })
            .collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}
```

### Rust (recursive width accumulation)
```rust
pub fn column_widths_recursive(rows: &[Vec<&str>]) -> Vec<usize> {
    fn update_widths(mut widths: Vec<usize>, row: &[&str]) -> Vec<usize> {
        for (i, word) in row.iter().enumerate() {
            if i >= widths.len() {
                widths.push(word.len());
            } else {
                widths[i] = widths[i].max(word.len());
            }
        }
        widths
    }
    rows.iter().fold(Vec::new(), |acc, row| update_widths(acc, row))
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Main function | `unit` (side-effectful) | `fn align_columns(&str, char, Alignment) -> String` |
| Row type | `string array` | `Vec<&str>` (borrowed slices) |
| Column widths | `int array` (mutable) | `Vec<usize>` (computed purely) |
| Alignment strategy | `word -> pad -> unit` callback | `Alignment` enum + `match` |
| Padding arithmetic | `String.make n ' '` + concat | `{word:<w$}` format spec |

## Key Insights

1. **Format strings eliminate padding math.** OCaml computes `String.make pad ' '`
   and concatenates; Rust's `{word:<w$}` / `{:>w$}` / `{:^w$}` build padding
   directly into the format spec — no arithmetic, no allocation for spaces.

2. **Enum replaces higher-order callback.** OCaml passes `fun word pad -> ...`
   to a generic `print` function. Rust's `Alignment` enum + `match` gives the
   same flexibility with exhaustiveness checking and zero runtime cost.

3. **Immutable width computation.** OCaml needs a mutable `pads` array updated
   with `Array.iteri`. Rust's `(0..n).map(|col| rows.filter_map(...).max())`
   computes all widths purely, with no mutable state.

4. **`str::lines()` vs `Str.split`.** OCaml requires `#load "str.cma"` and
   `Str.regexp_string` even to split on literal `\n`. Rust's `str::lines()` is
   built-in and handles all newline conventions (`\n`, `\r\n`) automatically.

5. **Ragged row safety.** OCaml's mutable accumulation handles ragged rows
   imperatively. Rust's `row.get(col)` returns `Option`, and `filter_map`
   naturally skips absent columns — no bounds check needed.

## When to Use Each Style

**Use idiomatic Rust (format specs + enum) when:** building a library API where
callers choose alignment; format specs are the fastest and most readable path
to padded output in Rust.

**Use recursive accumulation when:** demonstrating the functional fold pattern
explicitly, or when porting OCaml code where `List.fold_left` is the natural
structure and you want to mirror the original algorithm faithfully.
