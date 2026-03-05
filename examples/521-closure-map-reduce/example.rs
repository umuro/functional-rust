//! # 521. Map-Reduce with Closures
//! Transforming and aggregating data with iterator map and fold.

use std::collections::HashMap;

/// Generic map-reduce: transform each element, then aggregate
fn map_reduce<T, U, V, M, R>(items: &[T], mapper: M, reducer: R, init: V) -> V
where
    M: Fn(&T) -> U,
    R: Fn(V, U) -> V,
{
    items.iter().map(mapper).fold(init, reducer)
}

/// Word frequency count via map-reduce
fn word_count<'a>(words: &'a [&'a str]) -> HashMap<&'a str, usize> {
    words.iter().fold(HashMap::new(), |mut acc: HashMap<&'a str, usize>, &word| {
        *acc.entry(word).or_insert(0) += 1;
        acc
    })
}

/// Inverted index: word -> set of positions
fn inverted_index(text: &str) -> HashMap<&str, Vec<usize>> {
    text.split_whitespace()
        .enumerate()
        .fold(HashMap::new(), |mut idx, (pos, word)| {
            idx.entry(word).or_insert_with(Vec::new).push(pos);
            idx
        })
}

/// Statistics via a single-pass fold
#[derive(Debug)]
struct Stats {
    count: usize,
    sum: f64,
    min: f64,
    max: f64,
}

impl Stats {
    fn from_iter<I: Iterator<Item = f64>>(iter: I) -> Option<Self> {
        let init: Option<Stats> = None;
        iter.fold(init, |acc, x| {
            Some(match acc {
                None => Stats { count: 1, sum: x, min: x, max: x },
                Some(s) => Stats {
                    count: s.count + 1,
                    sum: s.sum + x,
                    min: s.min.min(x),
                    max: s.max.max(x),
                },
            })
        })
    }

    fn mean(&self) -> f64 { self.sum / self.count as f64 }
}

fn main() {
    let nums: Vec<i32> = (1..=10).collect();

    // Sum of squares
    let sum_sq = map_reduce(&nums, |&x| x * x, |acc, sq| acc + sq, 0);
    println!("sum of squares: {}", sum_sq);

    // Max after transform — single pass
    let max_val: i32 = nums.iter().map(|&x| x * 3 - 7).fold(i32::MIN, i32::max);
    println!("max(3x-7): {}", max_val);

    // Product of evens
    let product: i32 = nums.iter()
        .filter(|&&x| x % 2 == 0)
        .product();
    println!("product of evens: {}", product); // 2*4*6*8*10 = 3840

    // Word frequency
    let words = ["hello", "world", "foo", "bar", "hello", "foo", "foo"];
    let mut freq = word_count(&words);
    let mut entries: Vec<_> = freq.iter().collect();
    entries.sort();
    for (word, count) in entries {
        println!("{}: {}", word, count);
    }

    // Inverted index
    let text = "the cat sat on the mat the cat";
    let idx = inverted_index(text);
    let mut idx_entries: Vec<_> = idx.iter().collect();
    idx_entries.sort();
    println!("\nInverted index:");
    for (word, positions) in idx_entries {
        println!("  {:?} -> {:?}", word, positions);
    }

    // Stats in single pass
    let data: Vec<f64> = [3.0, 1.0, 4.0, 1.0, 5.0, 9.0, 2.0, 6.0].to_vec();
    if let Some(stats) = Stats::from_iter(data.into_iter()) {
        println!("\nStats: count={}, sum={}, min={}, max={}, mean={:.2}",
            stats.count, stats.sum, stats.min, stats.max, stats.mean());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_squares() {
        let v = vec![1, 2, 3];
        let result = map_reduce(&v, |&x| x * x, |a, b| a + b, 0);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_word_count() {
        let words = ["a", "b", "a", "c", "a", "b"];
        let freq = word_count(&words);
        assert_eq!(freq["a"], 3);
        assert_eq!(freq["b"], 2);
        assert_eq!(freq["c"], 1);
    }

    #[test]
    fn test_stats() {
        let s = Stats::from_iter([1.0, 2.0, 3.0, 4.0, 5.0].iter().copied()).unwrap();
        assert_eq!(s.count, 5);
        assert_eq!(s.sum, 15.0);
        assert_eq!(s.min, 1.0);
        assert_eq!(s.max, 5.0);
        assert_eq!(s.mean(), 3.0);
    }

    #[test]
    fn test_empty_stats() {
        let s = Stats::from_iter(std::iter::empty::<f64>());
        assert!(s.is_none());
    }
}
