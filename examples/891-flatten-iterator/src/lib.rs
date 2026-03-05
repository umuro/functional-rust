// Example 097: Flatten Iterator
// Collapse one level of nesting using .flatten() and .flat_map()

// === Approach 1: flatten Vec<Vec<T>> ===

pub fn flatten_vecs(nested: Vec<Vec<i32>>) -> Vec<i32> {
    nested.into_iter().flatten().collect()
}

// === Approach 2: flat_map — map then flatten ===

pub fn words_in_sentences(sentences: &[&str]) -> Vec<String> {
    sentences
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(String::from)
        .collect()
}

pub fn expand_ranges(ranges: &[(i32, i32)]) -> Vec<i32> {
    ranges.iter().flat_map(|&(lo, hi)| lo..=hi).collect()
}

pub fn chars_of_words(words: &[&str]) -> Vec<char> {
    words.iter().flat_map(|w| w.chars()).collect()
}

// === Approach 3: Flatten Option<T> — Some yields one item, None yields none ===

pub fn flatten_options(opts: Vec<Option<i32>>) -> Vec<i32> {
    opts.into_iter().flatten().collect()
}

pub fn parse_ints(strs: &[&str]) -> Vec<i32> {
    strs.iter().filter_map(|s| s.parse::<i32>().ok()).collect()
}

// === Approach 4: Flatten Result<T, E> — Ok yields one item, Err is skipped ===

pub fn collect_ok<T: Clone, E>(results: &[Result<T, E>]) -> Vec<T> {
    results
        .iter()
        .filter_map(|r| r.as_ref().ok().cloned())
        .collect()
}

// === Approach 5: Two levels deep ===

pub fn deep_flatten(nested: Vec<Vec<Vec<i32>>>) -> Vec<i32> {
    nested.into_iter().flatten().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_vecs_typical() {
        let nested = vec![vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];
        assert_eq!(flatten_vecs(nested), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_flatten_vecs_empty_inner() {
        let nested = vec![vec![], vec![1, 2], vec![], vec![3]];
        assert_eq!(flatten_vecs(nested), vec![1, 2, 3]);
    }

    #[test]
    fn test_flatten_vecs_empty_outer() {
        let nested: Vec<Vec<i32>> = vec![];
        assert_eq!(flatten_vecs(nested), vec![]);
    }

    #[test]
    fn test_words_in_sentences() {
        let sentences = ["hello world", "foo bar baz"];
        assert_eq!(
            words_in_sentences(&sentences),
            vec!["hello", "world", "foo", "bar", "baz"]
        );
    }

    #[test]
    fn test_expand_ranges() {
        let ranges = [(1, 3), (7, 9)];
        assert_eq!(expand_ranges(&ranges), vec![1, 2, 3, 7, 8, 9]);
    }

    #[test]
    fn test_expand_ranges_single() {
        assert_eq!(expand_ranges(&[(5, 5)]), vec![5]);
    }

    #[test]
    fn test_chars_of_words() {
        let words = ["hi", "yo"];
        assert_eq!(chars_of_words(&words), vec!['h', 'i', 'y', 'o']);
    }

    #[test]
    fn test_flatten_options_mixed() {
        let opts = vec![Some(1), None, Some(3), None, Some(5)];
        assert_eq!(flatten_options(opts), vec![1, 3, 5]);
    }

    #[test]
    fn test_flatten_options_all_none() {
        let opts: Vec<Option<i32>> = vec![None, None];
        assert_eq!(flatten_options(opts), vec![]);
    }

    #[test]
    fn test_parse_ints() {
        let strs = ["1", "two", "3", "four", "5"];
        assert_eq!(parse_ints(&strs), vec![1, 3, 5]);
    }

    #[test]
    fn test_collect_ok() {
        let results: Vec<Result<i32, &str>> = vec![Ok(1), Err("bad"), Ok(3)];
        assert_eq!(collect_ok(&results), vec![1, 3]);
    }

    #[test]
    fn test_deep_flatten() {
        let nested = vec![vec![vec![1, 2], vec![3]], vec![vec![4, 5, 6]]];
        assert_eq!(deep_flatten(nested), vec![1, 2, 3, 4, 5, 6]);
    }
}
