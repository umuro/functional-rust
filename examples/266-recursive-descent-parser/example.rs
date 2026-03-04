//! Recursive Descent Parser — standalone example (mirrors src/lib.rs public API)

/// AST node for arithmetic expressions.
/// `Box` enables recursive heap allocation — required in Rust for recursive enums.
#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

// ── Solution 1: Functional recursive descent (mirrors OCaml) ─────────────────

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

pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(a, b) => eval(a) + eval(b),
        Expr::Mul(a, b) => eval(a) * eval(b),
    }
}

// ── Solution 2: Struct-based parser (idiomatic Rust) ─────────────────────────

pub struct Parser<'t, 's> {
    tokens: &'t [&'s str],
    pos: usize,
}

impl<'t, 's> Parser<'t, 's> {
    pub fn new(tokens: &'t [&'s str]) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&'s str> {
        self.tokens.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

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

    pub fn parse_atom(&mut self) -> Expr {
        match self.peek() {
            Some(n) => {
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

pub fn eval_str(input: &str) -> i64 {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let (ast, _) = parse_expr(&tokens);
    eval(&ast)
}

fn main() {
    // Functional approach: thread token slices through return values
    let tokens = ["2", "+", "3", "*", "4"];
    let (ast, _) = parse_expr(&tokens);
    println!("Functional parser:");
    println!("  AST: {:?}", ast);
    println!("  2+3*4 = {} (expected 14)", eval(&ast));

    // Struct-based approach: cursor advances through the token slice
    println!("\nStruct-based Parser:");
    let tokens2 = ["1", "*", "2", "+", "3", "*", "4"];
    let mut p = Parser::new(&tokens2);
    let ast2 = p.parse_expr();
    println!("  1*2+3*4 = {} (expected 14)", eval(&ast2));

    // Convenience function: tokenise from string, parse, evaluate
    println!("\neval_str convenience:");
    println!("  eval_str(\"2 + 3 * 4\") = {}", eval_str("2 + 3 * 4"));
    println!("  eval_str(\"10 * 2 + 5\") = {}", eval_str("10 * 2 + 5"));
    println!("  eval_str(\"1 + 2 + 3\") = {}", eval_str("1 + 2 + 3"));
}

/* Output:
   Functional parser:
     AST: Add(Num(2), Mul(Num(3), Num(4)))
     2+3*4 = 14 (expected 14)

   Struct-based Parser:
     1*2+3*4 = 14 (expected 14)

   eval_str convenience:
     eval_str("2 + 3 * 4") = 14
     eval_str("10 * 2 + 5") = 25
     eval_str("1 + 2 + 3") = 6
*/

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(eval(&ast), 5);
    }

    #[test]
    fn test_multiplication() {
        let tokens = ["3", "*", "4"];
        let (ast, _) = parse_expr(&tokens);
        assert_eq!(eval(&ast), 12);
    }

    #[test]
    fn test_precedence_mul_before_add() {
        let tokens = ["2", "+", "3", "*", "4"];
        let (ast, _) = parse_expr(&tokens);
        assert_eq!(eval(&ast), 14);
    }

    #[test]
    fn test_ast_structure_for_2_plus_3_times_4() {
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
        let tokens = ["1", "+", "2", "+", "3"];
        let (ast, _) = parse_expr(&tokens);
        assert_eq!(eval(&ast), 6);
    }

    #[test]
    fn test_complex_expression() {
        let tokens = ["1", "*", "2", "+", "3", "*", "4"];
        let (ast, _) = parse_expr(&tokens);
        assert_eq!(eval(&ast), 14);
    }

    #[test]
    fn test_parser_struct_basic() {
        let tokens = ["2", "+", "3", "*", "4"];
        let mut p = Parser::new(&tokens);
        let ast = p.parse_expr();
        assert_eq!(eval(&ast), 14);
    }

    #[test]
    fn test_parser_struct_matches_functional() {
        let tokens = ["5", "*", "6", "+", "7"];
        let (func_ast, _) = parse_expr(&tokens);
        let mut p = Parser::new(&tokens);
        let struct_ast = p.parse_expr();
        assert_eq!(func_ast, struct_ast);
        assert_eq!(eval(&func_ast), 37);
    }

    #[test]
    fn test_eval_str_precedence() {
        assert_eq!(eval_str("2 + 3 * 4"), 14);
        assert_eq!(eval_str("10 * 2 + 5"), 25);
        assert_eq!(eval_str("7"), 7);
    }
}
