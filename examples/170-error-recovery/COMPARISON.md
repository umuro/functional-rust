# Comparison: Example 170 — Error Recovery

## Position tracking

**OCaml:**
```ocaml
type position = { offset: int; line: int; col: int }

let advance_pos pos c =
  if c = '\n' then { offset = pos.offset + 1; line = pos.line + 1; col = 1 }
  else { offset = pos.offset + 1; line = pos.line; col = pos.col + 1 }
```

**Rust:**
```rust
#[derive(Debug, Clone)]
struct Position { offset: usize, line: usize, col: usize }

fn advance_pos(pos: &Position, c: char) -> Position {
    if c == '\n' {
        Position { offset: pos.offset + 1, line: pos.line + 1, col: 1 }
    } else {
        Position { offset: pos.offset + 1, line: pos.line, col: pos.col + 1 }
    }
}
```

## Error merging

**OCaml:**
```ocaml
if e1.pos.offset = e2.pos.offset then
  Error { pos = e1.pos; expected = e1.expected @ e2.expected; got = e1.got }
```

**Rust:**
```rust
if e1.pos.offset == e2.pos.offset {
    let mut expected = e1.expected;
    expected.extend(e2.expected);
    Err(ParseError { pos: e1.pos, expected, got: e1.got })
}
```
