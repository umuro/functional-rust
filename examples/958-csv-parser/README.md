**Difficulty:** ⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐  

[csv-parser on hightechmind.io](https://hightechmind.io/posts/functional-rust/csv-parser)

---

## Problem Statement

Implement a CSV line parser that handles quoted fields (fields containing commas or newlines wrapped in double quotes) and escaped quotes (`""` inside a quoted field represents a literal `"`). Model the parser as a finite state machine with three states: `Normal`, `InQuote`, and `AfterQuote`. Implement both a simple split-only version and the full state-machine version.

## Learning Outcomes

- Implement a simple CSV split using `line.split(',').collect()`
- Model parser state as an enum: `Normal`, `InQuote`, `AfterQuote`
- Drive the state machine character by character with `match (&state, c)`
- Handle `""` escape: transition from `InQuote` to `AfterQuote` on `"`, then back to `InQuote` on another `"` (escaped quote) or to `Normal` on `,` (end of field)
- Recognize when `split(',')` is insufficient and a real state machine is required

## Rust Application

```rust
#[derive(Debug, PartialEq)]
enum State { Normal, InQuote, AfterQuote }

pub fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut state = State::Normal;

    for c in line.chars() {
        match (&state, c) {
            (State::Normal, '"')   => { state = State::InQuote; }
            (State::Normal, ',')   => { fields.push(current.clone()); current.clear(); }
            (State::Normal, c)     => { current.push(c); }
            (State::InQuote, '"')  => { state = State::AfterQuote; }
            (State::InQuote, c)    => { current.push(c); }
            (State::AfterQuote, '"') => { current.push('"'); state = State::InQuote; }
            (State::AfterQuote, ',') => { fields.push(current.clone()); current.clear(); state = State::Normal; }
            (State::AfterQuote, c)   => { current.push(c); state = State::Normal; }
        }
    }
    fields.push(current);
    fields
}
```

The match on `(&state, c)` — a reference to the state enum paired with the character — makes transitions explicit and exhaustive. Each arm handles one (state, input) combination.

`AfterQuote` is the key state: it disambiguates between `""` (escaped quote → stay in quote, emit `"`) and `",` (end of quoted field → go to Normal, push field). Without this third state, a bare two-state machine conflates these two cases.

The final `fields.push(current)` after the loop handles the last field, which has no trailing comma delimiter.

## OCaml Approach

```ocaml
type state = Normal | InQuote | AfterQuote

let parse_csv_line line =
  let fields = ref [] in
  let current = Buffer.create 64 in
  let state = ref Normal in

  String.iter (fun c ->
    match !state, c with
    | Normal, '"'      -> state := InQuote
    | Normal, ','      -> fields := Buffer.contents current :: !fields;
                          Buffer.clear current
    | Normal, c        -> Buffer.add_char current c
    | InQuote, '"'     -> state := AfterQuote
    | InQuote, c       -> Buffer.add_char current c
    | AfterQuote, '"'  -> Buffer.add_char current '"'; state := InQuote
    | AfterQuote, ','  -> fields := Buffer.contents current :: !fields;
                          Buffer.clear current; state := Normal
    | AfterQuote, c    -> Buffer.add_char current c; state := Normal
  ) line;
  fields := Buffer.contents current :: !fields;
  List.rev !fields
```

OCaml uses `Buffer` for efficient mutable string building (Rust equivalent: `String::with_capacity`). The `ref` pattern for mutable state is OCaml's imperative style. The `match !state, c` has identical structure to Rust's `match (&state, c)`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| String builder | `String` with `push` | `Buffer` with `add_char` |
| Mutable state | `let mut state = State::Normal` | `let state = ref Normal` |
| Pattern match | `match (&state, c)` | `match !state, c` |
| Character iteration | `line.chars()` | `String.iter (fun c -> ...)` |
| Final field | `fields.push(current)` after loop | `fields := contents :: !fields` + `List.rev` |

CSV parsing is a classic example where simple `split(',')` fails for real-world data. The three-state machine is the minimal FSM that correctly handles RFC 4180 quote escaping.

## Exercises

1. Extend the parser to handle multi-line records (quoted fields containing newlines).
2. Implement a full CSV file parser: split on newlines, parse each line, return `Vec<Vec<String>>`.
3. Add trimming of leading/trailing whitespace from unquoted fields.
4. Implement a streaming parser using `impl Iterator<Item=Vec<String>>` that processes one line at a time.
5. Write a property test: encode a `Vec<Vec<String>>` with the CSV writer (959), then parse with this parser — result should round-trip exactly.
