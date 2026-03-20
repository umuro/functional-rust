📖 **[View on hightechmind.io →](https://hightechmind.io/rust/163-whitespace-parser)**

---

# Whitespace Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Most text formats are whitespace-insensitive: `{"key": "value"}` and `{ "key" : "value" }` are equivalent JSON. Parsers must skip whitespace between tokens without interfering with token recognition. `ws0` (zero or more whitespace characters) and `ws1` (one or more) are standard utilities. Wrapping content parsers with `ws_wrap` allows callers to ignore whitespace concerns entirely, keeping individual token parsers clean and focused.

## Learning Outcomes

- Implement `ws0`, `ws1`, and `ws_wrap` as standard whitespace-handling utilities
- Understand why whitespace parsers always succeed (zero whitespace is valid)
- Learn the "wrap with whitespace" pattern for building whitespace-insensitive parsers
- See how comment-skipping extends whitespace handling for real languages

## Rust Application

`ws0()` is `many0(satisfy(|c| c.is_whitespace(), "whitespace"))` followed by `map(|_| ())` to discard the results — whitespace is consumed but not kept. `ws1()` uses `many1` to require at least one whitespace character. `ws_wrap(p)` is `preceded(ws0(), terminated(p, ws0()))` — skip leading whitespace, run `p`, skip trailing whitespace. This pattern wraps individual token parsers rather than inserting whitespace skipping everywhere.

## OCaml Approach

Angstrom provides `skip_while : (char -> bool) -> unit t` for efficient whitespace skipping without character-by-character overhead:
```ocaml
let ws = skip_while (fun c -> c = ' ' || c = '\t' || c = '\n' || c = '\r')
let ws_wrap p = ws *> p <* ws
```
OCaml's `skip_while` scans the buffer without constructing `char` values, making whitespace skipping more efficient than `many0(satisfy(...))`.

## Key Differences

1. **Efficiency**: OCaml's `skip_while` skips bytes without constructing values; Rust's `many0(satisfy(...))` creates `Vec<char>` and discards it.
2. **Optimization**: A production Rust parser would use `input.trim_start()` directly or scan with `str::find(|c: char| !c.is_whitespace())` — bypassing the combinator overhead.
3. **Line counting**: Neither basic `ws0` tracks line numbers; adding line/column tracking requires threading a position state through the parser.
4. **Comment handling**: Both can extend `ws0` to also skip comments; the typical approach is `many0(choice([whitespace, line_comment, block_comment]))`.

## Exercises

1. Extend `ws0` to also skip line comments: `// ...until end of line`.
2. Implement `ws_between(open: Parser<A>, sep: Parser<B>, close: Parser<C>) -> Parser<Vec<B>>` that handles whitespace around separators.
3. Write a `lexeme(p: Parser<T>) -> Parser<T>` combinator that skips whitespace after `p` (a common pattern in language parsers).
