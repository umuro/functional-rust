//! Recursive Descent Parser
//!
//! OCaml uses `type expr = Num of int | Add of expr * expr | Mul of expr * expr`
//! with mutually recursive `parse_expr`/`parse_term`/`parse_atom` functions.
//! Rust expresses the same structure with `enum Expr` + `Box` for heap-allocated
//! recursive nodes, and mutually recursive free functions or struct methods.

/// AST node for arithmetic expressions.
///
/// `Box` enables recursive heap allocation — required in Rust for recursive enums.
/// OCaml automatically boxes all algebraic data type values on the heap.
#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

// ── Solution 1: Functional recursive descent (mirrors OCaml) ─────────────────
//
// Grammar (right-associative, matching the OCaml direct recursion):
//   expr ::= term ('+' expr)?
//   term ::= atom ('*' term)?
//   atom ::= NUMBER
//
// Each function consumes a prefix of the token slice and returns
// (parsed_expr, unconsumed_remainder) — identical to OCaml's list-threading.
//
// Two lifetime parameters are needed:
//   't — lifetime of the slice reference itself (how long the &[...] borrow lasts)
//   's — lifetime of the &str elements inside the slice (how long the strings live)
// Keeping them separate lets callers hold a short-lived &[&str] over long-lived string data.

/// Parses an additive expression, handling `+` right-associatively.
///
/// OCaml: `let rec parse_expr tokens = let left, rest = parse_term tokens in ...`
/// Rust uses slice patterns `["+", tail @ ..]` instead of list head/tail matching.
pub fn parse_expr<'t, 's>(tokens: &'t [&'s str]) -> (Expr, &'t [&'s str]) {
    let (left, rest) = parse_term(tokens);
    match rest {
        ["+", tail @ ..] => {
            let (right, remaining) = parse_expr(tail);
            (Expr::Add(Box::new(left), Box::new(right)), remaining)
        }
        _ => (left, rest),
    }
}

/// Parses a multiplicative expression, handling `*` right-associatively.
pub fn parse_term<'t, 's>(tokens: &'t [&'s str]) -> (Expr, &'t [&'s str]) {
    let (left, rest) = parse_atom(tokens);
    match rest {
        ["*", tail @ ..] => {
            let (right, remaining) = parse_term(tail);
            (Expr::Mul(Box::new(left), Box::new(right)), remaining)
        }
        _ => (left, rest),
    }
}

/// Parses a numeric atom.
///
/// OCaml: `| n :: rest -> (Num (int_of_string n), rest) | [] -> failwith "unexpected end"`
/// Rust panics on invalid input, mirroring OCaml's `failwith`.
pub fn parse_atom<'t, 's>(tokens: &'t [&'s str]) -> (Expr, &'t [&'s str]) {
    match tokens {
        [n, rest @ ..] => {
            let num: i64 = n
                .parse()
                .unwrap_or_else(|_| panic!("expected a number, got {n:?}"));
            (Expr::Num(num), rest)
        }
        [] => panic!("unexpected end of input"),
    }
}

/// Evaluates an expression tree to an integer.
///
/// OCaml: `let rec eval = function | Num n -> n | Add(a,b) -> eval a + eval b | ...`
/// Rust takes `&Expr` — borrows the tree without consuming it.
pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

// ── Solution 2: Struct-based parser (idiomatic Rust) ─────────────────────────
//
// A `Parser` struct holds the token slice and a cursor position.
// Methods advance the cursor rather than threading remainder slices.
// This is the pattern used by real-world Rust parsers (rustc, syn, logos, etc.).

/// A recursive-descent parser that tracks position with a cursor index.
///
/// Vs the functional approach: state is encapsulated in `self` rather than
/// threaded through return values. Both implement the same grammar.
pub struct Parser<'t, 's> {
    // Borrows the token slice — no allocation needed for the token list itself.
    // 't: lifetime of the slice reference; 's: lifetime of the string data.
    tokens: &'t [&'s str],
    pos: usize,
}

impl<'t, 's> Parser<'t, 's> {
    pub fn new(tokens: &'t [&'s str]) -> Self {
        Parser { tokens, pos: 0 }
    }

    /// Returns the current token without consuming it.
    fn peek(&self) -> Option<&'s str> {
        // `.copied()` converts `Option<&&str>` → `Option<&str>` since `&str: Copy`.
        self.tokens.get(self.pos).copied()
    }

    /// Advances past the current token.
    fn advance(&mut self) {
        self.pos += 1;
    }

    /// Parses an additive expression.
    pub fn parse_expr(&mut self) -> Expr {
        let left = self.parse_term();
        if self.peek() == Some("+") {
            self.advance();
            let right = self.parse_expr();
            Expr::Add(Box::new(left), Box::new(right))
        } else {
            left
        }
    }

    /// Parses a multiplicative expression.
    pub fn parse_term(&mut self) -> Expr {
        let left = self.parse_atom();
        if self.peek() == Some("*") {
            self.advance();
            let right = self.parse_term();
            Expr::Mul(Box::new(left), Box::new(right))
        } else {
            left
        }
    }

    /// Parses a numeric atom.
    pub fn parse_atom(&mut self) -> Expr {
        match self.peek() {
            Some(n) => {
                // Copy `n` out before calling `advance()` (which mutably borrows self).
                let num: i64 = n
                    .parse()
                    .unwrap_or_else(|_| panic!("expected number, got {n:?}"));
                self.advance();
                Expr::Num(num)
            }
            None => panic!("unexpected end of input"),
        }
    }
}

