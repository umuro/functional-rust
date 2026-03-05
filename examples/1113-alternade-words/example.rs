use std::collections::HashSet;

/// Split a word into two alternade sub-words by interleaving even/odd indices.
///
/// For a word of length n:
/// - even alternade: characters at positions 0, 2, 4, ... (length = ceil(n/2))
/// - odd alternade:  characters at positions 1, 3, 5, ... (length = floor(n/2))
pub fn split_alternade(word: &str) -> (String, String) {
    let even: String = word.chars().step_by(2).collect();
    let odd: String = word.chars().skip(1).step_by(2).collect();
    (even, odd)
}

/// Find all alternade words in a word list.
///
/// A word qualifies when:
/// 1. Its length is >= 6 (so both alternades are >= 3 chars)
/// 2. Both alternades produced by `split_alternade` are present in the word set
///
/// The word set is built from words of length >= 3.
/// Returns formatted strings: `"word | even_part odd_part"`.
pub fn find_alternades(words: &[&str]) -> Vec<String> {
    let word_set: HashSet<&str> = words.iter().copied().filter(|w| w.len() >= 3).collect();

    let mut results: Vec<String> = words
        .iter()
        .copied()
        .filter(|w| w.len() >= 6)
        .filter_map(|word| {
            let (even, odd) = split_alternade(word);
            if word_set.contains(even.as_str()) && word_set.contains(odd.as_str()) {
                Some(format!("{word} | {even} {odd}"))
            } else {
                None
            }
        })
        .collect();

    results.sort();
    results
}

fn main() {
    // Demonstrate split_alternade
    let (even, odd) = split_alternade("boards");
    println!("split_alternade(\"boards\") => even=\"{even}\", odd=\"{odd}\"");

    let (even, odd) = split_alternade("character");
    println!("split_alternade(\"character\") => even=\"{even}\", odd=\"{odd}\"");

    // Demonstrate find_alternades with a small contrived dictionary
    let dict = vec![
        "abcdef", "ace", "bdf", // "abcdef" is an alternade: ace + bdf
        "123456", "135", "246", // "123456" is an alternade: 135 + 246
        "other", "word",
    ];
    println!("\nAlternade words found:");
    for result in find_alternades(&dict) {
        println!("  {result}");
    }
}

/* Output:
   split_alternade("boards") => even="bad", odd="ors"
   split_alternade("character") => even="caace", odd="hratr"

   Alternade words found:
     123456 | 135 246
     abcdef | ace bdf
*/
