# OCaml vs Rust: Lookahead with Peekable

## Side-by-Side Code

### OCaml

```ocaml
(* OCaml has no built-in peekable iterator; carry a mutable lookahead ref *)
type 'a peekable = {
  mutable peeked: 'a option;
  mutable rest: 'a list;
}

let make_peekable lst = { peeked = None; rest = lst }

let peek p =
  match p.peeked with
  | Some _ as v -> v
  | None ->
    match p.rest with
    | [] -> None
    | x :: _ -> p.peeked <- Some x; Some x

let next p =
  match p.peeked with
  | Some v ->
    p.peeked <- None;
    (match p.rest with _ :: xs -> p.rest <- xs | [] -> ());
    Some v
  | None ->
    match p.rest with
    | [] -> None
    | x :: xs -> p.rest <- xs; Some x
```

### Rust (idiomatic — stdlib Peekable)

```rust
pub fn group_consecutive<T: PartialEq + Copy>(data: &[T]) -> Vec<Vec<T>> {
    let mut iter = data.iter().peekable();
    let mut groups = Vec::new();

    while let Some(&val) = iter.peek() {
        let mut group = Vec::new();
        while iter.peek() == Some(&val) {
            group.push(*iter.next().unwrap());
        }
        groups.push(group);
    }
    groups
}
```

### Rust (tokenizer using Peekable)

```rust
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                let mut num_str = String::new();
                while chars.peek().map_or(false, |c| c.is_ascii_digit()) {
                    num_str.push(chars.next().unwrap());
                }
                tokens.push(Token::Number(num_str.parse().unwrap_or(0)));
            }
            '+' => { chars.next(); tokens.push(Token::Plus); }
            '-' => { chars.next(); tokens.push(Token::Minus); }
            ' ' => { chars.next(); }
            other => { chars.next(); tokens.push(Token::Unknown(other)); }
        }
    }
    tokens
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Peekable wrapper | `'a peekable` (custom mutable record) | `Peekable<I: Iterator>` (stdlib adapter) |
| Peek operation | `peek : 'a peekable -> 'a option` | `fn peek(&mut self) -> Option<&<I as Iterator>::Item>` |
| Advance | `next : 'a peekable -> 'a option` | `fn next(&mut self) -> Option<I::Item>` |
| Construction | `make_peekable : 'a list -> 'a peekable` | `.peekable()` on any iterator |

## Key Insights

1. **Standard vs. manual**: Rust's stdlib ships `Peekable<I>` as a zero-cost iterator adapter; OCaml's standard library has no equivalent, requiring a hand-rolled mutable record with a `peeked` slot.

2. **Ownership through the peek reference**: `peek()` returns `Option<&Item>` — a reference into the adapter's internal buffer. You can inspect the value without moving it; calling `next()` later yields the owned value.

3. **No "put-back" needed**: Both languages solve the same problem — you can't un-consume an iterator element — but Rust encapsulates the buffer inside `Peekable`, whereas OCaml requires the caller to manage the `peeked` field manually.

4. **Lifetime-free lookahead**: Even though `peek()` returns a reference, the borrow checker ensures it does not escape past the next `next()` call — no unsafe code, no lifetimes visible at the call site.

5. **Composability**: Because `Peekable<I>` itself implements `Iterator`, it can be chained with `.map()`, `.filter()`, `.zip()` and any other adapter — OCaml's mutable record cannot.

## When to Use Each Style

**Use idiomatic Rust (`Peekable`)** when you need to inspect the next element in any iterator pipeline — parsers, tokenizers, run-length encoders, merge steps.

**Use the recursive/functional style** when the algorithm is inherently recursive and the lookahead is captured naturally by pattern matching on the tail of a list — e.g., `match list with [a; b; ..rest]`.
