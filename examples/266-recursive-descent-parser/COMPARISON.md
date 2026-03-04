# OCaml vs Rust: Recursive Descent Parser

## Side-by-Side Code

### OCaml

```ocaml
type expr = Num of int | Add of expr * expr | Mul of expr * expr

let rec parse_expr tokens =
  let left, rest = parse_term tokens in
  match rest with
  | "+" :: rest' ->
    let right, rest'' = parse_expr rest' in
    (Add (left, right), rest'')
  | _ -> (left, rest)
and parse_term tokens =
  let left, rest = parse_atom tokens in
  match rest with
  | "*" :: rest' ->
    let right, rest'' = parse_term rest' in
    (Mul (left, right), rest'')
  | _ -> (left, rest)
and parse_atom = function
  | n :: rest -> (Num (int_of_string n), rest)
  | [] -> failwith "unexpected end of input"

let rec eval = function
  | Num n -> n
  | Add (a, b) -> eval a + eval b
  | Mul (a, b) -> eval a * eval b
```

### Rust (functional — mirrors OCaml directly)

```rust
#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

pub fn parse_expr<'a>(tokens: &'a [&str]) -> (Expr, &'a [&str]) {
    let (left, rest) = parse_term(tokens);
    match rest {
        ["+", tail @ ..] => {
            let (right, remaining) = parse_expr(tail);
            (Expr::Add(Box::new(left), Box::new(right)), remaining)
        }
        _ => (left, rest),
    }
}

pub fn parse_term<'a>(tokens: &'a [&str]) -> (Expr, &'a [&str]) {
    let (left, rest) = parse_atom(tokens);
    match rest {
        ["*", tail @ ..] => {
            let (right, remaining) = parse_term(tail);
            (Expr::Mul(Box::new(left), Box::new(right)), remaining)
        }
        _ => (left, rest),
    }
}

pub fn parse_atom<'a>(tokens: &'a [&str]) -> (Expr, &'a [&str]) {
    match tokens {
        [n, rest @ ..] => {
            let num: i64 = n.parse().unwrap_or_else(|_| panic!("expected number"));
            (Expr::Num(num), rest)
        }
        [] => panic!("unexpected end of input"),
    }
}

pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}
```

### Rust (struct-based — idiomatic real-world style)

```rust
pub struct Parser<'a> {
    tokens: &'a [&'a str],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [&'a str]) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&'a str> {
        self.tokens.get(self.pos).copied()
    }

    fn advance(&mut self) { self.pos += 1; }

    pub fn parse_expr(&mut self) -> Expr {
        let left = self.parse_term();
        if self.peek() == Some("+") {
            self.advance();
            let right = self.parse_expr();
            Expr::Add(Box::new(left), Box::new(right))
        } else { left }
    }

    // parse_term and parse_atom follow the same pattern...
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| AST type | `type expr = Num of int \| Add of expr * expr \| Mul of expr * expr` | `enum Expr { Num(i64), Add(Box<Expr>, Box<Expr>), Mul(Box<Expr>, Box<Expr>) }` |
| Parse function | `val parse_expr : string list -> expr * string list` | `fn parse_expr<'a>(tokens: &'a [&str]) -> (Expr, &'a [&str])` |
| Eval function | `val eval : expr -> int` | `fn eval(expr: &Expr) -> i64` |
| Mutual recursion | `let rec f … and g …` | plain `fn f` + `fn g` (forward refs visible in module) |
| Token list tail | `string list` (persistent linked list) | `&'a [&str]` (borrowed sub-slice, zero-copy) |
| Recursive heap node | implicit (OCaml boxes all ADT variants) | `Box<Expr>` (explicit heap allocation) |

## Key Insights

1. **`Box` for recursive enums:** OCaml's garbage collector automatically allocates ADT values on the heap — the compiler decides. Rust requires the programmer to be explicit: `Box<Expr>` stores child nodes on the heap so that `Expr` has a known size at compile time. Without `Box`, the compiler would reject the recursive `enum`.

2. **Slice patterns vs list patterns:** OCaml's `"+" :: rest'` destructures a linked list head/tail. Rust's `["+", tail @ ..]` destructures a slice — it's syntactically identical in spirit but operates on a contiguous memory region rather than heap-linked nodes. The `@` binding captures the sub-slice as a pointer+length pair.

3. **Mutual recursion without special syntax:** OCaml requires `let rec … and …` to define mutually recursive functions. Rust functions within a module can reference each other freely — all names are resolved before execution, so no special syntax is needed. `parse_expr` can call `parse_term` even though it is defined first.

4. **Lifetime-threaded remainder:** OCaml lists are persistent: sharing a tail costs nothing (immutable linked lists support structural sharing). Rust's `&'a [&str]` sub-slices are also zero-cost — `tail @ ..` captures a pointer into the original slice with the same lifetime `'a`. No copying occurs at any parse step.

5. **Two parser architectures for the same grammar:** The functional approach (thread remainder slices) and the struct approach (cursor + `peek`/`advance`) implement identical grammars. The struct approach is preferred in production Rust parsers (`rustc`'s parser, `syn`, `logos`) because state is centralised, error recovery is easier, and look-ahead tokens can be stored in the struct. For simple recursive descent, the functional approach is more concise and mirrors the mathematical grammar definition directly.

## When to Use Each Style

**Use functional recursive descent (slice-threading) when:** the grammar is simple, you want a direct translation from a BNF grammar, or you are matching OCaml/Haskell implementations for educational comparison. Each function's signature precisely matches its grammar rule.

**Use struct-based recursive descent when:** building a real parser that needs error recovery, diagnostic messages, lookahead buffers, source-position tracking, or incremental parsing. The `Parser` struct can carry all that state without polluting every function signature.
