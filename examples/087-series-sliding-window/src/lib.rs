// Solution 1: Idiomatic Rust — windows() on byte slices, collect into Strings
pub fn series(n: usize, s: &str) -> Vec<String> {
    if n == 0 {
        return vec![String::new(); s.len() + 1];
    }
    s.as_bytes()
        .windows(n)
        .map(|w| std::str::from_utf8(w).unwrap().to_owned())
        .collect()
}

// Solution 2: Functional/recursive — explicit index accumulation, mirrors OCaml List.init
pub fn series_functional(n: usize, s: &str) -> Vec<String> {
    let len = s.len();
    if n == 0 || n > len {
        if n == 0 {
            return vec![String::new(); len + 1];
        }
        return vec![];
    }
    (0..=len - n).map(|i| s[i..i + n].to_owned()).collect()
}

// Solution 1: Idiomatic — find max digit-product over sliding windows
pub fn largest_product(n: usize, s: &str) -> Result<u64, String> {
    if n == 0 {
        return Ok(1);
    }
    if n > s.len() {
        return Err("span too large".to_string());
    }
    if !s.chars().all(|c| c.is_ascii_digit()) {
        return Err("invalid character".to_string());
    }
    let max = series(n, s)
        .into_iter()
        .map(|sub| sub.chars().map(|c| c as u64 - '0' as u64).product::<u64>())
        .max()
        .unwrap_or(0);
    Ok(max)
}

// Solution 2: Recursive — process windows one at a time without collecting
pub fn largest_product_recursive(n: usize, s: &str) -> Result<u64, String> {
    if n == 0 {
        return Ok(1);
    }
    if n > s.len() {
        return Err("span too large".to_string());
    }
    fn digit_product(s: &str) -> u64 {
        s.chars().map(|c| c as u64 - '0' as u64).product()
    }
    fn go(n: usize, s: &str, best: u64) -> u64 {
        if s.len() < n {
            best
        } else {
            let p = digit_product(&s[..n]);
            go(n, &s[1..], best.max(p))
        }
    }
    Ok(go(n, s, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- series ---

    #[test]
    fn test_series_empty_string() {
        assert_eq!(series(1, ""), Vec::<String>::new());
    }

    #[test]
    fn test_series_single_char() {
        assert_eq!(series(1, "4"), vec!["4"]);
    }

    #[test]
    fn test_series_full_span() {
        assert_eq!(series(3, "49142"), vec!["491", "914", "142"]);
    }

    #[test]
    fn test_series_span_larger_than_string() {
        assert_eq!(series(6, "49142"), Vec::<String>::new());
    }

    #[test]
    fn test_series_functional_matches_idiomatic() {
        let s = "0123456789";
        for n in 1..=s.len() {
            assert_eq!(series(n, s), series_functional(n, s));
        }
    }

    // --- largest_product ---

    #[test]
    fn test_largest_product_zero_span() {
        assert_eq!(largest_product(0, "12345"), Ok(1));
    }

    #[test]
    fn test_largest_product_single_digit_span() {
        assert_eq!(largest_product(1, "9876"), Ok(9));
    }

    #[test]
    fn test_largest_product_typical() {
        // "0123456789": best 2-window is "89" = 72
        assert_eq!(largest_product(2, "0123456789"), Ok(72));
    }

    #[test]
    fn test_largest_product_span_too_large() {
        assert_eq!(
            largest_product(10, "12345"),
            Err("span too large".to_string())
        );
    }

    #[test]
    fn test_largest_product_contains_zero() {
        // "900" → windows of 2: "90","00" → max = 0
        assert_eq!(largest_product(2, "900"), Ok(0));
    }

    #[test]
    fn test_largest_product_recursive_matches() {
        let cases = [(2, "0123456789"), (3, "49142"), (1, "9999")];
        for (n, s) in cases {
            assert_eq!(largest_product(n, s), largest_product_recursive(n, s));
        }
    }
}
