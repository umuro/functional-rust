/// Decode Run-Length Encoding (99 Problems #12)
///
/// Given a modified RLE list, reconstruct the original list by expanding
/// `Many(n, x)` into n copies of x and keeping `One(x)` as-is.

#[derive(Debug, PartialEq, Clone)]
pub enum RleItem<T> {
    One(T),
    Many(usize, T),
}

// ── Idiomatic Rust: flat_map with repeat ────────────────────────────────────

pub fn decode<T: Clone>(encoded: &[RleItem<T>]) -> Vec<T> {
    encoded.iter().flat_map(|item| match item {
        RleItem::One(x) => vec![x.clone()],
        RleItem::Many(n, x) => vec![x.clone(); *n],
    }).collect()
}

// ── Iterator-based with std::iter::repeat ───────────────────────────────────

pub fn decode_iter<T: Clone>(encoded: &[RleItem<T>]) -> Vec<T> {
    encoded.iter().flat_map(|item| {
        let (count, value) = match item {
            RleItem::One(x) => (1, x),
            RleItem::Many(n, x) => (*n, x),
        };
        std::iter::repeat(value.clone()).take(count)
    }).collect()
}

// ── Recursive style ─────────────────────────────────────────────────────────

pub fn decode_recursive<T: Clone>(encoded: &[RleItem<T>]) -> Vec<T> {
    fn expand<T: Clone>(item: &RleItem<T>) -> Vec<T> {
        match item {
            RleItem::One(x) => vec![x.clone()],
            RleItem::Many(n, x) => {
                // Recursive expansion (functional style)
                if *n == 0 { vec![] }
                else {
                    let mut rest = expand(&RleItem::Many(n - 1, x.clone()));
                    rest.insert(0, x.clone());
                    rest
                }
            }
        }
    }

    match encoded.split_first() {
        None => vec![],
        Some((head, tail)) => {
            let mut result = expand(head);
            result.extend(decode_recursive(tail));
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use RleItem::*;

    #[test]
    fn test_empty() {
        assert_eq!(decode::<char>(&[]), vec![]);
    }

    #[test]
    fn test_all_singles() {
        let encoded = vec![One('a'), One('b'), One('c')];
        assert_eq!(decode(&encoded), vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_all_runs() {
        let encoded = vec![Many(3, 'x'), Many(2, 'y')];
        assert_eq!(decode(&encoded), vec!['x', 'x', 'x', 'y', 'y']);
    }

    #[test]
    fn test_mixed() {
        let encoded = vec![Many(3, 'a'), One('b'), Many(2, 'c'), Many(4, 'd')];
        let expected = vec!['a','a','a','b','c','c','d','d','d','d'];
        assert_eq!(decode(&encoded), expected);
        assert_eq!(decode_iter(&encoded), expected);
        assert_eq!(decode_recursive(&encoded), expected);
    }

    #[test]
    fn test_single_item() {
        assert_eq!(decode(&[One(42)]), vec![42]);
        assert_eq!(decode(&[Many(1, 42)]), vec![42]);
    }

    #[test]
    fn test_roundtrip_property() {
        // Encode then decode should give back the original (for valid inputs)
        let original = vec![1, 1, 1, 2, 3, 3];
        let encoded = vec![Many(3, 1), One(2), Many(2, 3)];
        assert_eq!(decode(&encoded), original);
    }
}
