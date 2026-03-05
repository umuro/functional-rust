//! # Boyer-Moore Algorithm
//! Efficient string search with bad character heuristic

pub fn boyer_moore(text: &str, pattern: &str) -> Vec<usize> {
    let (t, p) = (text.as_bytes(), pattern.as_bytes());
    if p.is_empty() || p.len() > t.len() { return vec![]; }
    
    let bad_char = build_bad_char_table(p);
    let mut result = vec![];
    let mut i = 0;
    
    while i <= t.len() - p.len() {
        let mut j = p.len() as i32 - 1;
        while j >= 0 && p[j as usize] == t[i + j as usize] { j -= 1; }
        if j < 0 { result.push(i); i += 1; }
        else { i += (j - bad_char[t[i + j as usize] as usize]).max(1) as usize; }
    }
    result
}

fn build_bad_char_table(pattern: &[u8]) -> [i32; 256] {
    let mut table = [-1i32; 256];
    for (i, &c) in pattern.iter().enumerate() { table[c as usize] = i as i32; }
    table
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_bm() { assert_eq!(boyer_moore("abcabcabc", "abc"), vec![0, 3, 6]); }
}
