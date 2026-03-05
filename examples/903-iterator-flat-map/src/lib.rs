//! 259. Flattening with flat_map()
//!
//! `flat_map(f)` = `map(f).flatten()` — the iterator monad's bind operation.
//! Map each element to zero-or-more outputs, collecting into a single flat sequence.

/// Split each sentence into words, producing a flat list of all words.
pub fn words_from_sentences<'a>(sentences: &[&'a str]) -> Vec<&'a str> {
    sentences
        .iter()
        .flat_map(|s| s.split_whitespace())
        .collect()
}

/// Expand each number n into the range 0..n, then flatten all ranges.
pub fn expand_ranges(nums: &[i32]) -> Vec<i32> {
    nums.iter().flat_map(|&n| 0..n).collect()
}

/// Parse strings into integers, silently dropping failures (zero-output case).
pub fn parse_valid(strings: &[&str]) -> Vec<i32> {
    strings.iter().flat_map(|s| s.parse::<i32>()).collect()
}

/// Extract all bytes from a slice of strings into a single flat byte sequence.
pub fn bytes_from_words(words: &[&str]) -> Vec<u8> {
    words.iter().flat_map(|w| w.bytes()).collect()
}

/// Double only successfully-parsed integers, dropping non-numeric strings.
pub fn parse_and_double(strings: &[&str]) -> Vec<i32> {
    strings
        .iter()
        .flat_map(|s| s.parse::<i32>().map(|n| n * 2))
        .collect()
}

/// Recursive equivalent: concat_map over a list, mirroring OCaml's List.concat_map.
pub fn concat_map<A, B, F>(items: &[A], f: F) -> Vec<B>
where
    F: Fn(&A) -> Vec<B>,
{
    items.iter().flat_map(f).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_words_from_sentences_basic() {
        let sentences = ["hello world", "foo bar"];
        assert_eq!(
            words_from_sentences(&sentences),
            vec!["hello", "world", "foo", "bar"]
        );
    }

    #[test]
    fn test_words_from_sentences_empty() {
        assert_eq!(words_from_sentences(&[]), Vec::<&str>::new());
    }

    #[test]
    fn test_expand_ranges() {
        // 0..1 = [0], 0..2 = [0,1], 0..3 = [0,1,2]
        assert_eq!(expand_ranges(&[1, 2, 3]), vec![0, 0, 1, 0, 1, 2]);
    }

    #[test]
    fn test_expand_ranges_with_zero() {
        // n=0 contributes nothing — zero-output case
        assert_eq!(expand_ranges(&[0, 2, 0]), vec![0, 1]);
    }

    #[test]
    fn test_parse_valid_filters_failures() {
        let strings = ["1", "two", "3", "four", "5"];
        assert_eq!(parse_valid(&strings), vec![1, 3, 5]);
    }

    #[test]
    fn test_parse_valid_all_fail() {
        let strings = ["abc", "def"];
        assert_eq!(parse_valid(&strings), Vec::<i32>::new());
    }

    #[test]
    fn test_bytes_from_words() {
        let words = ["hi", "ok"];
        let result = bytes_from_words(&words);
        assert_eq!(result, b"hiok");
    }

    #[test]
    fn test_parse_and_double() {
        let strings = ["1", "two", "3", "four", "5"];
        assert_eq!(parse_and_double(&strings), vec![2, 6, 10]);
    }

    #[test]
    fn test_concat_map_mirrors_ocaml() {
        // List.concat_map (fun n -> List.init n Fun.id) [1;2;3]
        let result = concat_map(&[1i32, 2, 3], |&n| (0..n).collect());
        assert_eq!(result, vec![0, 0, 1, 0, 1, 2]);
    }
}
