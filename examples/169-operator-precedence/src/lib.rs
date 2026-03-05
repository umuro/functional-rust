// Example 169: Operator Precedence
// Binary operators with left/right associativity and precedence levels

type ParseResult<'a, T> = Result<(T, &'a str), String>;

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Num(f64),
    BinOp(String, Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, Copy)]
enum Assoc { Left, Right }

#[derive(Debug, Clone)]
struct OpInfo {
    symbol: &'static str,
    precedence: u8,
    associativity: Assoc,
}

// ============================================================
// Approach 1: Table-driven operator precedence
// ============================================================

const OPERATORS: &[OpInfo] = &[
    OpInfo { symbol: "||", precedence: 1, associativity: Assoc::Left },
    OpInfo { symbol: "&&", precedence: 2, associativity: Assoc::Left },
    OpInfo { symbol: "==", precedence: 3, associativity: Assoc::Left },
    OpInfo { symbol: "!=", precedence: 3, associativity: Assoc::Left },
    OpInfo { symbol: "<=", precedence: 4, associativity: Assoc::Left },
    OpInfo { symbol: ">=", precedence: 4, associativity: Assoc::Left },
    OpInfo { symbol: "<",  precedence: 4, associativity: Assoc::Left },
    OpInfo { symbol: ">",  precedence: 4, associativity: Assoc::Left },
    OpInfo { symbol: "+",  precedence: 5, associativity: Assoc::Left },
    OpInfo { symbol: "-",  precedence: 5, associativity: Assoc::Left },
    OpInfo { symbol: "*",  precedence: 6, associativity: Assoc::Left },
    OpInfo { symbol: "/",  precedence: 6, associativity: Assoc::Left },
    OpInfo { symbol: "%",  precedence: 6, associativity: Assoc::Left },
    OpInfo { symbol: "^",  precedence: 7, associativity: Assoc::Right },
];

fn find_op(input: &str) -> Option<(&OpInfo, &str)> {
    let s = input.trim_start();
    // Try 2-char operators first
    for op in OPERATORS {
        if op.symbol.len() == 2 && s.starts_with(op.symbol) {
            return Some((op, &s[2..]));
        }
    }
    for op in OPERATORS {
        if op.symbol.len() == 1 && s.starts_with(op.symbol) {
            return Some((op, &s[1..]));
        }
    }
    None
}

fn binding_power(op: &OpInfo) -> (u8, u8) {
    let base = op.precedence * 2;
    match op.associativity {
        Assoc::Left => (base, base + 1),
        Assoc::Right => (base + 1, base),
    }
}

fn parse_number(input: &str) -> ParseResult<Expr> {
    let s = input.trim_start();
    let bytes = s.as_bytes();
    let mut pos = 0;
    while pos < bytes.len() && bytes[pos].is_ascii_digit() { pos += 1; }
    if pos < bytes.len() && bytes[pos] == b'.' {
        pos += 1;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() { pos += 1; }
    }
    if pos == 0 { return Err("Expected number".to_string()); }
    let n: f64 = s[..pos].parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
    Ok((Expr::Num(n), &s[pos..]))
}

// ============================================================
// Approach 2: Pratt parser using the table
// ============================================================

fn pratt_expr(input: &str, min_bp: u8) -> ParseResult<Expr> {
    let s = input.trim_start();
    let (mut lhs, mut rest) = if s.starts_with('(') {
        let (e, r) = pratt_expr(&s[1..], 0)?;
        let r = r.trim_start();
        if r.starts_with(')') { (e, &r[1..]) }
        else { return Err("Expected ')'".to_string()); }
    } else {
        parse_number(s)?
    };

    loop {
        let (op, after_op) = match find_op(rest) {
            Some(r) => r,
            None => break,
        };
        let (lbp, rbp) = binding_power(op);
        if lbp < min_bp { break; }
        let (rhs, r) = pratt_expr(after_op, rbp)?;
        lhs = Expr::BinOp(op.symbol.to_string(), Box::new(lhs), Box::new(rhs));
        rest = r;
    }
    Ok((lhs, rest))
}

// ============================================================
// Approach 3: Precedence climbing
// ============================================================

fn climb_expr(input: &str, min_prec: u8) -> ParseResult<Expr> {
    let s = input.trim_start();
    let (mut lhs, mut rest) = parse_number(s)?;

    loop {
        let (op, after_op) = match find_op(rest) {
            Some(r) => r,
            None => break,
        };
        if op.precedence < min_prec { break; }
        let next_min = match op.associativity {
            Assoc::Left => op.precedence + 1,
            Assoc::Right => op.precedence,
        };
        let (rhs, r) = climb_expr(after_op, next_min)?;
        lhs = Expr::BinOp(op.symbol.to_string(), Box::new(lhs), Box::new(rhs));
        rest = r;
    }
    Ok((lhs, rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_precedence_mul_over_add() {
        let (e, _) = pratt_expr("1 + 2 * 3", 0).unwrap();
        // Should be 1 + (2*3)
        match e {
            Expr::BinOp(ref op, _, ref rhs) => {
                assert_eq!(op, "+");
                assert!(matches!(rhs.as_ref(), Expr::BinOp(ref o, _, _) if o == "*"));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_right_associativity() {
        let (e, _) = pratt_expr("2 ^ 3 ^ 2", 0).unwrap();
        // Should be 2 ^ (3^2)
        match e {
            Expr::BinOp(ref op, _, ref rhs) => {
                assert_eq!(op, "^");
                assert!(matches!(rhs.as_ref(), Expr::BinOp(ref o, _, _) if o == "^"));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_left_associativity() {
        let (e, _) = pratt_expr("1 + 2 + 3", 0).unwrap();
        // Should be (1+2) + 3
        match e {
            Expr::BinOp(ref op, ref lhs, _) => {
                assert_eq!(op, "+");
                assert!(matches!(lhs.as_ref(), Expr::BinOp(ref o, _, _) if o == "+"));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_comparison_ops() {
        let (e, _) = pratt_expr("1 == 2", 0).unwrap();
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "=="));
    }

    #[test]
    fn test_multi_char_op() {
        let (e, _) = pratt_expr("1 <= 2", 0).unwrap();
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "<="));
    }

    #[test]
    fn test_climb_matches_pratt() {
        let (e1, _) = pratt_expr("1 + 2 * 3", 0).unwrap();
        let (e2, _) = climb_expr("1 + 2 * 3", 0).unwrap();
        assert_eq!(e1, e2);
    }

    #[test]
    fn test_parens() {
        let (e, _) = pratt_expr("(1 + 2) * 3", 0).unwrap();
        match e {
            Expr::BinOp(ref op, _, _) => assert_eq!(op, "*"),
            _ => panic!(),
        }
    }
}
