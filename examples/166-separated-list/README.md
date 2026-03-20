📖 **[View on hightechmind.io →](https://hightechmind.io/rust/166-separated-list)**

---

# Separated List

## Problem Statement

Comma-separated values, semicolon-separated statements, pipe-separated fields — separated lists appear everywhere in text formats. `separated_list0` and `separated_list1` parse sequences of items with a separator between each pair, correctly handling the last item (no trailing separator in strict formats) and empty lists. Building this from primitives requires careful handling of the separator-before-next-item pattern to avoid consuming the separator when the list has ended.

## Learning Outcomes

- Implement `separated_list0` and `separated_list1` combinators
- Understand the interleave pattern: item, (sep item)*, where `(sep item)*` is the tail
- Learn how separated list parsing relates to `many0` and `pair`
- See how separated lists form the basis for CSV, function argument lists, and array literals

## Rust Application

`separated_list1(sep, item)` parses one `item`, then `many0(preceded(sep, item))` — a separator followed by an item, repeated zero or more times. The separator is consumed as part of the tail, not as a lookahead. This correctly handles `"1"` (one item), `"1,2,3"` (three items), and rejects `""` (zero items for `_list1`). `separated_list0` wraps `opt(separated_list1)` to handle the empty case.

## OCaml Approach

Angstrom provides `sep_by : 'a t -> 'b t -> 'b list t` and `sep_by1`:
```ocaml
let csv_row = sep_by (char ',') field_parser
```
OCaml's functional style makes the separator combinator more concise. The implementation uses `many (sep *> item)` after the first item — structurally identical to Rust's approach but expressed with `>>=` and infix operators.

## Key Differences

1. **Separator consumption**: Both parsers consume the separator as part of the item following it (not before it) — this is the standard approach ensuring correct error location.
2. **Trailing separator**: Neither `sep_by` variant allows trailing separators; a separate `opt(sep)` must be added for formats like Rust's trailing commas in arrays.
3. **Return type**: Rust returns `Vec<T>`; OCaml returns `'a list` — both are ordered sequences.
4. **Whitespace around separators**: Production parsers wrap separators with `ws_wrap` or use `lexeme`; these examples show the pure combinator without whitespace.

## Exercises

1. Parse a function argument list: `"(a, b, c)"` → `vec!["a", "b", "c"]` using `delimited` + `separated_list0`.
2. Add support for trailing separators: `"1, 2, 3,"` → `vec![1, 2, 3]`.
3. Parse nested lists: `"[[1,2],[3,4,5]]"` → `vec![vec![1,2], vec![3,4,5]]` using recursive separated list parsers.
