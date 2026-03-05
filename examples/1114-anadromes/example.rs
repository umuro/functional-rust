use std::collections::BTreeSet;

/// Reverse a string by reversing its Unicode characters.
pub fn string_rev(s: &str) -> String {
    s.chars().rev().collect()
}

/// Find all anadrome pairs from a set of words.
///
/// Each pair is emitted once: the lexicographically smaller word comes first.
pub fn get_anadromes(words: &BTreeSet<String>) -> Vec<(String, String)> {
    words
        .iter()
        .filter_map(|s| {
            let r = string_rev(s);
            if s.as_str() < r.as_str() && words.contains(&r) {
                Some((s.clone(), r))
            } else {
                None
            }
        })
        .collect()
}

/// Build a word set from an iterator of strings, filtering by minimum length
/// and normalising to lowercase.
pub fn build_word_set<I>(words: I, min_len: usize) -> BTreeSet<String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    words
        .into_iter()
        .map(|w| w.as_ref().to_lowercase())
        .filter(|w| w.len() > min_len)
        .collect()
}

fn main() {
    let sample = vec![
        "stressed",
        "desserts",
        "repaid",
        "diaper",
        "drawer",
        "reward",
        "wolf",
        "flow",
        "unrelated",
        "hello",
    ];

    // Rosetta Code uses min_len = 6 (words with length > 6)
    let word_set = build_word_set(sample.iter(), 6);
    let pairs = get_anadromes(&word_set);

    println!("Anadrome pairs (min length > 6):");
    for (a, b) in &pairs {
        println!("  {:>12} | {}", a, b);
    }

    println!();
    println!("string_rev(\"stressed\") = {}", string_rev("stressed"));
    println!("string_rev(\"racecar\")  = {}", string_rev("racecar"));
}

/* Output:
   Anadrome pairs (min length > 6):
        desserts | stressed

   string_rev("stressed") = desserts
   string_rev("racecar")  = racecar
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev_empty() {
        assert_eq!(string_rev(""), "");
    }

    #[test]
    fn test_rev_typical_word() {
        assert_eq!(string_rev("stressed"), "desserts");
    }

    #[test]
    fn test_single_anadrome_pair() {
        let set: BTreeSet<String> = ["stressed", "desserts", "unrelated"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let pairs = get_anadromes(&set);
        assert_eq!(pairs.len(), 1);
        assert_eq!(pairs[0], ("desserts".to_string(), "stressed".to_string()));
    }

    #[test]
    fn test_build_word_set_lowercases() {
        let words = ["STRESSED", "Desserts"];
        let set = build_word_set(words.iter(), 6);
        assert!(set.contains("stressed"));
        assert!(set.contains("desserts"));
    }
}
