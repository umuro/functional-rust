#![allow(clippy::all)]
//! # Recursive Descent Parser
//!
//! A simple expression parser using recursive descent.

/// Token types
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Eof,
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
            Some('(') => {
                self.advance();
                Token::LParen
            }
            Some(')') => {
                self.advance();
                Token::RParen
            }
            Some(c) if c.is_ascii_digit() || c == '.' => {
                let start = self.pos;
                while let Some(c) = self.peek_char() {
                    if c.is_ascii_digit() || c == '.' {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let num_str = &self.input[start..self.pos];
                Token::Number(num_str.parse().unwrap_or(0.0))
            }
            Some(c) => panic!("Unexpected character: {}", c),
        }
    }
}

/// AST nodes
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    BinOp {
        op: char,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryMinus(Box<Expr>),
}

/// Recursive descent parser
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

    /// expr = term (('+' | '-') term)*
    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;

        loop {
            match &self.current {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::BinOp {
                        op: '+',
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::BinOp {
                        op: '-',
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// term = factor (('*' | '/') factor)*
    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_factor()?;

        loop {
            match &self.current {
                Token::Star => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expr::BinOp {
                        op: '*',
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expr::BinOp {
                        op: '/',
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// factor = '-' factor | number | '(' expr ')'
    fn parse_factor(&mut self) -> Result<Expr, String> {
        match &self.current {
            Token::Minus => {
                self.advance();
                let expr = self.parse_factor()?;
                Ok(Expr::UnaryMinus(Box::new(expr)))
            }
            Token::Number(n) => {
                let n = *n;
                self.advance();
                Ok(Expr::Number(n))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                if self.current != Token::RParen {
                    return Err("Expected ')'".to_string());
                }
                self.advance();
                Ok(expr)
            }
            _ => Err(format!("Unexpected token: {:?}", self.current)),
        }
    }
}

/// Evaluate an expression
pub fn eval(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::UnaryMinus(e) => -eval(e),
        Expr::BinOp { op, left, right } => {
            let l = eval(left);
            let r = eval(right);
            match op {
                '+' => l + r,
                '-' => l - r,
                '*' => l * r,
                '/' => l / r,
                _ => 0.0,
            }
        }
    }
}

/// Parse and evaluate
pub fn calculate(input: &str) -> Result<f64, String> {
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr()?;
    Ok(eval(&expr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        assert_eq!(calculate("42").unwrap(), 42.0);
    }

    #[test]
    fn test_addition() {
        assert_eq!(calculate("1 + 2").unwrap(), 3.0);
    }

    #[test]
    fn test_precedence() {
        assert_eq!(calculate("1 + 2 * 3").unwrap(), 7.0);
    }

    #[test]
    fn test_parens() {
        assert_eq!(calculate("(1 + 2) * 3").unwrap(), 9.0);
    }

    #[test]
    fn test_unary_minus() {
        assert_eq!(calculate("-5").unwrap(), -5.0);
        assert_eq!(calculate("--5").unwrap(), 5.0);
    }

    #[test]
    fn test_complex() {
        assert_eq!(calculate("(10 - 2) / 4 + 1").unwrap(), 3.0);
    }
}
