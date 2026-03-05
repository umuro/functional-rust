// 770. Recursive Descent Parser from Scratch
// Grammar: expr → term (('+'|'-') term)*
//          term → factor (('*'|'/') factor)*
//          factor → '(' expr ')' | number

// ── AST ────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    BinOp { op: char, left: Box<Expr>, right: Box<Expr> },
}

impl Expr {
    pub fn eval(&self) -> f64 {
        match self {
            Expr::Num(n) => *n,
            Expr::BinOp { op, left, right } => {
                let l = left.eval();
                let r = right.eval();
                match op {
                    '+' => l + r,
                    '-' => l - r,
                    '*' => l * r,
                    '/' => l / r,
                    _   => panic!("unknown op {op}"),
                }
            }
        }
    }
}

// ── Parser ─────────────────────────────────────────────────────────────────────

#[derive(Debug)]
pub struct ParseError(pub String);

pub struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input: input.as_bytes(), pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).map(|&b| b as char)
    }

    fn advance(&mut self) { self.pos += 1; }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(' ') | Some('\t')) {
            self.advance();
        }
    }

    fn parse_number(&mut self) -> Result<f64, ParseError> {
        self.skip_ws();
        let start = self.pos;
        if self.peek() == Some('-') { self.advance(); }
        while matches!(self.peek(), Some('0'..='9') | Some('.')) {
            self.advance();
        }
        let tok = std::str::from_utf8(&self.input[start..self.pos]).unwrap();
        tok.parse::<f64>().map_err(|_| ParseError(format!("bad number: '{tok}'")))
    }

    fn parse_factor(&mut self) -> Result<Expr, ParseError> {
        self.skip_ws();
        if self.peek() == Some('(') {
            self.advance();
            let e = self.parse_expr()?;
            self.skip_ws();
            if self.peek() != Some(')') {
                return Err(ParseError("expected ')'".into()));
            }
            self.advance();
            Ok(e)
        } else {
            Ok(Expr::Num(self.parse_number()?))
        }
    }

    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_factor()?;
        loop {
            self.skip_ws();
            match self.peek() {
                Some(op @ ('*' | '/')) => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expr::BinOp { op, left: Box::new(left), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    pub fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;
        loop {
            self.skip_ws();
            match self.peek() {
                Some(op @ ('+' | '-')) => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::BinOp { op, left: Box::new(left), right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }
}

pub fn parse(input: &str) -> Result<Expr, ParseError> {
    Parser::new(input).parse_expr()
}

fn main() {
    let tests = [
        ("1 + 2 * 3",        7.0),
        ("(1 + 2) * 3",      9.0),
        ("10 / 2 - 3",       2.0),
        ("2 * (3 + 4) / 2",  7.0),
        ("100",             100.0),
    ];

    for (expr, expected) in tests {
        let ast = parse(expr).expect("parse failed");
        let result = ast.eval();
        println!("{expr:25} = {result:8.1}  (expected {expected:.1}) {}",
            if (result - expected).abs() < 1e-9 { "✓" } else { "✗" });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(s: &str) -> f64 { parse(s).unwrap().eval() }

    #[test]
    fn addition()         { assert_eq!(eval("1 + 2"), 3.0); }
    #[test]
    fn precedence()       { assert_eq!(eval("1 + 2 * 3"), 7.0); }
    #[test]
    fn parens_override()  { assert_eq!(eval("(1 + 2) * 3"), 9.0); }
    #[test]
    fn nested_parens()    { assert_eq!(eval("((2 + 3) * (4 - 1))"), 15.0); }
    #[test]
    fn division()         { assert!((eval("10 / 4") - 2.5).abs() < 1e-9); }
}
