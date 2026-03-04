/// String Anagram Check — Sorting and Frequency Counting Approaches

/// Sort-based: mirrors OCaml's approach.
pub fn is_anagram_sort(s1: &str, s2: &str) -> bool {
    let normalize = |s: &str| -> String { s.to_lowercase() };
    let sorted_chars = |s: &str| -> Vec<char> {
        let mut chars: Vec<char> = s.to_lowercase().chars().collect();
        chars.sort_unstable();
        chars
    };
    normalize(s1) != normalize(s2) && sorted_chars(s1) == sorted_chars(s2)
}

/// Frequency-based: O(n) using HashMap.
pub fn is_anagram_freq(s1: &str, s2: &str) -> bool {
    use std::collections::HashMap;
    let lower1 = s1.to_lowercase();
    let lower2 = s2.to_lowercase();
    if lower1 == lower2 { return false; }
    let freq = |s: &str| -> HashMap<char, i32> {
        let mut map = HashMap::new();
        for c in s.chars() { *map.entry(c).or_insert(0) += 1; }
        map
    };
    freq(&lower1) == freq(&lower2)
}

pub fn find_anagrams<'a>(word: &str, candidates: &[&'a str]) -> Vec<&'a str> {
    candidates.iter().copied().filter(|c| is_anagram_sort(word, c)).collect()
}

fn main() {
    let results = find_anagrams("listen", &["enlists", "google", "inlets", "silent"]);
    println!("Anagrams of 'listen': {:?}", results);
    println!("is_anagram('listen', 'silent') = {}", is_anagram_sort("listen", "silent"));
    println!("is_anagram('listen', 'listen') = {}", is_anagram_sort("listen", "listen"));
}

/* Output:
   Anagrams of 'listen': ["inlets", "silent"]
   is_anagram('listen', 'silent') = true
   is_anagram('listen', 'listen') = false
*/
