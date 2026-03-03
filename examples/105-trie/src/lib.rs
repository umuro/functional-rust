//! # Trie — Prefix Tree for Strings
//!
//! A trie stores strings with shared prefixes efficiently.
//! OCaml's `Map.Make(Char)` for children maps to Rust's `HashMap<char, Trie>`.

use std::collections::HashMap;
use std::collections::BTreeMap;

// ---------------------------------------------------------------------------
// Approach A: HashMap-based trie (idiomatic Rust)
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct Trie {
    is_word: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_word
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return false,
            }
        }
        true
    }
}

// ---------------------------------------------------------------------------
// Approach B: Immutable trie (mirrors OCaml's functional style)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct FunctionalTrie {
    is_word: bool,
    children: BTreeMap<char, FunctionalTrie>,
}

impl FunctionalTrie {
    pub fn empty() -> Self {
        FunctionalTrie { is_word: false, children: BTreeMap::new() }
    }

    pub fn insert(&self, word: &str) -> Self {
        self.insert_chars(&word.chars().collect::<Vec<_>>(), 0)
    }

    fn insert_chars(&self, chars: &[char], i: usize) -> Self {
        if i == chars.len() {
            FunctionalTrie { is_word: true, children: self.children.clone() }
        } else {
            let c = chars[i];
            let child = self.children.get(&c)
                .cloned()
                .unwrap_or_else(FunctionalTrie::empty);
            let new_child = child.insert_chars(chars, i + 1);
            let mut new_children = self.children.clone();
            new_children.insert(c, new_child);
            FunctionalTrie { is_word: self.is_word, children: new_children }
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        let chars: Vec<char> = word.chars().collect();
        self.contains_chars(&chars, 0)
    }

    fn contains_chars(&self, chars: &[char], i: usize) -> bool {
        if i == chars.len() {
            self.is_word
        } else {
            match self.children.get(&chars[i]) {
                Some(child) => child.contains_chars(chars, i + 1),
                None => false,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Approach C: Array-based trie (performance-oriented, ASCII only)
// ---------------------------------------------------------------------------

pub struct ArrayTrie {
    is_word: bool,
    children: [Option<Box<ArrayTrie>>; 26],
}

impl ArrayTrie {
    pub fn new() -> Self {
        ArrayTrie {
            is_word: false,
            children: Default::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(ArrayTrie::new()));
        }
        node.is_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            match &node.children[idx] {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_contains() {
        let mut t = Trie::new();
        for w in &["cat", "car", "card", "care", "dare"] {
            t.insert(w);
        }
        assert!(t.contains("cat"));
        assert!(!t.contains("ca"));
        assert!(t.contains("card"));
        assert!(t.contains("dare"));
        assert!(!t.contains("dog"));
    }

    #[test]
    fn test_starts_with() {
        let mut t = Trie::new();
        t.insert("hello");
        assert!(t.starts_with("hel"));
        assert!(!t.starts_with("world"));
    }

    #[test]
    fn test_functional_trie() {
        let t = FunctionalTrie::empty();
        let t = t.insert("cat").insert("car").insert("card");
        assert!(t.contains("cat"));
        assert!(t.contains("car"));
        assert!(!t.contains("ca"));
    }

    #[test]
    fn test_array_trie() {
        let mut t = ArrayTrie::new();
        t.insert("cat");
        t.insert("car");
        assert!(t.contains("cat"));
        assert!(t.contains("car"));
        assert!(!t.contains("ca"));
    }

    #[test]
    fn test_empty_string() {
        let mut t = Trie::new();
        t.insert("");
        assert!(t.contains(""));
    }
}
