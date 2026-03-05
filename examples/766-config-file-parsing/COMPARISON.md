# OCaml vs Rust: Config File Parsing

## Config Structure

### Rust
```rust
pub struct Config {
    pub global: HashMap<String, String>,
    pub sections: HashMap<String, HashMap<String, String>>,
}
```

### OCaml
```ocaml
type config = {
  global: (string * string) list;
  sections: (string * (string * string) list) list;
}
```

## Parsing

### Rust
```rust
if line.starts_with('[') && line.ends_with(']') {
    let name = line[1..line.len()-1].trim();
    current_section = Some(name.to_string());
} else if let Some((key, value)) = line.split_once('=') {
    // Insert key-value
}
```

### OCaml
```ocaml
match String.get line 0, String.get line (String.length line - 1) with
| '[', ']' ->
    let name = String.sub line 1 (String.length line - 2) in
    parse_lines rest ~current:(Some name)
| _ ->
    match String.split_on_char '=' line with
    | [key; value] -> (* insert *) ...
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Map type | Association list | HashMap |
| String split | `split_on_char` | `.split_once()` |
| Section start | Pattern match | `starts_with` |
| Mutable state | Recursive accumulator | Mutable vars |
