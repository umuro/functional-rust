# 173: Lisp / S-expression Parser

**Difficulty:** 3  **Level:** Advanced

Parse the simplest recursive syntax ever invented — and implement `'x` sugar and round-trip printing.

## The Problem This Solves

S-expressions are the syntax of Lisp: `(+ 1 (* 2 3))`, `(define x 42)`, `(if (> x 0) "positive" "non-positive")`. They're also used in configuration languages (Emacs Lisp), build tools (Clojure/Leiningen), and as a serialization format.

What makes S-expressions interesting to parse is that they're recursive by design. A list contains values, and values can be lists. The grammar is minimal — only a handful of token types — but the nesting can be arbitrarily deep. You need a recursive parser (as shown in example 167) plus atom classification (is `42` a number or a symbol?).

This example also shows syntactic sugar: Lisp's `'x` notation is shorthand for `(quote x)`. The parser expands it during parsing — a technique used in every Lisp, and in many other languages for similar shortcuts. Finally, a `Display` impl lets you print parsed values back to valid Lisp syntax.

## The Intuition

Parse one character at a time to decide what you're looking at: `(` → start a list, `"` → start a string, `'` → quote sugar, digits/sign → number, anything else → atom. For the list case, call `parse_sexp` recursively until you see `)`.

```
input: "(+ 1 (* 2 3))"
see '(' → start list
  atom: "+"
  number: 1
  see '(' → start list (recursive!)
    atom: "*"
    number: 2
    number: 3
  see ')' → end inner list: (* 2 3)
see ')' → end outer list: (+ 1 (* 2 3))
```

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
enum Sexp {
    Atom(String),
    Number(f64),
    Str(String),
    Bool(bool),
    Nil,
    List(Vec<Sexp>),
}

fn parse_sexp(input: &str) -> ParseResult<Sexp> {
    let input = input.trim_start();
    match input.chars().next() {
        Some('(') => parse_list(&input[1..]),
        Some('"') => parse_string(&input[1..]),
        Some('\'') => {
            // Quote sugar: 'x → (quote x)
            let (inner, rest) = parse_sexp(&input[1..])?;
            Ok((Sexp::List(vec![Sexp::Atom("quote".into()), inner]), rest))
        }
        Some('#') => {
            // Booleans: #t and #f
            if input.starts_with("#t") { Ok((Sexp::Bool(true),  &input[2..])) }
            else if input.starts_with("#f") { Ok((Sexp::Bool(false), &input[2..])) }
            else { Err("unknown # literal".to_string()) }
        }
        Some(_) => {
            // Atom or number — find the boundary
            let end = input.find(|c: char| c.is_whitespace() || "()\"'".contains(c))
                .unwrap_or(input.len());
            let token = &input[..end];
            let rest = &input[end..];
            if token == "nil" {
                Ok((Sexp::Nil, rest))
            } else if let Ok(n) = token.parse::<f64>() {
                Ok((Sexp::Number(n), rest))
            } else {
                Ok((Sexp::Atom(token.to_string()), rest))
            }
        }
        None => Err("unexpected end of input".to_string()),
    }
}

fn parse_list(input: &str) -> ParseResult<Sexp> {
    let mut items = Vec::new();
    let mut remaining = input;
    loop {
        remaining = remaining.trim_start();
        if remaining.starts_with(')') {
            return Ok((Sexp::List(items), &remaining[1..]));
        }
        let (item, rest) = parse_sexp(remaining)?;
        items.push(item);
        remaining = rest;
    }
}

// Display: print back to valid Lisp syntax
impl std::fmt::Display for Sexp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Sexp::Atom(s)   => write!(f, "{}", s),
            Sexp::Number(n) => write!(f, "{}", n),
            Sexp::Str(s)    => write!(f, "\"{}\"", s.replace('"', "\\\"")),
            Sexp::Bool(b)   => write!(f, "{}", if *b { "#t" } else { "#f" }),
            Sexp::Nil       => write!(f, "nil"),
            Sexp::List(xs)  => {
                write!(f, "(")?;
                for (i, x) in xs.iter().enumerate() {
                    if i > 0 { write!(f, " ")?; }
                    write!(f, "{}", x)?;
                }
                write!(f, ")")
            }
        }
    }
}
```

## What This Unlocks

- **Lisp dialects** — Scheme, Clojure, Racket, Emacs Lisp — all start here.
- **Homoiconic formats** — data-as-code formats where the AST is the data structure.
- **Syntactic sugar** — the `'x → (quote x)` pattern applies to `~x`, `` `x ``, and many other reader macros.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Variant type | `type sexp = Atom of string \| Number of float \| List of sexp list` | `enum Sexp { Atom(String), Number(f64), List(Vec<Sexp>) }` |
| Round-trip display | `let rec sexp_to_string = function ...` | `impl Display for Sexp` |
| Atom classification | `try float_of_string s with _ → Atom s` | `s.parse::<f64>().map_or(Atom, Number)` |
| Quote sugar | Pattern on `'\''` char | `input.starts_with('\'')` |
