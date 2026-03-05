📖 **[View on hightechmind.io →](https://hightechmind.io/rust/160-flatmap-parser)**

---

# 160: FlatMap Parser

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

`and_then(parser, f)` lets the next parser depend on the result of the previous one — context-sensitive parsing without giving up composability.

## The Problem This Solves

`map` transforms a parsed value, but always produces a fixed type using a pure function. What if what you need to parse *next* depends on what you just parsed?

Classic example: a length-prefixed string in a protocol like `"3:abc"`. First you parse the number `3`. Then — and this is the key — you parse *exactly three characters*. You can't express "parse N characters" as a fixed parser, because N changes based on the input. You need to parse one thing, look at its value, and *decide what to parse next*.

`and_then` (also known as `flatMap` or monadic bind) is exactly this. It says: "run this parser, pass its result to a function, and that function produces the *next parser* to run." The second parser isn't fixed — it's computed from the first result.

## The Intuition

You know `Option::and_then`:
```rust
let s = Some("42");
let n = s.and_then(|x| x.parse::<u32>().ok());
// n = Some(42) — the second step depends on the first
```
Parser `and_then` is the same idea. The closure receives the parsed value and returns a new parser — not a new value, a new *parser*. That new parser then runs on the remaining input.

This is the difference between `map` and `and_then`:
- `map(p, f)`: run p, apply `f` to the result → new value
- `and_then(p, f)`: run p, apply `f` to the result → **new parser**, then run that parser

In functional programming terms, `and_then` is monadic bind (`>>=`). If `map` makes parsers *functors*, `and_then` makes them *monads* — which means they can express any context-sensitive grammar.

## How It Works in Rust

**`and_then` — monadic bind:**
```rust
fn and_then<'a, A: 'a, B: 'a, F>(parser: Parser<'a, A>, f: F) -> Parser<'a, B>
where F: Fn(A) -> Parser<'a, B> + 'a  // f takes A and RETURNS a Parser, not just B
{
    Box::new(move |input: &'a str| {
        let (value, rest) = parser(input)?;  // run first parser
        (f(value))(rest)                     // f produces a new parser; run it on `rest`
    })
}
```
The type signature is the key: `F: Fn(A) -> Parser<'a, B>`. The function `f` returns a `Parser`, not just a `B`. Then `(f(value))(rest)` calls that returned parser on the remaining input.

**Length-prefixed strings — a real use case:**
```rust
fn length_prefixed<'a>() -> Parser<'a, &'a str> {
    and_then(parse_nat(), |n| {
        // `n` is the length we just parsed
        Box::new(move |input: &'a str| {
            if input.starts_with(':') {
                let rest = &input[1..];  // skip the ':'
                if rest.len() >= n {
                    Ok((&rest[..n], &rest[n..]))  // take exactly n bytes
                } else {
                    Err("Not enough characters".to_string())
                }
            } else {
                Err("Expected ':'".to_string())
            }
        })
    })
}
// parse_nat()  →  and_then  →  parse n chars after ':'
// "3:abcrest"  →  n=3       →  Ok(("abc", "rest"))
```
Notice: the inner closure captures `n` from the outer scope. The parser it creates *knows* how many characters to consume because it was created with that knowledge baked in.

**Conditional parsing — choose the parser based on a tag:**
```rust
fn conditional_parser<'a>() -> Parser<'a, String> {
    and_then(
        satisfy(|c| c == 'i' || c == 's', "type tag"),
        |tag_char| {
            if tag_char == 'i' {
                // 'i' means: parse digits
                map(many1(satisfy(|c| c.is_ascii_digit(), "digit")),
                    |chars| chars.into_iter().collect())
            } else {
                // 's' means: parse letters
                map(many1(satisfy(|c| c.is_ascii_lowercase(), "letter")),
                    |chars| chars.into_iter().collect())
            }
        },
    )
}
// "i42rest" → tag='i' → parse digits → Ok(("42", "rest"))
// "sabcREST" → tag='s' → parse letters → Ok(("abc", "REST"))
```

## What This Unlocks

- **Context-sensitive grammars** — anything where "what comes next" depends on "what came before": length-prefixed data, tagged unions, variable-length records.
- **Full grammar power** — `map` + `and_then` together give you the full expressiveness of monadic parsers. Any grammar that can be expressed programmatically can be expressed with these two.
- **Protocol parsing** — binary protocol fields with length prefixes, type-tagged payloads, and similar patterns are exactly what `and_then` was made for.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bind operator | `p >>= fun x -> ...` (infix) | `and_then(p, \|x\| ...)` (function call) |
| Infix bind | `let (>>=) = and_then` is idiomatic | Not idiomatic in Rust; trait method preferred |
| Closure returns parser | Natural — functions return functions | `F: Fn(A) -> Parser<'a, B> + 'a` explicit |
| vs. `map` | `>>=` is strictly more powerful | `and_then` is strictly more powerful than `map` |
| Context sensitivity | Easy — functions close over anything | Same, with explicit lifetime bounds |
