# 170: Error Recovery

**Difficulty:** 3  **Level:** Advanced

Build a parser that collects all errors — not just the first — like a real compiler showing 10 problems at once.

## The Problem This Solves

Every parser fails on bad input. The question is *how* it fails. A parser that stops at the first error forces you to fix one problem, rerun, see the second problem, fix it, rerun — painfully slow feedback. Real compilers (Rust, GHC, Clang) show all the errors they found in one pass.

To collect multiple errors, the parser needs to *recover* after each failure: skip forward to a known-good synchronization point (like a semicolon or newline), report the error, and keep going. This requires threading position information (line and column numbers) through parsing so errors can be located precisely.

This example adds a `Position` type, richer `ParseError` structs with expected/found information, error merging in alternatives, and sync-point recovery that resumes parsing after failures.

## The Intuition

Normal parsers return `Result<(T, &str), String>`. Error-recovering parsers return `Result<(T, &str, Vec<Error>), String>` — they carry *accumulated* errors alongside the successful parse. When a sub-parser fails, instead of propagating the error up, you record it and skip to the next `;` or `\n` to resynchronize.

```
input: "let x = ; let y = 42;"
parse "let x = ..." → fails at ";"
  → record error: "expected expression at line 1, col 9"
  → skip to ";" → resume
parse "let y = 42;" → ok
result: errors=["expected expression at 1:9"], y=42
```

## How It Works in Rust

```rust
#[derive(Clone, Debug)]
struct Position {
    offset: usize,
    line: usize,
    col: usize,
}

#[derive(Debug)]
struct ParseError {
    position: Position,
    expected: Vec<String>,
    found: String,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "line {}, col {}: expected {}, found '{}'",
            self.position.line, self.position.col,
            self.expected.join(" or "), self.found)
    }
}

// Advance position tracking line/column numbers
fn advance_pos(pos: &Position, consumed: &str) -> Position {
    let mut p = pos.clone();
    for ch in consumed.chars() {
        p.offset += ch.len_utf8();
        if ch == '\n' { p.line += 1; p.col = 1; }
        else { p.col += 1; }
    }
    p
}

// Recovery: skip to a synchronization character, record the error
fn recover<'a>(
    input: &'a str,
    pos: &Position,
    error: ParseError,
    sync_char: char,
    errors: &mut Vec<ParseError>,
) -> (&'a str, Position) {
    errors.push(error);
    // Skip forward to sync point
    match input.find(sync_char) {
        Some(i) => {
            let skipped = &input[..i + sync_char.len_utf8()];
            (&input[i + sync_char.len_utf8()..], advance_pos(pos, skipped))
        }
        None => ("", advance_pos(pos, input)),  // sync not found → consume all
    }
}

// Alt that merges errors from both branches at the same position
fn alt_pos<'a, T>(
    first: impl Fn(&'a str, &Position) -> Result<(T, &'a str, Position), ParseError>,
    second: impl Fn(&'a str, &Position) -> Result<(T, &'a str, Position), ParseError>,
) -> impl Fn(&'a str, &Position) -> Result<(T, &'a str, Position), ParseError> {
    move |input, pos| {
        match first(input, pos) {
            Ok(r) => Ok(r),
            Err(e1) => match second(input, pos) {
                Ok(r) => Ok(r),
                Err(mut e2) => {
                    // Merge expected lists if errors are at the same position
                    if e1.position.offset == e2.position.offset {
                        e2.expected.extend(e1.expected);
                    }
                    Err(e2)
                }
            }
        }
    }
}
```

## What This Unlocks

- **IDE-quality error messages** — report line and column, what was expected, what was found.
- **Batch error reporting** — parse an entire file and show all problems at once.
- **Resilient tooling** — syntax highlighters and formatters that keep working despite errors.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Position type | Record `{ offset; line; col }` | `struct Position` with `#[derive(Clone)]` |
| Error display | `Printf.sprintf "line %d ..."` | `impl Display for ParseError` |
| Error merging | List concatenation `e1 @ e2` | `Vec::extend(e1.expected)` |
| Lazy alternative | Automatic (OCaml is strict but closures defer) | `impl FnOnce()` parameter for lazy second branch |
