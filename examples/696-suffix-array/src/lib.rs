//! # Suffix Array
//! Suffix array construction O(n log n)

pub fn build_suffix_array(s: &str) -> Vec<usize> {
    let n = s.len();
    let bytes = s.as_bytes();
    let mut sa: Vec<usize> = (0..n).collect();
    sa.sort_by_key(|&i| &bytes[i..]);
    sa
}

pub fn search_suffix_array(text: &str, pattern: &str, sa: &[usize]) -> Option<usize> {
    let (lo, hi) = (0, sa.len());
    let bytes = text.as_bytes();
    let pat = pattern.as_bytes();
    
    let mut left = lo;
    let mut right = hi;
    while left < right {
        let mid = (left + right) / 2;
        if bytes[sa[mid]..].starts_with(pat) { return Some(sa[mid]); }
        if &bytes[sa[mid]..] < pat { left = mid + 1; } else { right = mid; }
    }
    if left < hi && bytes[sa[left]..].starts_with(pat) { Some(sa[left]) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sa() {
        let s = "banana";
        let sa = build_suffix_array(s);
        assert_eq!(search_suffix_array(s, "ana", &sa), Some(1));
    }
}
