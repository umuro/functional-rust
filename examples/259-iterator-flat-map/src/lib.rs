//! 259. Flattening with flat_map()
//!
//! `flat_map(f)` = `map(f).flatten()` — the iterator monad's bind operation.

#[cfg(test)]
mod tests {
    #[test]
    fn test_flat_map_expand() {
        let result: Vec<i32> = [1i32, 2, 3].iter().flat_map(|&n| 0..n).collect();
        assert_eq!(result, vec![0, 0, 1, 0, 1, 2]);
    }

    #[test]
    fn test_flat_map_filter_parse() {
        let strings = ["1", "x", "2", "y", "3"];
        let result: Vec<i32> = strings.iter()
            .flat_map(|s| s.parse::<i32>())
            .collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_flat_map_words() {
        let sentences = ["hello world", "foo bar"];
        let words: Vec<&str> = sentences.iter()
            .flat_map(|s| s.split_whitespace())
            .collect();
        assert_eq!(words.len(), 4);
    }
}
