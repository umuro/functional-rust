# Comparison: Example 151 — Parser Combinator Introduction

## Core Parser Type

**OCaml:**
```ocaml
type 'a parse_result = Ok of 'a * string | Error of string
type 'a parser = string -> 'a parse_result
```

**Rust:**
```rust
type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;
```

## Character Parser

**OCaml:**
```ocaml
let char (c : char) : char parser = fun input ->
  if String.length input > 0 && input.[0] = c then
    Ok (c, String.sub input 1 (String.length input - 1))
  else
    Error (Printf.sprintf "Expected '%c'" c)
```

**Rust:**
```rust
fn char_p<'a>(expected: char) -> Parser<'a, char> {
    Box::new(move |input: &'a str| {
        match input.chars().next() {
            Some(c) if c == expected => Ok((c, &input[c.len_utf8()..])),
            Some(c) => Err(format!("Expected '{}', got '{}'", expected, c)),
            None => Err(format!("Expected '{}', got end of input", expected)),
        }
    })
}
```

## Pure / Return

**OCaml:**
```ocaml
let return (x : 'a) : 'a parser = fun input -> Ok (x, input)
```

**Rust:**
```rust
fn pure<'a, T: Clone + 'a>(value: T) -> Parser<'a, T> {
    Box::new(move |input| Ok((value.clone(), input)))
}
```

## Running a Parser

**OCaml:**
```ocaml
let run (p : 'a parser) (input : string) = p input
```

**Rust:**
```rust
fn run_parser<'a, T>(parser: &Parser<'a, T>, input: &'a str) -> ParseResult<'a, T> {
    parser(input)
}
```
