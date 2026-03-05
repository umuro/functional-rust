# 765: CSV Parsing Without External Crates

**Difficulty:** 3  **Level:** Intermediate

A complete RFC 4180-compliant CSV parser using an explicit state machine — handles quoted fields, embedded commas, and escaped quotes.

## The Problem This Solves

CSV looks trivial — split on commas, right? Then you encounter `"Smith, John"` and realize commas inside quotes are valid. Then `"She said ""hello"""` and realize quotes inside quoted fields are represented as doubled quotes. Then Windows line endings (`\r\n`). Then empty fields. The naive `split(',')` approach breaks on all of these.

In production, CSV appears everywhere: exports from databases, spreadsheets, billing systems, analytics platforms. Getting the parsing wrong means silent data corruption — you read `Smith` from a field that should be `Smith, John`, and the downstream system gets garbage. RFC 4180 defines the standard, and a correct parser follows it precisely.

Writing this by hand also teaches you state machines — a fundamental tool in systems programming. The CSV state machine has exactly three states (`Normal`, `Quoted`, `QuoteInQuoted`), and the transition logic fits in a single `match`. Understanding this pattern makes every other format parser easier to reason about.

## The Intuition

Think of Python's `csv.reader` — it handles all these edge cases internally. In JavaScript, `Papa Parse` does the same. In Rust, the `csv` crate is excellent for production. But writing it by hand shows you *exactly* what those libraries are doing.

The state machine approach is cleaner than hand-tracking indices. You process one character at a time and transition between states:
- `Normal`: outside quotes — commas separate fields, `"` starts a quoted field
- `Quoted`: inside quotes — everything is literal, `"` might end the field or be an escape
- `QuoteInQuoted`: just saw `"` inside quotes — if the next char is `"`, it's an escaped quote; if it's `,`, the field ended; if it's end-of-input, the field ended

## How It Works in Rust

```rust
#[derive(Debug, PartialEq)]
enum State { Normal, Quoted, QuoteInQuoted }

pub fn parse_fields(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut buf = String::new();
    let mut state = State::Normal;

    for ch in line.chars() {
        match (&state, ch) {
            // Normal: comma ends field, quote starts quoted field
            (State::Normal, ',') => { fields.push(buf.clone()); buf.clear(); }
            (State::Normal, '"') => { state = State::Quoted; }
            (State::Normal, c)   => { buf.push(c); }

            // Quoted: quote might end field or be escaped
            (State::Quoted, '"') => { state = State::QuoteInQuoted; }
            (State::Quoted, c)   => { buf.push(c); }

            // Just saw closing quote — is it escaped or end of field?
            (State::QuoteInQuoted, '"') => {
                buf.push('"');              // "" = escaped quote
                state = State::Quoted;
            }
            (State::QuoteInQuoted, ',') => {
                fields.push(buf.clone());  // field ended
                buf.clear();
                state = State::Normal;
            }
            (State::QuoteInQuoted, c)   => {
                buf.push(c);               // trailing content after closing quote
                state = State::Normal;
            }
        }
    }
    fields.push(buf);  // last field (no trailing comma)
    fields
}

// Parse typed records from rows
impl Person {
    pub fn from_row(row: &[String]) -> Option<Self> {
        if row.len() < 3 { return None; }
        let age = row[1].trim().parse().ok()?;
        Some(Person { name: row[0].clone(), age, city: row[2].clone() })
    }
}

// Parse a whole CSV document
pub fn parse_csv(text: &str) -> Vec<Vec<String>> {
    text.lines()
        .map(|l| l.trim_end_matches('\r'))  // handle Windows \r\n
        .filter(|l| !l.is_empty())
        .map(parse_fields)
        .collect()
}
```

Result for `"Bob, Jr.",25,"New York"`:
- Field 1: `Bob, Jr.` (comma inside quotes — not a separator)
- Field 2: `25`
- Field 3: `New York`

Result for `"a""b",c`:
- Field 1: `a"b` (doubled quote → single quote)
- Field 2: `c`

Key points:
- Pattern matching on `(&state, ch)` — the state and character together determine the transition
- `buf.clone()` + `buf.clear()` accumulates each field into a buffer, pushes on comma
- `trim_end_matches('\r')` handles Windows line endings without pulling in platform-specific code
- The final `fields.push(buf)` handles the last field — CSV rows don't end with a comma

## What This Unlocks

- **Data import pipelines**: correctly handle any CSV export from Excel, PostgreSQL `COPY`, or billing systems — including fields with commas and embedded quotes
- **State machine fluency**: the same three-state pattern applies to tokenizers, protocol parsers, and format readers — master it once, apply it everywhere
- **Custom column mapping**: `Person::from_row` shows how to map string columns to typed fields with proper error handling via `Option`

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| State machine | Variant type + recursive function | `enum State` + `match (&state, ch)` |
| Field accumulation | `Buffer.t` | `String` with `push` / `clear` |
| State transition | Match on `(state, char)` | Same — match on `(&state, ch)` |
| Row parsing | `String.split_on_char ','` (naive) | State machine — handles quotes and escapes |
| Production library | `csv-ex`, `octavius` | `csv` crate |
| Windows line endings | Manual stripping | `.trim_end_matches('\r')` |
