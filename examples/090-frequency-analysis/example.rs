use std::collections::{BTreeMap, HashMap};

/// Solution 1: Idiomatic Rust — HashMap with fold
pub fn frequency(s: &str) -> HashMap<char, usize> {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}

/// Solution 2: BTreeMap — mirrors OCaml's Map.Make(Char), keys stay sorted
pub fn frequency_btree(s: &str) -> BTreeMap<char, usize> {
    s.chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .fold(BTreeMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        })
}

/// Solution 3: Sorted by frequency descending, ties broken alphabetically
pub fn sorted_freq(s: &str) -> Vec<(char, usize)> {
    let mut pairs: Vec<(char, usize)> = frequency(s).into_iter().collect();
    pairs.sort_by(|(c1, n1), (c2, n2)| n2.cmp(n1).then(c1.cmp(c2)));
    pairs
}

fn main() {
    let text = "The quick brown fox jumps over the lazy dog";

    println!("=== Sorted by frequency (HashMap) ===");
    for (c, n) in sorted_freq(text) {
        println!("{}: {} ({})", c, "#".repeat(n), n);
    }

    println!("\n=== Alphabetical order (BTreeMap) ===");
    for (c, n) in frequency_btree(text) {
        println!("{}: {}", c, n);
    }

    println!("\n=== Total distinct letters ===");
    println!("{}", frequency(text).len());
}

/* Output:
   === Sorted by frequency (HashMap) ===
   o: #### (4)
   e: ### (3)
   h: ## (2)
   r: ## (2)
   t: ## (2)
   u: ## (2)
   a: # (1)
   b: # (1)
   c: # (1)
   d: # (1)
   f: # (1)
   g: # (1)
   i: # (1)
   j: # (1)
   k: # (1)
   l: # (1)
   m: # (1)
   n: # (1)
   p: # (1)
   q: # (1)
   s: # (1)
   v: # (1)
   w: # (1)
   x: # (1)
   y: # (1)
   z: # (1)

   === Alphabetical order (BTreeMap) ===
   a: 1
   b: 1
   c: 1
   d: 1
   e: 3
   f: 1
   g: 1
   h: 2
   i: 1
   j: 1
   k: 1
   l: 1
   m: 1
   n: 1
   o: 4
   p: 1
   q: 1
   r: 2
   s: 1
   t: 2
   u: 2
   v: 1
   w: 1
   x: 1
   y: 1
   z: 1

   === Total distinct letters ===
   26
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_string() {
        assert_eq!(frequency(""), HashMap::new());
        assert!(sorted_freq("").is_empty());
    }

    #[test]
    fn test_single_character() {
        let freq = frequency("a");
        assert_eq!(freq[&'a'], 1);
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn test_case_insensitive() {
        let freq = frequency("AaAa");
        assert_eq!(freq[&'a'], 4);
        assert_eq!(freq.len(), 1);
    }

    #[test]
    fn test_non_alpha_ignored() {
        let freq = frequency("a1b2c! a");
        assert_eq!(freq[&'a'], 2);
        assert_eq!(freq[&'b'], 1);
        assert_eq!(freq[&'c'], 1);
        assert_eq!(freq.len(), 3);
    }

    #[test]
    fn test_pangram_all_letters_present() {
        let text = "The quick brown fox jumps over the lazy dog";
        let freq = frequency(text);
        assert_eq!(freq.len(), 26);
        assert_eq!(freq[&'e'], 3);
        assert_eq!(freq[&'o'], 4);
    }

    #[test]
    fn test_btree_sorted_by_key() {
        let freq = frequency_btree("bac");
        let keys: Vec<char> = freq.keys().copied().collect();
        assert_eq!(keys, vec!['a', 'b', 'c']);
    }

    #[test]
    fn test_sorted_freq_descending() {
        let result = sorted_freq("aaabbc");
        assert_eq!(result[0], ('a', 3));
        assert_eq!(result[1], ('b', 2));
        assert_eq!(result[2], ('c', 1));
    }

    #[test]
    fn test_sorted_freq_ties_broken_alphabetically() {
        let result = sorted_freq("aabb");
        assert_eq!(result[0], ('a', 2));
        assert_eq!(result[1], ('b', 2));
    }
}
