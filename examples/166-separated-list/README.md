📖 **[View on hightechmind.io →](https://hightechmind.io/rust/166-separated-list)**

---

# 166: Separated List

**Difficulty:** 3  **Level:** Advanced

Parse `a, b, c` — the comma-separated pattern used everywhere: function arguments, array literals, CSV rows, import lists.

## The Problem This Solves

Separated lists are everywhere. Function calls: `foo(x, y, z)`. Array literals: `[1, 2, 3]`. CSV rows: `Alice,30,Engineer`. Import lists: `use std::{io, fs, collections}`. Every language, every data format — they all have this pattern.

The tricky part is the separator. You can't just parse `item (sep item)*` naively — what about trailing separators like `[1, 2, 3,]`? And what about the empty list `[]`? You need to handle three cases: empty list, single item, and multiple items with separators between them.

The harder problem is *backtracking on the separator*. If you parse `1,` and then fail to find another item, you've consumed the trailing comma. You need to save your position before trying the separator, and only advance past it if the next item also succeeds.

## The Intuition

Parse the first item. Then loop: save position, try the separator, try the next item. If both succeed, keep going. If either fails, *restore position to before the separator* and stop.

```
input: "1, 2, 3"
parse first: 1, remaining: ", 2, 3"
  save pos → try sep "," → ok → try item → 2 → keep
  save pos → try sep "," → ok → try item → 3 → keep
  save pos → try sep "," → fail → restore → stop
result: [1, 2, 3]
```

## How It Works in Rust

```rust
fn separated_list0<'a, T, S>(
    separator: impl Fn(&'a str) -> ParseResult<S>,
    item: impl Fn(&'a str) -> ParseResult<T>,
) -> impl Fn(&'a str) -> ParseResult<Vec<T>> {
    move |input| {
        let mut results = Vec::new();
        let mut remaining = input;

        // Try first item — if this fails, return empty list (list0 allows empty)
        match item(remaining) {
            Err(_) => return Ok((results, remaining)),
            Ok((val, rest)) => {
                results.push(val);
                remaining = rest;
            }
        }

        // Now try separator + item pairs
        loop {
            let before_sep = remaining;  // save position BEFORE separator

            match separator(before_sep) {
                Err(_) => break,  // no separator → done
                Ok((_, after_sep)) => {
                    match item(after_sep) {
                        Err(_) => break,  // separator but no item → trailing sep, restore
                        Ok((val, rest)) => {
                            results.push(val);
                            remaining = rest;  // advance past both sep and item
                        }
                    }
                }
            }
            // Note: if sep succeeded but item failed, remaining is still before_sep
            // because we didn't update it — that's the backtracking
        }

        Ok((results, remaining))
    }
}

// list1: requires at least one item
fn separated_list1<'a, T, S>(
    separator: impl Fn(&'a str) -> ParseResult<S>,
    item: impl Fn(&'a str) -> ParseResult<T>,
) -> impl Fn(&'a str) -> ParseResult<Vec<T>> {
    move |input| {
        let (results, rest) = separated_list0(&separator, &item)(input)?;
        if results.is_empty() {
            Err("expected at least one item".to_string())
        } else {
            Ok((results, rest))
        }
    }
}
```

## What This Unlocks

- **Function argument lists** — `parse_args = separated_list0(comma, parse_expr)`.
- **Array/tuple literals** — wrap with `[` and `]` using `delimited`.
- **CSV rows and data formats** — any flat list with a separator between items.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Loop style | Tail-recursive helper function | `loop` + `break` |
| Backtracking | Return previous `remaining` value | Don't update `remaining` if item fails |
| Trailing sep | Separate `separated_list_trailing` variant | Same approach — separate function |
| Empty list | `list0` returns `[]` | Returns `Vec::new()` |
