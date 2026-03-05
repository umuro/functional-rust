use std::collections::HashMap;

/// Idiomatic Rust: count word frequencies using HashMap and iterator combinators.
/// Words are lowercased and split on whitespace.
pub fn count_words(text: &str) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let w = word.to_lowercase();
        *map.entry(w).or_insert(0) += 1;
    }
    map
}

/// Functional style: fold over words to build the frequency map.
pub fn count_words_fold(text: &str) -> HashMap<String, usize> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), |mut acc, w| {
            *acc.entry(w).or_insert(0) += 1;
            acc
        })
}

/// Return words sorted by frequency (descending), then alphabetically for ties.
pub fn top_words(freq: &HashMap<String, usize>) -> Vec<(&str, usize)> {
    let mut pairs: Vec<(&str, usize)> = freq.iter().map(|(k, &v)| (k.as_str(), v)).collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    pairs
}

fn main() {
    let text = "the cat sat on the mat the cat";
    let freq = count_words(text);
    let top = top_words(&freq);

    println!("Word frequencies for: {:?}", text);
    for (word, count) in &top {
        println!("  {}: {}", word, count);
    }

    println!("\nUsing fold variant (same result):");
    let freq2 = count_words_fold(text);
    println!("  'the' count: {}", freq2["the"]);
    println!("  'cat' count: {}", freq2["cat"]);
}

/* Output:
   Word frequencies for: "the cat sat on the mat the cat"
     the: 3
     cat: 2
     mat: 1
     on: 1
     sat: 1

   Using fold variant (same result):
     'the' count: 3
     'cat' count: 2
*/
