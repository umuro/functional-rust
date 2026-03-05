# 151: Introduction to Parser Combinators

**Difficulty:** ⭐⭐⭐  **Level:** Foundations

A parser is just a function: take a string, return `(parsed_value, remaining_string)` or an error — and from that one idea, you can build a parser for any language.

## The Problem This Solves

You need to read structured text: a config file, a JSON document, a math expression, some custom DSL your team invented. You reach for regex — and it works, until the input gets nested. Regex can't count matching brackets. It can't parse `(1 + (2 * 3))` and know where the inner expression ends. The moment your format has any nesting at all, regex hits a wall.

You could use a parser generator (ANTLR, Yacc), but those require a separate grammar file, a build step, and learning a new notation just to get a `Vec<Token>` back. What if you could write your parser in pure Rust, with no external tools, no codegen, no macros — just functions composed together?

Parser combinators give you exactly that. The core insight is almost comically simple: **a parser is a function**. It takes a string slice and returns either a success (value + what's left to parse) or a failure. Once you have that definition, every other concept — repetition, sequencing, alternation — is just a function that takes parsers and returns a new parser. No magic, no framework. Just functions all the way down.

## The Intuition

You already know this pattern. Imagine you're reading a sentence word by word. You look at the first word, recognize it, and remember where you stopped. Then you pass "where you stopped" to the next step, which does the same. If any step fails, you stop and report what went wrong.

That's exactly what a parser does. The "where you stopped" is `&str` — a slice into the original input. This is zero-copy: we never duplicate the string, we just move a pointer forward.

The type alias captures everything:
```rust
type ParseResult<'a, T> = Result<(T, &'a str), String>;
```
- `T` is whatever you parsed (a `char`, a `u64`, a `Vec<Token>`, anything)
- `&'a str` is the remaining input — what the next parser will see
- `String` is the error message if parsing failed
- The lifetime `'a` ties the remaining slice to the original input (it's a view into the same memory)

A `Parser` is just a boxed function with that signature:
```rust
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;
```
`Box<dyn Fn(...)>` is how Rust stores a function when you don't know its exact type at compile time. Closures have unique anonymous types — boxing erases that type so different parsers can be stored and passed around uniformly.

## How It Works in Rust

**The simplest parser — a plain function:**
```rust
fn parse_char_a(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some('a') => Ok(('a', &input[1..])),  // consume the 'a', return rest
        Some(c)   => Err(format!("Expected 'a', got '{}'", c)),
        None      => Err("Expected 'a', got end of input".to_string()),
    }
}
```
`input.chars().next()` peeks at the first Unicode character. `&input[1..]` moves forward by one byte (fine for ASCII; for multi-byte chars, use `c.len_utf8()`).

**A factory function that builds parsers:**
```rust
fn char_p<'a>(expected: char) -> Parser<'a, char> {
    Box::new(move |input: &'a str| {  // 'move' captures `expected` by value
        match input.chars().next() {
            Some(c) if c == expected => Ok((c, &input[c.len_utf8()..])),
            Some(c) => Err(format!("Expected '{}', got '{}'", expected, c)),
            None    => Err(format!("Expected '{}', got end of input", expected)),
        }
    })
}
```
`move` is required: the closure must own `expected` because it will outlive the function call that created it.

**Utility parsers for any type:**
```rust
fn pure<'a, T: Clone + 'a>(value: T) -> Parser<'a, T> {
    Box::new(move |input| Ok((value.clone(), input)))  // always succeeds, consumes nothing
}

fn fail<'a, T: 'a>(msg: &str) -> Parser<'a, T> {
    let msg = msg.to_string();
    Box::new(move |_| Err(msg.clone()))  // always fails
}
```
These seem trivial, but `pure` becomes essential when you need a parser that "returns a default value" and `fail` is useful for testing and error recovery.

**Running a parser:**
```rust
let p = char_p('h');
match p("hello") {
    Ok((ch, rest)) => println!("Parsed '{}', remaining: \"{}\"", ch, rest),
    Err(e)         => println!("Error: {}", e),
}
// → Parsed 'h', remaining: "ello"
```

## What This Unlocks

- **No external dependencies** — your parser is pure Rust, no crates needed, no build magic.
- **Full composability** — every example from 152 onward builds on this one type definition; combining parsers is just combining functions.
- **Easy error handling** — `Result` gives you structured failure for free; combinators can enrich errors as they propagate.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Parser type | `string -> ('a * string, string) result` | `Box<dyn Fn(&'a str) -> Result<(T, &'a str), String> + 'a>` |
| Why boxed? | Not needed — GC handles function values | Closures have unique types; `Box<dyn Fn>` erases them |
| Lifetime `'a` | Handled by GC | Explicit: ties output slice to input lifetime |
| Closure capture | Automatic | `move` keyword required for owned captures |
| Library | `Angstrom`, `opal` | `nom`, `combine`, `winnow` (or roll your own) |
