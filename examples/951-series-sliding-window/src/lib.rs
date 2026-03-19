pub fn series(n: usize, s: &str) -> Vec<String> {
    if n == 0 {
        return vec![String::new(); s.len() + 1];
    }
    s.as_bytes()
        .windows(n)
        .map(|w| std::str::from_utf8(w).unwrap().to_owned())
        .collect()
}

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

/* Output:
   series(3, "49142") = ["491", "914", "142"]
   series_functional(3, "49142") = ["491", "914", "142"]
   largest_product(2, "0123456789") = Ok(72)
   largest_product(6, "49142") = Err("span too large")
   largest_product_recursive(2, "0123456789") = Ok(72)
*/
