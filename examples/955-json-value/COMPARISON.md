# JSON Value Type — Comparison

## Core Insight
Both OCaml and Rust model JSON as a recursive algebraic data type. The mapping is nearly 1:1: OCaml variants become Rust enum variants, `list` becomes `Vec`, and `string` becomes `String`. The key difference is Rust requires explicit `PartialEq` derivation while OCaml's structural equality is built-in.

## OCaml Approach
- `type json = Null | Bool of bool | ...` — recursive variant type
- Pattern matching with `match` exhaustively handles all cases
- Structural equality (`=`) works automatically for all types
- `of (string * json) list` naturally models JSON objects as association lists
- Recursive types require no special annotation (OCaml handles it)

## Rust Approach
- `enum JsonValue { Null, Bool(bool), ... }` — direct enum translation
- `Box<T>` not needed here since `Vec` provides indirection for recursion
- `#[derive(Debug, Clone, PartialEq)]` adds traits OCaml has by default
- `matches!` macro for concise type-checking predicates
- Strings are `String` (owned), not `&str` (borrowed), for owned data

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type definition | `type json = Null \| Bool of bool \| ...` | `enum JsonValue { Null, Bool(bool), ... }` |
| Structural equality | Built-in `=` | `#[derive(PartialEq)]` |
| List type | `json list` | `Vec<JsonValue>` |
| String type | `string` | `String` |
| Object representation | `(string * json) list` | `Vec<(String, JsonValue)>` |
| Pattern matching | `match j with \| Null -> ...` | `match self { JsonValue::Null => ... }` |
| Recursive types | Implicit | Implicit (via Vec/Box) |
| Clone | `let j2 = j` (GC copies) | `#[derive(Clone)]` explicit |
