# OCaml vs Rust: Parser Combinator Pattern

## Basic Combinators

### Rust
```rust
pub fn char_p(c: char) -> impl Fn(&str) -> ParseResult<char> {
    move |input| {
        input.chars().next()
            .filter(|&ch| ch == c)
            .map(|ch| (ch, &input[ch.len_utf8()..]))
    }
}

pub fn then<A, B>(p1: P1, p2: P2) -> impl Fn(&str) -> ParseResult<(A, B)> {
    move |input| {
        let (a, rest) = p1(input)?;
        let (b, rest) = p2(rest)?;
        Some(((a, b), rest))
    }
}
```

### OCaml
```ocaml
let char_p c input =
  match String.get_opt input 0 with
  | Some ch when ch = c -> Some (ch, String.sub input 1 ...)
  | _ -> None

let (>>=) p1 p2 input =
  match p1 input with
  | Some (a, rest) ->
      (match p2 rest with
       | Some (b, rest') -> Some ((a, b), rest')
       | None -> None)
  | None -> None
```

## Combinator Types

| Combinator | Purpose |
|------------|---------|
| `char_p` | Match single char |
| `string_p` | Match string |
| `map` | Transform result |
| `then` | Sequence |
| `or` | Alternative |
| `many` | Zero or more |
| `sep_by` | Separated list |

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Closure syntax | `fun x -> ...` | `\|x\| ...` or `move \|x\| ...` |
| Return type | `('a * string) option` | `Option<(T, &str)>` |
| Higher-order | First-class | `impl Fn` or `Box<dyn Fn>` |
| Lifetime | GC | Explicit `'a` |
