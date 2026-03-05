# Comparison: Example 173 — Lisp Parser

## Data type

**OCaml:**
```ocaml
type sexp =
  | Atom of string
  | Number of float
  | Str of string
  | List of sexp list
  | Bool of bool
  | Nil
```

**Rust:**
```rust
#[derive(Debug, Clone, PartialEq)]
enum Sexp {
    Atom(String),
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    List(Vec<Sexp>),
}
```

## Main dispatch

**OCaml:**
```ocaml
let rec parse_sexp input =
  let s = ws0 input in
  if s.[0] = '(' then parse_list s
  else if s.[0] = '\'' then
    match parse_sexp (String.sub s 1 ...) with
    | Ok (v, rest) -> Ok (List [Atom "quote"; v], rest)
  else if s.[0] = '"' then parse_string s
  else parse_atom s
```

**Rust:**
```rust
fn parse_sexp(input: &str) -> ParseResult<Sexp> {
    let s = input.trim_start();
    match s.chars().next().unwrap() {
        '(' => parse_list(s),
        '\'' => {
            let (val, rest) = parse_sexp(&s[1..])?;
            Ok((Sexp::List(vec![Sexp::Atom("quote".into()), val]), rest))
        }
        '"' => parse_string(s),
        _ => parse_atom(s),
    }
}
```
