use std::collections::{BTreeMap, HashMap};

// ── Solution 1: Idiomatic Rust ─────────────────────────────────────────────
// Mutable trie backed by HashMap.  Mirrors how a Rust programmer would write
// this: mutate in place, no allocation overhead from structural sharing.

#[derive(Default)]
pub struct Trie {
    is_word: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Trie::default()
    }

    /// Insert `word` into the trie (mutates in place).
    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_word = true;
    }

    /// Returns `true` if `word` was previously inserted.
    pub fn contains(&self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            match node.children.get(&c) {
                None => return false,
                Some(child) => node = child,
            }
        }
        node.is_word
    }

    /// Returns `true` if any inserted word starts with `prefix`.
    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                None => return false,
                Some(child) => node = child,
            }
        }
        true
    }
}

// ── Solution 2: Functional / persistent ───────────────────────────────────
// Mirrors OCaml's `Map.Make(Char)` approach: each insert returns a *new*
// trie value rather than mutating the existing one.  BTreeMap gives ordered
// children, just like OCaml's balanced-tree map.

#[derive(Clone, Default)]
pub struct FunctionalTrie {
    is_word: bool,
    children: BTreeMap<char, FunctionalTrie>,
}

impl FunctionalTrie {
    pub fn empty() -> Self {
        FunctionalTrie::default()
    }

    /// Returns a new trie with `word` added — the original is consumed.
    /// Calling code uses: `trie = trie.insert("word");`  — the OCaml pattern.
    pub fn insert(self, word: &str) -> Self {
        fn go(mut node: FunctionalTrie, chars: &[char]) -> FunctionalTrie {
            match chars {
                [] => FunctionalTrie {
                    is_word: true,
                    ..node
                },
                [c, rest @ ..] => {
                    let child = node.children.remove(c).unwrap_or_default();
                    node.children.insert(*c, go(child, rest));
                    node
                }
            }
        }
        let chars: Vec<char> = word.chars().collect();
        go(self, &chars)
    }

    /// Returns `true` if `word` was previously inserted.
    pub fn contains(&self, word: &str) -> bool {
        fn go(node: &FunctionalTrie, chars: &[char]) -> bool {
            match chars {
                [] => node.is_word,
                [c, rest @ ..] => node.children.get(c).is_some_and(|child| go(child, rest)),
            }
        }
        let chars: Vec<char> = word.chars().collect();
        go(self, &chars)
    }
}

// ── Convenience: build from an iterator ───────────────────────────────────
/// Build a `FunctionalTrie` from an iterator of words — mirrors the OCaml
/// `List.fold_left (fun t w -> insert w t) empty words` idiom.
pub fn build_trie<'a>(words: impl IntoIterator<Item = &'a str>) -> FunctionalTrie {
    words
        .into_iter()
        .fold(FunctionalTrie::empty(), |t, w| t.insert(w))
}

// ── Tests ──────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    // ── Mutable Trie ──────────────────────────────────────────────────────

    #[test]
    fn test_empty_trie_contains_nothing() {
        let t = Trie::new();
        assert!(!t.contains(""));
        assert!(!t.contains("a"));
        assert!(!t.contains("hello"));
    }

    #[test]
    fn test_single_word_round_trip() {
        let mut t = Trie::new();
        t.insert("rust");
        assert!(t.contains("rust"));
        assert!(!t.contains("rus"));
        assert!(!t.contains("rusts"));
        assert!(!t.contains(""));
    }

    #[test]
    fn test_multiple_words_and_prefixes() {
        let mut t = Trie::new();
        for w in ["cat", "car", "card", "care", "dare"] {
            t.insert(w);
        }
        assert!(t.contains("cat"));
        assert!(t.contains("car"));
        assert!(t.contains("card"));
        assert!(t.contains("care"));
        assert!(t.contains("dare"));
        // prefixes that are NOT words
        assert!(!t.contains("ca"));
        assert!(!t.contains("c"));
        assert!(!t.contains("dar"));
        // unrelated word
        assert!(!t.contains("dog"));
    }

    #[test]
    fn test_starts_with_prefix_check() {
        let mut t = Trie::new();
        for w in ["cat", "car", "card"] {
            t.insert(w);
        }
        assert!(t.starts_with("ca"));
        assert!(t.starts_with("car"));
        assert!(t.starts_with("card"));
        assert!(t.starts_with("cat"));
        assert!(t.starts_with("")); // empty prefix matches everything
        assert!(!t.starts_with("cb"));
        assert!(!t.starts_with("dog"));
    }

    #[test]
    fn test_insert_empty_string() {
        let mut t = Trie::new();
        t.insert("");
        assert!(t.contains(""));
        assert!(!t.contains("a"));
    }

    // ── Functional Trie ───────────────────────────────────────────────────

    #[test]
    fn test_functional_empty_contains_nothing() {
        let t = FunctionalTrie::empty();
        assert!(!t.contains("cat"));
        assert!(!t.contains(""));
    }

    #[test]
    fn test_functional_single_word() {
        let t = FunctionalTrie::empty().insert("ocaml");
        assert!(t.contains("ocaml"));
        assert!(!t.contains("ocam"));
        assert!(!t.contains("ocamls"));
    }

    #[test]
    fn test_functional_ocaml_example() {
        // Mirrors: List.fold_left (fun t w -> insert w t) empty [...]
        let t = build_trie(["cat", "car", "card", "care", "dare"]);
        assert!(t.contains("cat"));
        assert!(!t.contains("ca"));
        assert!(t.contains("card"));
        assert!(t.contains("dare"));
        assert!(!t.contains("dog"));
    }

    #[test]
    fn test_functional_shared_prefixes() {
        let t = build_trie(["pre", "prefix", "prepare", "present"]);
        assert!(t.contains("pre"));
        assert!(t.contains("prefix"));
        assert!(t.contains("prepare"));
        assert!(t.contains("present"));
        assert!(!t.contains("pref"));
        assert!(!t.contains("presen"));
    }
}
