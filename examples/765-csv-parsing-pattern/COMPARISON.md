# OCaml vs Rust: CSV Parsing Pattern

## CSV Parser

### Rust
```rust
pub fn parse_csv(input: &str) -> Result<Vec<Row>, CsvError> {
    let mut rows = Vec::new();
    for (line_num, line) in input.lines().enumerate() {
        let row = parse_row(line, line_num)?;
        rows.push(row);
    }
    Ok(rows)
}
```

### OCaml
```ocaml
let parse_csv input =
  input
  |> String.split_on_char '\n'
  |> List.mapi (fun i line -> parse_row line i)
  |> Result.all
```

## Quoted Fields

### Rust
```rust
if ch == '"' {
    if chars.peek() == Some(&'"') {
        chars.next();
        current.push('"');
    } else {
        in_quotes = false;
    }
}
```

### OCaml
```ocaml
| '"' :: '"' :: rest when in_quotes ->
    parse rest ~in_quotes (Buffer.add_char buf '"')
| '"' :: rest when in_quotes ->
    parse rest ~in_quotes:false buf
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Line iteration | `String.split` | `.lines()` |
| Character lookahead | Pattern match | `.peek()` |
| String building | `Buffer.t` | `String` |
| Error type | `result` | `Result<T, E>` |
