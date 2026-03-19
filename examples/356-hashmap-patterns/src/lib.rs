//! # HashMap Patterns
//! Common patterns: word count, grouping, frequency analysis.

use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
}

pub fn group_by<T, K, F>(items: Vec<T>, key: F) -> HashMap<K, Vec<T>>
where
    K: Eq + std::hash::Hash,
    F: Fn(&T) -> K,
{
    let mut map: HashMap<K, Vec<T>> = HashMap::new();
    for item in items {
        map.entry(key(&item)).or_default().push(item);
    }
    map
}

pub fn frequency_top_n(map: &HashMap<String, usize>, n: usize) -> Vec<(&str, usize)> {
    let mut pairs: Vec<_> = map.iter().map(|(k, &v)| (k.as_str(), v)).collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    pairs.into_iter().take(n).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_words() {
        let wc = word_count("the cat sat on the mat");
        assert_eq!(wc["the"], 2);
        assert_eq!(wc["cat"], 1);
    }
    #[test]
    fn group_by_parity() {
        let grouped = group_by(
            vec![1, 2, 3, 4, 5],
            |&x| if x % 2 == 0 { "even" } else { "odd" },
        );
        assert_eq!(grouped["even"].len(), 2);
        assert_eq!(grouped["odd"].len(), 3);
    }
    #[test]
    fn top_n_frequency() {
        let wc = word_count("a a a b b c");
        let top = frequency_top_n(&wc, 2);
        assert_eq!(top[0].0, "a");
        assert_eq!(top[1].0, "b");
    }
}
