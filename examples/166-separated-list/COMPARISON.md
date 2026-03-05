# Comparison: Example 166 — Separated List

## separated_list0

**OCaml:**
```ocaml
let separated_list0 (sep : 'b parser) (item : 'a parser) : 'a list parser = fun input ->
  match item input with
  | Error _ -> Ok ([], input)
  | Ok (first, rest) ->
    let rec go acc remaining =
      match sep remaining with
      | Error _ -> Ok (List.rev acc, remaining)
      | Ok (_, after_sep) ->
        match item after_sep with
        | Error _ -> Ok (List.rev acc, remaining)
        | Ok (v, rest') -> go (v :: acc) rest'
    in go [first] rest
```

**Rust:**
```rust
fn separated_list0<'a, T: 'a, S: 'a>(
    sep: Parser<'a, S>, item: Parser<'a, T>,
) -> Parser<'a, Vec<T>> {
    Box::new(move |input: &'a str| {
        let (first, mut remaining) = match item(input) {
            Err(_) => return Ok((vec![], input)),
            Ok(r) => r,
        };
        let mut results = vec![first];
        loop {
            let after_sep = match sep(remaining) {
                Err(_) => break,
                Ok((_, r)) => r,
            };
            match item(after_sep) {
                Ok((val, rest)) => { results.push(val); remaining = rest; }
                Err(_) => break,
            }
        }
        Ok((results, remaining))
    })
}
```
