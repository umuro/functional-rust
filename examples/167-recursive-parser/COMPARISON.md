# Comparison: Example 167 — Recursive Parser

## Direct recursion

**OCaml:**
```ocaml
let rec parse_sexp input =
  match satisfy (fun c -> c >= 'a' && c <= 'z') "letter" input with
  | Ok (c, rest) -> (* parse atom continuation *)
  | Error _ -> parse_sexp_list input

and parse_sexp_list input =
  match tag "(" input with
  | Ok (_, rest) -> (* parse list items using parse_sexp *)
```

**Rust:**
```rust
fn parse_sexp(input: &str) -> ParseResult<Sexp> {
    if let Some(c) = input.chars().next() {
        if c.is_ascii_lowercase() { /* parse atom */ }
    }
    parse_sexp_list(input)
}

fn parse_sexp_list(input: &str) -> ParseResult<Sexp> {
    if !input.starts_with('(') { return Err(...); }
    // parse items using parse_sexp
}
```

## Fix-point combinator

**OCaml:**
```ocaml
let fix (f : 'a parser -> 'a parser) : 'a parser =
  let rec p input = (f p) input in p
```

**Rust:**
```rust
fn rc_fix<'a, T: 'a>(
    f: impl Fn(RcParser<'a, T>) -> RcParser<'a, T> + 'a,
) -> RcParser<'a, T> {
    let parser: Rc<RefCell<Option<RcParser<'a, T>>>> = Rc::new(RefCell::new(None));
    let parser_clone = parser.clone();
    let lazy: RcParser<'a, T> = Rc::new(move |input| {
        (parser_clone.borrow().as_ref().unwrap())(input)
    });
    let actual = f(lazy);
    *parser.borrow_mut() = Some(actual.clone());
    actual
}
```
