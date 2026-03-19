#![allow(clippy::all)]
//! # Pratt Parser
//!
//! Operator precedence parsing using the Pratt algorithm.

/// Token types
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Eof,
}

/// AST
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Ident(String),
    Prefix {
        op: char,
        expr: Box<Expr>,
    },
    Infix {
        op: char,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

/// Get binding power (precedence) for infix operators
fn infix_binding_power(op: char) -> Option<(u8, u8)> {
    match op {
        '+' | '-' => Some((1, 2)), // left associative
        '*' | '/' => Some((3, 4)), // left associative
        '^' => Some((6, 5)),       // right associative
        _ => None,
    }
}

/// Get binding power for prefix operators
fn prefix_binding_power(op: char) -> Option<u8> {
    match op {
        '+' | '-' => Some(5),
        _ => None,
    }
}

/// Lexer
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, pos: 0 }
    }

    fn peek_char(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) {
        if let Some(c) = self.peek_char() {
            self.pos += c.len_utf8();
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        match self.peek_char() {
            None => Token::Eof,
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('-') => {
                self.advance();
                Token::Minus
            }
            Some('*') => {
                self.advance();
                Token::Star
            }
            Some('/') => {
                self.advance();
                Token::Slash
            }
            Some('^') => {
                self.advance();
                Token::Caret
            }
            Some('(') => {
                self.advance();
                Token::LParen
            }
            Some(')') => {
                self.advance();
                Token::RParen
            }
            Some(c) if c.is_ascii_digit() => {
                let start = self.pos;
                while let Some(c) = self.peek_char() {
                    if c.is_ascii_digit() || c == '.' {
                        self.advance();
                    } else {
                        break;
                    }
                }
                Token::Number(self.input[start..self.pos].parse().unwrap())
            }
            Some(c) if c.is_alphabetic() => {
                let start = self.pos;
                while let Some(c) = self.peek_char() {
                    if c.is_alphanumeric() {
                        self.advance();
                    } else {
                        break;
                    }
                }
                Token::Ident(self.input[start..self.pos].to_string())
            }
            Some(c) => panic!("Unexpected: {}", c),
        }
    }
}

/// Pratt parser
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer::new(input);
        let current = lexer.next_token();
        Parser { lexer, current }
    }

    fn advance(&mut self) {
        self.current = self.lexer.next_token();
    }

    fn token_to_op(token: &Token) -> Option<char> {
        match token {
            Token::Plus => Some('+'),
            Token::Minus => Some('-'),
            Token::Star => Some('*'),
            Token::Slash => Some('/'),
            Token::Caret => Some('^'),
            _ => None,
        }
    }

    /// Main Pratt parsing function
    pub fn parse_expr(&mut self, min_bp: u8) -> Result<Expr, String> {
        // Handle prefix (atoms and prefix operators)
        let mut lhs = match &self.current.clone() {
            Token::Number(n) => {
                let n = *n;
                self.advance();
                Expr::Number(n)
            }
            Token::Ident(s) => {
                let s = s.clone();
                self.advance();
                Expr::Ident(s)
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr(0)?;
                if self.current != Token::RParen {
                    return Err("Expected ')'".to_string());
                }
                self.advance();
                expr
            }
            Token::Plus | Token::Minus => {
                let op = Self::token_to_op(&self.current).unwrap();
                self.advance();
                let bp = prefix_binding_power(op).unwrap();
                let rhs = self.parse_expr(bp)?;
                Expr::Prefix {
                    op,
                    expr: Box::new(rhs),
                }
            }
            _ => return Err(format!("Unexpected token: {:?}", self.current)),
        };

        // Handle infix operators
        loop {
            let op = match Self::token_to_op(&self.current) {
                Some(op) => op,
                None => break,
            };

            let (l_bp, r_bp) = match infix_binding_power(op) {
                Some(bp) => bp,
                None => break,
            };

            if l_bp < min_bp {
                break;
            }

            self.advance();
            let rhs = self.parse_expr(r_bp)?;
            lhs = Expr::Infix {
                op,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }

        Ok(lhs)
    }
}

/// Evaluate expression
pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Ident(_) => 0.0, // Variables not supported
        Expr::Prefix { op, expr } => {
            let v = eval(expr);
            match op {
                '-' => -v,
                '+' => v,
                _ => v,
            }
        }
        Expr::Infix { op, left, right } => {
            let l = eval(left);
            let r = eval(right);
            match op {
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => l / r,
                '^' => l.powf(r),
                _ => 0.0,
            }
        }
    }
}

pub fn calculate(input: &str) -> Result<f64, String> {
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr(0)?;
    Ok(eval(&expr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precedence() {
        assert_eq!(calculate("1 + 2 * 3").unwrap(), 7.0);
        assert_eq!(calculate("2 * 3 + 1").unwrap(), 7.0);
    }

    #[test]
    fn test_power_right_assoc() {
        assert_eq!(calculate("2 ^ 3 ^ 2").unwrap(), 512.0); // 2^(3^2) = 2^9
    }

    #[test]
    fn test_unary() {
        assert_eq!(calculate("-5").unwrap(), -5.0);
        assert_eq!(calculate("--5").unwrap(), 5.0);
    }

    #[test]
    fn test_parens() {
        assert_eq!(calculate("(1 + 2) * 3").unwrap(), 9.0);
    }

    #[test]
    fn test_complex() {
        assert_eq!(calculate("2 ^ 2 + 3 * 4").unwrap(), 16.0);
    }
}
