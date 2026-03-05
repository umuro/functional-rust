# 771: Pratt Parser for Operator Precedence Expressions

**Difficulty:** 4  **Level:** Advanced

Operator precedence and associativity driven by a binding-power table — one loop replaces the grammar-per-precedence-level pattern of recursive descent.

## The Problem This Solves

Recursive descent handles operator precedence by nesting grammar rules: `expr` → `term` → `factor` → ... Each precedence level needs its own rule. For a language with 5 precedence levels that's 5 methods. For a language like Rust with 15 precedence levels, that's unmanageable.

Pratt parsing (invented by Vaughan Pratt in 1973) is more scalable: each operator declares its *binding power* (precedence) and *associativity* as data. The parser is a single loop that consults this table instead of a call graph. Adding a new operator means adding one entry to the table — not modifying the parser.

Pratt parsers also handle right-associative operators cleanly. Exponentiation `2^3^4` should parse as `2^(3^4)`, not `(2^3)^4`. In recursive descent, right-associativity requires a separate grammar rule. In Pratt, it's a single integer change: the right binding power is one less than the left.

## The Intuition

The key insight: every operator has two binding powers — left and right. For `+` (left-associative, low precedence), both are equal and low: `(10, 11)`. For `*` (left-associative, higher precedence): `(20, 21)`. For `^` (right-associative): `(40, 39)` — the right side is *lower*, so the next `^` "captures" rightward.

The main loop: parse a "prefix" (a number or unary operator). Then, as long as the *next* operator's left binding power is greater than our current minimum, consume it and parse its right side. The minimum power is what controls nesting depth.

This is the algorithm behind `rustc`'s own expression parser, `Lua`, and many professional compiler frontends.

## How It Works in Rust

```rust
// Each operator has (left_bp, right_bp) — right lower = right-associative
fn infix_bp(op: char) -> Option<(u8, u8)> {
    match op {
        '+' | '-' => Some((10, 11)),  // low precedence, left-assoc
        '*' | '/' => Some((20, 21)),  // higher precedence, left-assoc
        '^'       => Some((40, 39)),  // highest, RIGHT-associative (right < left)
        _         => None,
    }
}

fn prefix_bp(op: char) -> Option<u8> {
    match op { '-' => Some(30), _ => None }  // unary minus: high precedence
}

// One function, one loop — no per-precedence-level functions
fn parse_expr(lexer: &mut Lexer, min_bp: u8) -> Result<Expr, ParseError> {
    // Parse prefix: number or unary operator
    let mut lhs = match lexer.next() {
        Token::Num(n) => Expr::Num(n),
        Token::Minus  => {
            let bp = prefix_bp('-').unwrap();
            let rhs = parse_expr(lexer, bp)?;   // recurse for unary right side
            Expr::Unary { op: '-', operand: Box::new(rhs) }
        }
        Token::LParen => {
            let expr = parse_expr(lexer, 0)?;   // full sub-expression
            assert_eq!(lexer.next(), Token::RParen);
            expr
        }
        t => return Err(ParseError(format!("unexpected token: {t:?}"))),
    };

    loop {
        let op = match lexer.peek() {
            Token::Plus  => '+',
            Token::Star  => '*',
            Token::Caret => '^',
            Token::Eof | Token::RParen => break,
            t => return Err(ParseError(format!("unexpected: {t:?}"))),
        };

        let (l_bp, r_bp) = match infix_bp(op) {
            Some(bp) => bp,
            None     => break,          // not an infix operator here
        };

        if l_bp < min_bp { break; }     // this operator binds less tightly — stop

        lexer.next();                   // consume the operator
        let rhs = parse_expr(lexer, r_bp)?;  // recurse with RIGHT binding power

        lhs = Expr::Binary { op, left: Box::new(lhs), right: Box::new(rhs) };
    }

    Ok(lhs)
}

// Public entry point
pub fn parse(input: &str) -> Result<Expr, ParseError> {
    parse_expr(&mut Lexer::new(input), 0)
}
```

Examples:
- `"1 + 2 * 3"` → `1 + (2 * 3)` = 7 (because `*` bp 20 > `+` bp 10)
- `"2 ^ 3 ^ 4"` → `2 ^ (3 ^ 4)` = 2^81 (right-assoc: right bp 39 < left bp 40)
- `"-2 ^ 2"` → `-(2^2)` = -4 (unary minus bp 30 < `^` bp 40)
- `"(1 + 2) * 3"` → `(1+2) * 3` = 9 (parentheses reset `min_bp` to 0)

Key points:
- `min_bp` is the current "floor" — only consume operators that bind tighter
- Right-associativity: use `r_bp = l_bp - 1` so the same operator on the right is consumed
- Unary prefix operators have only a right binding power — no left side to compete with
- The lexer is a separate `struct Lexer` with `peek()` and `next()` — never consume tokens you're not sure about

## What This Unlocks

- **Scalable operator tables**: add `%`, `<<`, `>>`, `&&`, `||` by adding entries to `infix_bp` — no parser logic changes
- **Expression languages in compilers**: `rustc`, LuaJIT, and V8 all use Pratt or equivalent — this is the production technique
- **Mixed prefix/infix/postfix**: Pratt handles `++i`, `i++`, `-i`, `i?` (Rust's `?` operator) with different BP rules — recursive descent struggles with this

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Precedence encoding | Grammar rules or precedence tables | Binding power function returning `(u8, u8)` |
| Operator table | Separate `precedence_of` function | `infix_bp(op: char) -> Option<(u8, u8)>` |
| Right-associativity | Separate grammar rule | `r_bp = l_bp - 1` |
| Unary operators | Prefix grammar rule | `prefix_bp` returning `Option<u8>` |
| Loop vs recursion | Recursive style natural | `loop` with `break` on lower precedence |
| Production use | `menhir` precedence declarations | `rustc`'s own expression parser uses this algorithm |
