//! # Manacher's Algorithm
//! Linear time longest palindromic substring

pub fn manacher(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 { return String::new(); }
    
    let mut t = vec!['#'];
    for c in &chars { t.push(*c); t.push('#'); }
    
    let mut p = vec![0; t.len()];
    let (mut c, mut r) = (0, 0);
    
    for i in 0..t.len() {
        if i < r { p[i] = (r - i).min(p[2 * c - i]); }
        while i > p[i] && i + p[i] + 1 < t.len() && t[i - p[i] - 1] == t[i + p[i] + 1] { p[i] += 1; }
        if i + p[i] > r { c = i; r = i + p[i]; }
    }
    
    let (max_len, center) = p.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap();
    let start = (center - max_len) / 2;
    chars[start..start + max_len].iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_manacher() { assert_eq!(manacher("babad").len(), 3); }
}
