//! # Z Algorithm
//! Linear time pattern preprocessing

pub fn z_function(s: &str) -> Vec<usize> {
    let s = s.as_bytes();
    let n = s.len();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    
    for i in 1..n {
        if i < r { z[i] = (r - i).min(z[i - l]); }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] { z[i] += 1; }
        if i + z[i] > r { l = i; r = i + z[i]; }
    }
    z
}

pub fn z_search(text: &str, pattern: &str) -> Vec<usize> {
    let concat = format!("{}${}", pattern, text);
    let z = z_function(&concat);
    z.iter().enumerate()
        .filter(|(i, &len)| len == pattern.len() && *i > pattern.len())
        .map(|(i, _)| i - pattern.len() - 1)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_z() { assert_eq!(z_search("abcabc", "abc"), vec![0, 3]); }
}