/// Convenience: tokenise a space-separated expression string and evaluate it.
///
/// Builds the token `Vec` locally; the `'t` (slice) and `'s` (string) lifetimes
/// are both satisfied since the strings borrow from `input` and the Vec outlives
/// the `parse_expr` call.
pub fn eval_str(input: &str) -> i64 {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let (ast, _) = parse_expr(&tokens);
    eval(&ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Functional parser tests ───────────────────────────────────────────────

    #[test]
    fn test_single_number() {
        let tokens = ["42"];
        let (ast, rest) = parse_expr(&tokens);
        assert_eq!(ast, Expr::Num(42));
        assert!(rest.is_empty());
        assert_eq!(eval(&ast), 42);
    }

    #[test]
    fn test_addition() {
        let tokens = ["2", "+", "3"];
        let (ast, _) = parse_expr(&tokens);
        assert_eq!(
            ast,
            Expr::Add(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)))
        );
        assert_eq!(eval(&ast), 5);
    }

    #[test]
    fn test_multiplication() {
        let tokens = ["3", "*", "4"];
        let (ast, _) = parse_expr(&tokens);
        assert_eq!(
            ast,
            Expr::Mul(Box::new(Expr::Num(3)), Box::new(Expr::Num(4)))
        );
        assert_eq!(eval(&ast), 12);
    }

    #[test]
    fn test_precedence_mul_before_add() {
        // 2 + 3 * 4 must evaluate to 14 (not 20) — * binds tighter than +
        let tokens = ["2", "+", "3", "*", "4"];
        let (ast, _) = parse_expr(&tokens);
        assert_eq!(eval(&ast), 14);
    }

    #[test]
    fn test_ast_structure_for_2_plus_3_times_4() {
        // Verify the tree: Add(Num(2), Mul(Num(3), Num(4)))
        let tokens = ["2", "+", "3", "*", "4"];
        let (ast, _) = parse_expr(&tokens);
        let expected = Expr::Add(
            Box::new(Expr::Num(2)),
            Box::new(Expr::Mul(Box::new(Expr::Num(3)), Box::new(Expr::Num(4)))),
        );
        assert_eq!(ast, expected);
    }

    #[test]
    fn test_chained_addition_is_right_associative() {
        // 1 + 2 + 3 → Add(1, Add(2, 3)) = 6 (right-associative, matches OCaml)
        let tokens = ["1", "+", "2", "+", "3"];
        let (ast, _) = parse_expr(&tokens);
        assert_eq!(eval(&ast), 6);
        let expected = Expr::Add(
            Box::new(Expr::Num(1)),
            Box::new(Expr::Add(Box::new(Expr::Num(2)), Box::new(Expr::Num(3)))),
        );
        assert_eq!(ast, expected);
    }

    #[test]
    fn test_complex_expression() {
        // 1 * 2 + 3 * 4 = 2 + 12 = 14
        let tokens = ["1", "*", "2", "+", "3", "*", "4"];
        let (ast, _) = parse_expr(&tokens);
        assert_eq!(eval(&ast), 14);
    }

    // ── Struct-based parser tests ─────────────────────────────────────────────

    #[test]
    fn test_parser_struct_basic() {
        let tokens = ["2", "+", "3", "*", "4"];
        let mut p = Parser::new(&tokens);
        let ast = p.parse_expr();
        assert_eq!(eval(&ast), 14);
    }

    #[test]
    fn test_parser_struct_matches_functional() {
        // Both approaches must produce identical ASTs
        let tokens = ["5", "*", "6", "+", "7"];
        let (func_ast, _) = parse_expr(&tokens);
        let mut p = Parser::new(&tokens);
        let struct_ast = p.parse_expr();
        assert_eq!(func_ast, struct_ast);
        assert_eq!(eval(&func_ast), 37);
    }

    // ── eval_str convenience tests ────────────────────────────────────────────

    #[test]
    fn test_eval_str_precedence() {
        assert_eq!(eval_str("2 + 3 * 4"), 14);
        assert_eq!(eval_str("10 * 2 + 5"), 25);
        assert_eq!(eval_str("7"), 7);
    }
}
