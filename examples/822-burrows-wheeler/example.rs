/// Burrows-Wheeler Transform (BWT)
///
/// Forward: append '$', sort all cyclic rotations, take last column.
/// Inverse: use LF-mapping to recover original string in O(n).

/// Returns (transformed string, index of original rotation in sorted order).
fn bwt(input: &str) -> (String, usize) {
    let mut s = input.to_string();
    s.push('$');
    let bytes = s.as_bytes();
    let n = bytes.len();

    // Sort rotation indices lexicographically
    let mut indices: Vec<usize> = (0..n).collect();
    indices.sort_unstable_by(|&a, &b| {
        for k in 0..n {
            let ca = bytes[(a + k) % n];
            let cb = bytes[(b + k) % n];
            match ca.cmp(&cb) {
                std::cmp::Ordering::Equal => continue,
                other => return other,
            }
        }
        std::cmp::Ordering::Equal
    });

    // Last column: character immediately before each sorted rotation
    let transformed: String = indices
        .iter()
        .map(|&i| bytes[(i + n - 1) % n] as char)
        .collect();

    // Row where the original string (rotation starting at 0) appears
    let original_row = indices.iter().position(|&i| i == 0).unwrap();

    (transformed, original_row)
}

/// Inverse BWT using the LF (last-to-first) mapping.
fn ibwt(bwt_str: &str, original_row: usize) -> String {
    let l: Vec<u8> = bwt_str.bytes().collect();
    let n = l.len();

    // First column F = sorted L
    let mut f = l.clone();
    f.sort_unstable();

    // rank[i] = how many times l[i] occurred before position i in L
    let mut rank = vec![0usize; n];
    let mut count = [0usize; 256];
    for (i, &c) in l.iter().enumerate() {
        rank[i] = count[c as usize];
        count[c as usize] += 1;
    }

    // first_occ[c] = first position of c in F
    let mut first_occ = [0usize; 256];
    let mut seen = [false; 256];
    for (i, &c) in f.iter().enumerate() {
        if !seen[c as usize] {
            first_occ[c as usize] = i;
            seen[c as usize] = true;
        }
    }

    // Follow LF-mapping n-1 times to recover the original + '$'
    let mut result = Vec::with_capacity(n - 1);
    let mut row = original_row;
    for _ in 0..n - 1 {
        let c = l[row];
        result.push(c);
        row = first_occ[c as usize] + rank[row];
    }

    // Reverse and remove trailing '$'
    result.reverse();
    // Strip the '$' which is at the end
    let s = String::from_utf8(result).unwrap();
    s.trim_end_matches('$').to_string()
}

fn main() {
    let words = ["banana", "abracadabra", "mississippi", "hello", "rust"];
    for word in &words {
        let (transformed, row) = bwt(word);
        let recovered = ibwt(&transformed, row);
        println!(
            "bwt({:?}) = {:?} (row={}), recovered = {:?}, ok={}",
            word, transformed, row, recovered, &recovered == word
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(s: &str) -> bool {
        let (t, row) = bwt(s);
        ibwt(&t, row) == s
    }

    #[test]
    fn test_banana() {
        let (t, _) = bwt("banana");
        // Classic result: "annb$aa" — last column of sorted rotations
        assert_eq!(t, "annb$aa");
    }

    #[test]
    fn test_roundtrip_banana() {
        assert!(roundtrip("banana"));
    }

    #[test]
    fn test_roundtrip_abracadabra() {
        assert!(roundtrip("abracadabra"));
    }

    #[test]
    fn test_roundtrip_mississippi() {
        assert!(roundtrip("mississippi"));
    }

    #[test]
    fn test_roundtrip_single() {
        assert!(roundtrip("a"));
    }

    #[test]
    fn test_roundtrip_hello() {
        assert!(roundtrip("hello"));
    }

    #[test]
    fn test_clustering() {
        // BWT of "mississippi" should cluster repeated characters
        let (t, _) = bwt("mississippi");
        // 'i' appears 4 times in original — they should cluster in BWT
        let i_count = t.chars().filter(|&c| c == 'i').count();
        assert_eq!(i_count, 4);
    }
}
