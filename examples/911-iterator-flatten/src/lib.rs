//! 272. One-level flattening with flatten()
//!
//! `flatten()` removes exactly one level of iterator nesting.
//! It is the monadic `join` — if `flat_map` is `bind`, `flatten` is `join`.

/// Flatten a `Vec<Vec<T>>` into a `Vec<T>` — one level of nesting removed.
pub fn flatten_vecs<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

/// Flatten an iterator of `Option<T>` — keeps only `Some` values.
/// Each `Option` is itself iterable (yields 0 or 1 items).
pub fn flatten_options<T>(opts: impl IntoIterator<Item = Option<T>>) -> Vec<T> {
    opts.into_iter().flatten().collect()
}

/// Flatten `Option<Option<T>>` → `Option<T>`.
/// The inner `None` or outer `None` both produce `None`.
pub fn flatten_option_option<T>(opt: Option<Option<T>>) -> Option<T> {
    opt.flatten()
}

/// Flatten character-level: split words into their constituent chars.
pub fn words_to_chars(words: &[&str]) -> Vec<char> {
    words.iter().flat_map(|w| w.chars()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flatten_vecs_basic() {
        let nested = vec![vec![1i32, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(flatten_vecs(nested), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_flatten_vecs_empty_inner() {
        let nested: Vec<Vec<i32>> = vec![vec![], vec![1, 2], vec![], vec![3]];
        assert_eq!(flatten_vecs(nested), vec![1, 2, 3]);
    }

    #[test]
    fn test_flatten_vecs_all_empty() {
        let nested: Vec<Vec<i32>> = vec![vec![], vec![]];
        assert_eq!(flatten_vecs(nested), vec![]);
    }

    #[test]
    fn test_flatten_options_filters_none() {
        let opts = vec![Some(1), None, Some(3), None, Some(5)];
        assert_eq!(flatten_options(opts), vec![1, 3, 5]);
    }

    #[test]
    fn test_flatten_options_all_none() {
        let opts: Vec<Option<i32>> = vec![None, None];
        assert_eq!(flatten_options(opts), vec![]);
    }

    #[test]
    fn test_flatten_options_all_some() {
        let opts = vec![Some(10), Some(20), Some(30)];
        assert_eq!(flatten_options(opts), vec![10, 20, 30]);
    }

    #[test]
    fn test_flatten_option_option_some_some() {
        assert_eq!(flatten_option_option(Some(Some(42i32))), Some(42));
    }

    #[test]
    fn test_flatten_option_option_some_none() {
        assert_eq!(flatten_option_option(Some(None::<i32>)), None);
    }

    #[test]
    fn test_flatten_option_option_none() {
        assert_eq!(flatten_option_option(None::<Option<i32>>), None);
    }

    #[test]
    fn test_words_to_chars() {
        let words = vec!["hi", "yo"];
        assert_eq!(words_to_chars(&words), vec!['h', 'i', 'y', 'o']);
    }

    #[test]
    fn test_flatten_one_level_only() {
        // flatten removes exactly ONE level — a Vec<Vec<Vec<T>>> becomes Vec<Vec<T>>
        let triple: Vec<Vec<Vec<i32>>> = vec![vec![vec![1, 2], vec![3]], vec![vec![4]]];
        let one_less: Vec<Vec<i32>> = triple.into_iter().flatten().collect();
        assert_eq!(one_less, vec![vec![1, 2], vec![3], vec![4]]);
    }
}
