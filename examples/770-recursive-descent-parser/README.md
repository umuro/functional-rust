📖 **[View on hightechmind.io →](https://hightechmind.io/rust/770-recursive-descent-parser)**

---

# 770: Recursive Descent Parser from Scratch

**Difficulty:** 4  **Level:** Advanced

Turn a context-free grammar directly into mutually recursive functions — each grammar rule becomes a method, and the parser walks the input building an evaluatable AST.

## The Problem This Solves

Regular expressions can match patterns, but they can't parse nested structure. `"(1 + 2) * (3 + 4)"` has parentheses that can be arbitrarily deep — you need a parser that calls itself recursively to handle them. Recursive descent is the most readable parsing technique: each grammar rule maps to one function, and you can read the code as documentation of the grammar.

Understanding recursive descent unlocks a whole class of tools: you can write calculators, template engines, configuration DSLs, query languages, and custom command parsers. It's also the foundation for understanding how Rust's own compiler parses source code, and how tools like `nom` and `winnow` organize their combinators.

The grammar for arithmetic expressions also teaches operator precedence by structure. `*` binds tighter than `+` because multiplication is handled at a deeper recursion level. This structural encoding of precedence is cleaner than looking up a precedence table.

## The Intuition

Imagine the grammar written out:
```
expr   → term  (('+'|'-') term)*
term   → factor (('*'|'/') factor)*
factor → '(' expr ')' | number
```

Each line becomes one method. `parse_expr` calls `parse_term`. `parse_term` calls `parse_factor`. `parse_factor` either calls `parse_expr` (for parentheses) or reads a number. The recursion in the grammar becomes recursion in the code.

In Python, you'd write the same structure — a class with methods for each rule. The difference in Rust is that the parser is position-tracking (`pos: usize`) and all error cases return `Result`. This matches production parsers like `syn` (Rust's macro parser) exactly.

## How It Works in Rust

```rust
// Grammar:
// expr   → term (('+'|'-') term)*
// term   → factor (('*'|'/') factor)*
// factor → '(' expr ')' | number

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    BinOp { op: char, left: Box<Expr>, right: Box<Expr> },
}

pub struct Parser<'a> { input: &'a [u8], pos: usize }

impl<'a> Parser<'a> {
    // expr: lowest precedence — handles + and -
    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;        // always parse a term first
        self.skip_ws();
        while matches!(self.peek(), Some('+') | Some('-')) {
            let op = self.advance_op();
            let right = self.parse_term()?;       // right side is also a term
            left = Expr::BinOp {
                op, left: Box::new(left), right: Box::new(right)
            };
        }
        Ok(left)
    }

    // term: higher precedence — handles * and /
    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_factor()?;      // always parse a factor first
        self.skip_ws();
        while matches!(self.peek(), Some('*') | Some('/')) {
            let op = self.advance_op();
            let right = self.parse_factor()?;
            left = Expr::BinOp { op, left: Box::new(left), right: Box::new(right) };
        }
        Ok(left)
    }

    // factor: highest precedence — parentheses or a literal number
    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        self.skip_ws();
        if self.peek() == Some('(') {
            self.advance();                        // consume '('
            let expr = self.parse_expr()?;         // RECURSE — full expression inside parens
            self.skip_ws();
            self.expect(')')?;                     // consume ')'
            return Ok(expr);
        }
        Ok(Expr::Num(self.parse_number()?))        // base case: a literal
    }
}

// The AST evaluates itself
impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(n) => *n,
            Expr::BinOp { op, left, right } => {
                let (l, r) = (left.eval(), right.eval());
                match op { '+' => l+r, '-' => l-r, '*' => l*r, '/' => l/r, _ => 0.0 }
            }
        }
    }
}

// Usage
pub fn eval(input: &str) -> Result<f64, ParseError> {
    Ok(Parser::new(input).parse_expr()?.eval())
}

// "2 + 3 * 4" → 14 (not 20 — precedence is correct because term is called before +)
// "(2 + 3) * 4" → 20
```

Key points:
- `Box<Expr>` is required for recursive enum variants — otherwise the type would be infinitely sized
- The precedence hierarchy is: `expr` → `term` → `factor` → number/`(expr)` — tighter binding = deeper recursion level
- `parse_factor` calling `parse_expr` creates the recursive cycle that handles nested parentheses
- Error type `ParseError(String)` keeps it simple — production parsers add span information (byte offset, line/column)
- `while matches!(...)` handles left-associativity: `1 - 2 - 3` = `(1-2)-3`, not `1-(2-3)`

## What This Unlocks

- **Custom DSLs**: a query language, template engine, or expression evaluator follows this exact pattern — grammar → rules → methods → AST → evaluator
- **Foundation for Pratt parsing**: once you understand recursive descent, the Pratt technique (example 771) is a natural evolution for handling operator precedence tables
- **Production parsing**: `syn` (Rust macro AST), `swc` (JavaScript compiler), and many language servers use recursive descent — this is the same technique at scale

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Parser state | Functional: thread input through returns | Mutable `Parser` struct with `pos: usize` |
| AST recursive type | `type expr = Num of float \| BinOp of ...` | `enum Expr` with `Box<Expr>` for recursion |
| Precedence | Separate parser functions per level | Same — `parse_expr` calls `parse_term` calls `parse_factor` |
| Error handling | Exceptions or `result` | `Result<Expr, ParseError>` — propagated with `?` |
| Left associativity | Accumulate via function recursion | `while` loop with left-accumulation |
| Production library | `menhir`, `angstrom` | `nom`, `winnow`, `pest`, `lalrpop` |
