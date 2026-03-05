// 771. Pratt Parser for Operator Precedence Expressions
// Handles: (1+2)*3, unary minus, right-assoc ^

// ── Token ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Plus, Minus, Star, Slash, Caret,
    LParen, RParen,
    Eof,
}

// ── AST ────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Unary  { op: char, operand: Box<Expr> },
    Binary { op: char, left: Box<Expr>, right: Box<Expr> },
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(n) => *n,
            Expr::Unary { op: '-', operand } => -operand.eval(),
            Expr::Unary { operand, .. } => operand.eval(),
            Expr::Binary { op, left, right } => {
                let (l, r) = (left.eval(), right.eval());
                match op {
                    '+' => l + r, '-' => l - r,
                    '*' => l * r, '/' => l / r,
                    '^' => l.powf(r),
                    _ => panic!("unknown op"),
                }
            }
        }
    }
}

// ── Lexer ──────────────────────────────────────────────────────────────────────

pub fn tokenize(s: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' => { i += 1; }
            '+' => { tokens.push(Token::Plus);   i += 1; }
            '-' => { tokens.push(Token::Minus);  i += 1; }
            '*' => { tokens.push(Token::Star);   i += 1; }
            '/' => { tokens.push(Token::Slash);  i += 1; }
            '^' => { tokens.push(Token::Caret);  i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            c if c.is_ascii_digit() || c == '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let num: f64 = chars[start..i].iter().collect::<String>().parse().unwrap();
                tokens.push(Token::Num(num));
            }
            _ => { i += 1; }
        }
    }
    tokens.push(Token::Eof);
    tokens
}

// ── Pratt parser ───────────────────────────────────────────────────────────────

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug)]
pub struct ParseError(pub String);

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn consume(&mut self) -> Token {
        let t = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        t
    }

    /// Returns (left_bp, right_bp) for infix operators
    fn infix_bp(tok: &Token) -> Option<(u8, u8)> {
        match tok {
            Token::Plus  | Token::Minus => Some((10, 11)),
            Token::Star  | Token::Slash => Some((20, 21)),
            Token::Caret                => Some((30, 29)), // right-assoc
            _ => None,
        }
    }

    fn op_char(tok: &Token) -> char {
        match tok {
            Token::Plus  => '+', Token::Minus => '-',
            Token::Star  => '*', Token::Slash => '/',
            Token::Caret => '^',
            _ => '?',
        }
    }

    fn parse_nud(&mut self) -> Result<Expr, ParseError> {
        match self.consume() {
            Token::Num(n) => Ok(Expr::Num(n)),
            Token::Minus => {
                let operand = self.parse_bp(25)?;
                Ok(Expr::Unary { op: '-', operand: Box::new(operand) })
            }
            Token::LParen => {
                let e = self.parse_bp(0)?;
                if self.consume() != Token::RParen {
                    return Err(ParseError("expected ')'".into()));
                }
                Ok(e)
            }
            t => Err(ParseError(format!("unexpected token: {t:?}"))),
        }
    }

    pub fn parse_bp(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        let mut left = self.parse_nud()?;
        loop {
            let tok = self.peek().clone();
            match Self::infix_bp(&tok) {
                Some((lbp, rbp)) if lbp > min_bp => {
                    self.consume();
                    let right = self.parse_bp(rbp)?;
                    let op = Self::op_char(&tok);
                    left = Expr::Binary { op, left: Box::new(left), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }
}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    Parser::new(tokenize(input)).parse_bp(0)
}

fn main() {
    let tests: &[(&str, f64)] = &[
        ("(1 + 2) * 3",        9.0),
        ("2 ^ 3 ^ 2",        512.0),  // right-assoc: 2^(3^2) = 2^9
        ("-2 * 3",            -6.0),
        ("1 + 2 * 3 - 4 / 2",  5.0),
        ("(10 - 3) / (2 + 5)", 1.0),
    ];

    for (expr, expected) in tests {
        let result = parse(expr).unwrap().eval();
        let ok = if (result - expected).abs() < 1e-9 { "✓" } else { "✗" };
        println!("{expr:30} = {result:8.2}  (expected {expected:.2}) {ok}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(s: &str) -> f64 { parse(s).unwrap().eval() }

    #[test]
    fn basic_ops()          { assert_eq!(ev("1 + 2"), 3.0); }
    #[test]
    fn precedence()         { assert_eq!(ev("1 + 2 * 3"), 7.0); }
    #[test]
    fn parens_change_prec() { assert_eq!(ev("(1 + 2) * 3"), 9.0); }
    #[test]
    fn right_assoc_power()  { assert!((ev("2 ^ 3 ^ 2") - 512.0).abs() < 1e-9); }
    #[test]
    fn unary_minus()        { assert_eq!(ev("-3 + 5"), 2.0); }
    #[test]
    fn unary_in_parens()    { assert_eq!(ev("(-3) * 2"), -6.0); }
}
