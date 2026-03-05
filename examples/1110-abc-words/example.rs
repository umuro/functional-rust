// ABC Words — idiomatic Rust

/// Idiomatic Rust: stateful iterator — the cursor advances past each matched
/// letter, so each `.any()` call searches only the remaining suffix.
pub fn is_abc_word(word: &str) -> bool {
    let mut chars = word.chars();
    chars.any(|c| c == 'a') && chars.any(|c| c == 'b') && chars.any(|c| c == 'c')
}

/// Fold-based: accumulate progress through the target sequence without mutation.
pub fn is_abc_word_fold(word: &str) -> bool {
    const TARGET: [char; 3] = ['a', 'b', 'c'];
    word.chars().fold(0usize, |idx, ch| {
        if idx < TARGET.len() && ch == TARGET[idx] {
            idx + 1
        } else {
            idx
        }
    }) == TARGET.len()
}

/// Recursive: mirrors the OCaml pattern-match style explicitly.
pub fn is_abc_word_recursive(word: &str) -> bool {
    fn find_seq(chars: &[char], seq: &[char]) -> bool {
        match (chars, seq) {
            (_, []) => true,
            ([], _) => false,
            ([ch, rest @ ..], [target, remaining @ ..]) if ch == target => {
                find_seq(rest, remaining)
            }
            ([_, rest @ ..], _) => find_seq(rest, seq),
        }
    }

    let chars: Vec<char> = word.chars().collect();
    find_seq(&chars, &['a', 'b', 'c'])
}

/// Filter a slice of words, returning those that are ABC words.
pub fn filter_abc_words<'a>(words: &[&'a str]) -> Vec<&'a str> {
    words.iter().copied().filter(|w| is_abc_word(w)).collect()
}

fn main() {
    let test_words = [
        "abc", "cab", "abacus", "abstracted", "abba", "xyzabc", "dog", "aababc", "",
    ];

    println!("=== ABC Words ===");
    println!("A word is an 'ABC word' if a, b, c appear in that order as a subsequence.\n");

    for word in &test_words {
        let result = is_abc_word(word);
        let marker = if result { "✓" } else { "✗" };
        println!("  {marker}  {:15} → {result}", format!("{word:?}"));
    }

    println!("\n=== All three implementations agree ===");
    for word in &test_words {
        let a = is_abc_word(word);
        let b = is_abc_word_fold(word);
        let c = is_abc_word_recursive(word);
        assert_eq!(a, b);
        assert_eq!(b, c);
        println!("  {:15} iter={a} fold={b} rec={c}", format!("{word:?}"));
    }

    println!("\n=== filter_abc_words ===");
    let result = filter_abc_words(&test_words);
    println!("  ABC words: {:?}", result);
}

/* Output:
=== ABC Words ===
A word is an 'ABC word' if a, b, c appear in that order as a subsequence.

  ✓  "abc"           → true
  ✗  "cab"           → false
  ✓  "abacus"        → true
  ✓  "abstracted"    → true
  ✗  "abba"          → false
  ✓  "xyzabc"        → true
  ✗  "dog"           → false
  ✓  "aababc"        → true
  ✗  ""              → false

=== All three implementations agree ===
  "abc"           iter=true fold=true rec=true
  "cab"           iter=false fold=false rec=false
  "abacus"        iter=true fold=true rec=true
  "abstracted"    iter=true fold=true rec=true
  "abba"          iter=false fold=false rec=false
  "xyzabc"        iter=true fold=true rec=true
  "dog"           iter=false fold=false rec=false
  "aababc"        iter=true fold=true rec=true
  ""              iter=false fold=false rec=false

=== filter_abc_words ===
  ABC words: ["abc", "abacus", "abstracted", "xyzabc", "aababc"]
*/
