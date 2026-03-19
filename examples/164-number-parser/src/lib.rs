// Example 164: Number Parser
// Parse floating point numbers with optional sign and decimal

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

// ============================================================
// Approach 1: Imperative scanner — collect number string
// ============================================================

fn float_string<'a>() -> Parser<'a, &'a str> {
    Box::new(|input: &'a str| {
        let bytes = input.as_bytes();
        let len = bytes.len();
        let mut pos = 0;
        // optional sign
        if pos < len && (bytes[pos] == b'+' || bytes[pos] == b'-') {
            pos += 1;
        }
        let start_digits = pos;
        // integer part
        while pos < len && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
        // decimal part
        if pos < len && bytes[pos] == b'.' {
            pos += 1;
            while pos < len && bytes[pos].is_ascii_digit() {
                pos += 1;
            }
        }
        // exponent
        if pos < len && (bytes[pos] == b'e' || bytes[pos] == b'E') {
            pos += 1;
            if pos < len && (bytes[pos] == b'+' || bytes[pos] == b'-') {
                pos += 1;
            }
            while pos < len && bytes[pos].is_ascii_digit() {
                pos += 1;
            }
        }
        if pos == start_digits && (pos == 0 || bytes[pos - 1] != b'.') {
            return Err("Expected number".to_string());
        }
        if pos == 0 {
            return Err("Expected number".to_string());
        }
        Ok((&input[..pos], &input[pos..]))
    })
}

// ============================================================
// Approach 2: Parse to f64 directly
// ============================================================

fn number<'a>() -> Parser<'a, f64> {
    Box::new(|input: &'a str| {
        let (s, rest) = float_string()(input)?;
        match s.parse::<f64>() {
            Ok(n) => Ok((n, rest)),
            Err(_) => Err(format!("Invalid number: {}", s)),
        }
    })
}

// ============================================================
// Approach 3: Combinator-based (no raw indexing)
// ============================================================

fn number_combinator<'a>() -> Parser<'a, f64> {
    Box::new(|input: &'a str| {
        let mut pos = 0;
        let chars: Vec<char> = input.chars().collect();
        let len = chars.len();

        // optional sign
        if pos < len && (chars[pos] == '+' || chars[pos] == '-') {
            pos += 1;
        }

        let digit_start = pos;
        // integer part
        while pos < len && chars[pos].is_ascii_digit() {
            pos += 1;
        }
        let has_int = pos > digit_start;

        // decimal part
        let mut has_frac = false;
        if pos < len && chars[pos] == '.' {
            pos += 1;
            let frac_start = pos;
            while pos < len && chars[pos].is_ascii_digit() {
                pos += 1;
            }
            has_frac = pos > frac_start;
        }

        if !has_int && !has_frac {
            return Err("Expected number".to_string());
        }

        // exponent
        if pos < len && (chars[pos] == 'e' || chars[pos] == 'E') {
            pos += 1;
            if pos < len && (chars[pos] == '+' || chars[pos] == '-') {
                pos += 1;
            }
            while pos < len && chars[pos].is_ascii_digit() {
                pos += 1;
            }
        }

        let byte_len: usize = chars[..pos].iter().map(|c| c.len_utf8()).sum();
        let num_str = &input[..byte_len];
        match num_str.parse::<f64>() {
            Ok(n) => Ok((n, &input[byte_len..])),
            Err(_) => Err(format!("Invalid number: {}", num_str)),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        assert_eq!(float_string()("42rest"), Ok(("42", "rest")));
    }

    #[test]
    fn test_float() {
        assert_eq!(float_string()("3.14!"), Ok(("3.14", "!")));
    }

    #[test]
    fn test_negative() {
        assert_eq!(float_string()("-2.5x"), Ok(("-2.5", "x")));
    }

    #[test]
    fn test_exponent() {
        assert_eq!(float_string()("1e10"), Ok(("1e10", "")));
    }

    #[test]
    fn test_full_scientific() {
        assert_eq!(float_string()("1.5e-3rest"), Ok(("1.5e-3", "rest")));
    }

    #[test]
    fn test_number_f64() {
        let (n, _) = number()("3.14").unwrap();
        assert!((n - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_number_negative() {
        assert_eq!(number()("-42"), Ok((-42.0, "")));
    }

    #[test]
    fn test_number_combinator() {
        let (n, _) = number_combinator()("3.14").unwrap();
        assert!((n - 3.14).abs() < 1e-10);
    }

    #[test]
    fn test_number_fail() {
        assert!(number()("abc").is_err());
    }

    #[test]
    fn test_leading_dot() {
        let (n, _) = number_combinator()(".5").unwrap();
        assert!((n - 0.5).abs() < 1e-10);
    }
}
