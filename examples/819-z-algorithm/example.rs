/// Z-Algorithm: linear-time pattern matching
///
/// Z[i] = length of the longest substring starting at s[i] that is
/// also a prefix of s. Sentinel '$' keeps Z-values bounded at the boundary.

fn z_array(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut z = vec![0usize; n];
    z[0] = n;
    let (mut l, mut r) = (0usize, 0usize);
    for i in 1..n {
        if i < r {
            z[i] = z[i - l].min(r - i);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }
    z
}

/// Returns 0-based start positions of `pattern` in `text`.
fn z_search(pattern: &str, text: &str) -> Vec<usize> {
    let m = pattern.len();
    // Sentinel must not appear in pattern or text for correctness
    let combined: Vec<u8> = pattern
        .bytes()
        .chain(std::iter::once(b'$'))
        .chain(text.bytes())
        .collect();
    let z = z_array(&combined);
    z.iter()
        .enumerate()
        .skip(m + 1)
        .filter_map(|(i, &zi)| if zi == m { Some(i - m - 1) } else { None })
        .collect()
}

fn main() {
    let text = "aabxaabxaab";
    let pattern = "aab";
    let positions = z_search(pattern, text);
    println!("Pattern '{}' in '{}': {:?}", pattern, text, positions);

    // Show the Z-array itself
    let s = b"aabxaa";
    let z = z_array(s);
    println!("Z-array of 'aabxaa': {:?}", z);
    // Expected: [6, 1, 0, 0, 2, 1]

    let positions2 = z_search("ab", "ababab");
    println!("'ab' in 'ababab': {:?}", positions2);
    // Expected: [0, 2, 4]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_array_basic() {
        // z[0] always equals len; z[1] = length of match with prefix
        let z = z_array(b"aabxaa");
        assert_eq!(z[0], 6);
        assert_eq!(z[1], 1); // "a" matches prefix "a"
        assert_eq!(z[2], 0); // "b" doesn't match "a"
        assert_eq!(z[4], 2); // "aa" matches prefix "aa"
    }

    #[test]
    fn test_z_array_all_same() {
        let z = z_array(b"aaaa");
        // z[0]=4, z[1]=3, z[2]=2, z[3]=1
        assert_eq!(z, vec![4, 3, 2, 1]);
    }

    #[test]
    fn test_search_multiple() {
        assert_eq!(z_search("aab", "aabxaabxaab"), vec![0, 4, 8]);
    }

    #[test]
    fn test_search_overlapping() {
        // Pattern "aa" in "aaaa": positions 0, 1, 2
        assert_eq!(z_search("aa", "aaaa"), vec![0, 1, 2]);
    }

    #[test]
    fn test_search_not_found() {
        assert_eq!(z_search("xyz", "abcdef"), vec![]);
    }

    #[test]
    fn test_search_full_match() {
        assert_eq!(z_search("abc", "abc"), vec![0]);
    }

    #[test]
    fn test_search_at_end() {
        assert_eq!(z_search("end", "the end"), vec![4]);
    }

    #[test]
    fn test_repeated_pattern() {
        assert_eq!(z_search("ab", "ababab"), vec![0, 2, 4]);
    }
}
