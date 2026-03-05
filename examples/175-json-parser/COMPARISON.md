# Comparison: Example 175 — JSON Parser

## JSON type

**OCaml:**
```ocaml
type json =
  | Null
  | Bool of bool
  | Number of float
  | String of string
  | Array of json list
  | Object of (string * json) list
```

**Rust:**
```rust
#[derive(Debug, Clone, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number(f64),
    Str(String),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}
```

## Main dispatch

**OCaml:**
```ocaml
let rec parse_json input =
  let s = ws0 input in
  match s.[0] with
  | 'n' -> parse_keyword s "null" Null
  | 't' -> parse_keyword s "true" (Bool true)
  | 'f' -> parse_keyword s "false" (Bool false)
  | '"' -> parse_json_string s |> map (fun s -> String s)
  | '[' -> parse_array s
  | '{' -> parse_object s
  | '-' | '0'..'9' -> parse_json_number s
  | c -> Error (Printf.sprintf "Unexpected: '%c'" c)
```

**Rust:**
```rust
fn parse_json(input: &str) -> ParseResult<Json> {
    let s = input.trim_start();
    match s.as_bytes()[0] {
        b'n' => parse_keyword(s, "null", Json::Null),
        b't' => parse_keyword(s, "true", Json::Bool(true)),
        b'f' => parse_keyword(s, "false", Json::Bool(false)),
        b'"' => { let (v, r) = parse_json_string(s)?; Ok((Json::Str(v), r)) }
        b'[' => parse_array(s),
        b'{' => parse_object(s),
        b'-' | b'0'..=b'9' => parse_json_number(s),
        c => Err(format!("Unexpected: '{}'", c as char)),
    }
}
```

## Object parsing

**OCaml:**
```ocaml
and parse_object input =
  let rest = ws0 (String.sub input 1 ...) in
  if rest.[0] = '}' then Ok (Object [], ...)
  else
    let rec go acc remaining =
      match parse_json_string remaining with
      | Ok (key, rest) -> (* parse : then value, loop *)
    in go [] rest
```

**Rust:**
```rust
fn parse_object(input: &str) -> ParseResult<Json> {
    let mut remaining = input[1..].trim_start();
    if remaining.starts_with('}') { return Ok((Json::Object(vec![]), &remaining[1..])); }
    let mut entries = Vec::new();
    loop {
        let (key, rest) = parse_json_string(remaining)?;
        let rest = rest.trim_start();
        if !rest.starts_with(':') { return Err("Expected ':'".into()); }
        let (value, rest) = parse_json(&rest[1..])?;
        entries.push((key, value));
        // check for , or }
    }
}
```
