// Example 161: Digit Parser
// Parse digits: single digit, multi-digit integer, positive/negative

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

fn satisfy<'a, F>(pred: F, desc: &str) -> Parser<'a, char>
where
    F: Fn(char) -> bool + 'a,
{
    let desc = desc.to_string();
    Box::new(move |input: &'a str| match input.chars().next() {
        Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),
        _ => Err(format!("Expected {}", desc)),
    })
}

fn many1<'a, T: 'a>(p: Parser<'a, T>) -> Parser<'a, Vec<T>> {
    Box::new(move |input: &'a str| {
        let (first, mut rem) = p(input)?;
        let mut v = vec![first];
        while let Ok((val, r)) = p(rem) {
            v.push(val);
            rem = r;
        }
        Ok((v, rem))
    })
}

fn map<'a, A: 'a, B: 'a, F>(p: Parser<'a, A>, f: F) -> Parser<'a, B>
where
    F: Fn(A) -> B + 'a,
{
    Box::new(move |input: &'a str| {
        let (v, r) = p(input)?;
        Ok((f(v), r))
    })
}

fn opt<'a, T: 'a>(p: Parser<'a, T>) -> Parser<'a, Option<T>> {
    Box::new(move |input: &'a str| match p(input) {
        Ok((v, r)) => Ok((Some(v), r)),
        Err(_) => Ok((None, input)),
    })
}

// ============================================================
// Approach 1: Single digit → u32
// ============================================================

fn digit<'a>() -> Parser<'a, u32> {
    map(satisfy(|c| c.is_ascii_digit(), "digit"), |c| {
        c as u32 - '0' as u32
    })
}

// ============================================================
// Approach 2: Natural number (unsigned) → u64
// ============================================================

fn natural<'a>() -> Parser<'a, u64> {
    map(many1(satisfy(|c| c.is_ascii_digit(), "digit")), |digits| {
        digits
            .iter()
            .fold(0u64, |acc, &d| acc * 10 + (d as u64 - '0' as u64))
    })
}

// ============================================================
// Approach 3: Signed integer → i64
// ============================================================

fn integer<'a>() -> Parser<'a, i64> {
    Box::new(|input: &'a str| {
        let (sign, rest) = opt(satisfy(|c| c == '+' || c == '-', "sign"))(input)?;
        let (n, rem) = natural()(rest)?;
        let value = match sign {
            Some('-') => -(n as i64),
            _ => n as i64,
        };
        Ok((value, rem))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit() {
        assert_eq!(digit()("5rest"), Ok((5, "rest")));
    }

    #[test]
    fn test_digit_zero() {
        assert_eq!(digit()("0x"), Ok((0, "x")));
    }

    #[test]
    fn test_digit_fail() {
        assert!(digit()("abc").is_err());
    }

    #[test]
    fn test_natural() {
        assert_eq!(natural()("42rest"), Ok((42, "rest")));
    }

    #[test]
    fn test_natural_zero() {
        assert_eq!(natural()("0"), Ok((0, "")));
    }

    #[test]
    fn test_natural_large() {
        assert_eq!(natural()("123456"), Ok((123456, "")));
    }

    #[test]
    fn test_integer_positive() {
        assert_eq!(integer()("42"), Ok((42, "")));
    }

    #[test]
    fn test_integer_negative() {
        assert_eq!(integer()("-42"), Ok((-42, "")));
    }

    #[test]
    fn test_integer_plus() {
        assert_eq!(integer()("+42"), Ok((42, "")));
    }

    #[test]
    fn test_integer_zero() {
        assert_eq!(integer()("0"), Ok((0, "")));
    }

    #[test]
    fn test_integer_fail() {
        assert!(integer()("abc").is_err());
    }
}
