📖 **[View on hightechmind.io →](https://hightechmind.io/rust/152-char-parser)**

---

# 152: Character Parsers

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

The atom of parsing: consume exactly one character from the input — specific, any, or from a set.

## The Problem This Solves

Every parser, no matter how complex, eventually bottoms out at "read one character." JSON parsers read `{` and `}`. Number parsers read digits. Language parsers read letters. Before you can build anything bigger, you need these atomic parsers — the smallest possible parsers that do meaningful work.

Single-character parsers also expose a real Rust concern: Unicode. A Rust `&str` is UTF-8, meaning a single "character" like `é` takes 2 bytes, `€` takes 3, and emoji take 4. If you naively slice with `&input[1..]`, you'll panic on any non-ASCII character. The right way is `&input[c.len_utf8()..]` — advance by the actual byte count of the character you just consumed.

## The Intuition

Think of a parser as a cursor reading a document. `char_parser('h')` checks: "Is the next character `h`? If yes, advance past it and return `h`. If no, stop and report what you found."

`any_char` is the same, but it never complains — whatever is there, take it. `none_of` and `one_of` work like character class filters: `none_of(vec!['x', 'y', 'z'])` accepts any character *except* those three. This mirrors regex's `[^xyz]` and `[abc]` syntax, but as composable functions instead of pattern strings.

## How It Works in Rust

**Parse a specific character:**
```rust
fn char_parser<'a>(expected: char) -> Parser<'a, char> {
    Box::new(move |input: &'a str| {
        match input.chars().next() {
            // c.len_utf8() correctly handles multi-byte Unicode
            Some(c) if c == expected => Ok((c, &input[c.len_utf8()..])),
            Some(c) => Err(format!("Expected '{}', got '{}'", expected, c)),
            None    => Err(format!("Expected '{}', got EOF", expected)),
        }
    })
}
```
The `move` captures `expected` by value. `input.chars().next()` returns the first Unicode scalar value without copying. The `if c == expected` guard on the pattern match is Rust's "match guard" — pattern matching with an extra condition.

**Parse any single character:**
```rust
fn any_char<'a>() -> Parser<'a, char> {
    Box::new(|input: &'a str| {
        match input.chars().next() {
            Some(c) => Ok((c, &input[c.len_utf8()..])),
            None    => Err("Expected any character, got EOF".to_string()),
        }
    })
}
```
No `move` needed here — there's nothing to capture.

**Parse a character NOT in a set:**
```rust
fn none_of<'a>(chars: Vec<char>) -> Parser<'a, char> {
    Box::new(move |input: &'a str| {
        match input.chars().next() {
            Some(c) if !chars.contains(&c) => Ok((c, &input[c.len_utf8()..])),
            Some(c) => Err(format!("Unexpected character '{}'", c)),
            None    => Err("Expected a character, got EOF".to_string()),
        }
    })
}
```
`chars` is moved into the closure (`move`). The `!chars.contains(&c)` check is `O(n)` — for large sets, consider `HashSet<char>`.

**Usage:**
```rust
let p = char_parser('h');
println!("{:?}", p("hello")); // Ok(('h', "ello"))
println!("{:?}", p("world")); // Err("Expected 'h', got 'w'")

// Unicode works correctly
let p = char_parser('é');
println!("{:?}", p("école")); // Ok(('é', "cole")) — advanced 2 bytes
```

## What This Unlocks

- **Foundation for all other parsers** — every combinator in examples 153–162 is built on top of functions exactly like these.
- **Safe Unicode handling** — `c.len_utf8()` means your parser works on any valid UTF-8 input without panics.
- **Character class parsing** — `one_of` and `none_of` let you express "match any vowel" or "match anything except a quote" without regex.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| First character | `input.[0]` (byte index) | `input.chars().next()` (Unicode scalar) |
| Advance past char | `String.sub input 1 (len-1)` | `&input[c.len_utf8()..]` |
| Set membership | `List.mem ch chars` | `chars.contains(&c)` |
| Unicode safety | Manual (byte-level strings) | Built-in (UTF-8 guaranteed by type system) |
| Closure capture | Automatic | `move` keyword required |
