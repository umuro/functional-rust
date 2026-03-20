#![allow(clippy::all)]
//! Trie (Prefix Tree) for efficient string lookups
//!
//! O(m) lookup and prefix search where m is the key length.

use std::collections::HashMap;

/// A trie node containing children and an end-of-word marker
#[derive(Default, Debug, Clone)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    /// Create a new empty trie node
    pub fn new() -> Self {
        Self::default()
    }
}

/// A trie (prefix tree) for string storage and lookup
#[derive(Debug, Clone)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    // === Approach 1: Basic insert/search API ===

    /// Create a new empty trie
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    /// Insert a word into the trie - O(m)
    pub fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_end = true;
    }

    /// Search for an exact word - O(m)
    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            match node.children.get(&c) {
                None => return false,
                Some(n) => node = n,
            }
        }
        node.is_end
    }

    /// Check if any word starts with the given prefix - O(m)
    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                None => return false,
                Some(n) => node = n,
            }
        }
        true
    }

    // === Approach 2: Prefix collection ===

    /// Get all words with a given prefix - O(m + k) where k is result count
    pub fn words_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                None => return vec![],
                Some(n) => node = n,
            }
        }
        let mut result = Vec::new();
        Self::collect(node, prefix.to_string(), &mut result);
        result
    }

    fn collect(node: &TrieNode, current: String, result: &mut Vec<String>) {
        if node.is_end {
            result.push(current.clone());
        }
        for (c, child) in &node.children {
            let mut next = current.clone();
            next.push(*c);
            Self::collect(child, next, result);
        }
    }

    // === Approach 3: Additional utilities ===

    /// Remove a word from the trie (marks it as not an end)
    pub fn remove(&mut self, word: &str) -> bool {
        let mut node = &mut self.root;
        for c in word.chars() {
            match node.children.get_mut(&c) {
                None => return false,
                Some(n) => node = n,
            }
        }
        if node.is_end {
            node.is_end = false;
            true
        } else {
            false
        }
    }

    /// Count all words in the trie
    pub fn count_words(&self) -> usize {
        Self::count_words_recursive(&self.root)
    }

    fn count_words_recursive(node: &TrieNode) -> usize {
        let mut count = if node.is_end { 1 } else { 0 };
        for child in node.children.values() {
            count += Self::count_words_recursive(child);
        }
        count
    }

    /// Get all words in the trie
    pub fn all_words(&self) -> Vec<String> {
        let mut result = Vec::new();
        Self::collect(&self.root, String::new(), &mut result);
        result
    }

    /// Check if the trie is empty
    pub fn is_empty(&self) -> bool {
        self.root.children.is_empty() && !self.root.is_end
    }

    /// Count words with a given prefix
    pub fn count_with_prefix(&self, prefix: &str) -> usize {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                None => return 0,
                Some(n) => node = n,
            }
        }
        Self::count_words_recursive(node)
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");

        assert!(trie.search("hello"));
        assert!(trie.search("world"));
        assert!(!trie.search("hell"));
        assert!(!trie.search("worlds"));
    }

    #[test]
    fn test_starts_with() {
        let mut trie = Trie::new();
        trie.insert("apple");
        trie.insert("application");

        assert!(trie.starts_with("app"));
        assert!(trie.starts_with("apple"));
        assert!(trie.starts_with("appli"));
        assert!(!trie.starts_with("xyz"));
    }

    #[test]
    fn test_words_with_prefix() {
        let mut trie = Trie::new();
        for word in ["cat", "car", "card", "care", "cart"] {
            trie.insert(word);
        }

        let mut results = trie.words_with_prefix("car");
        results.sort();
        assert_eq!(results, vec!["car", "card", "care", "cart"]);
    }

    #[test]
    fn test_empty_prefix() {
        let mut trie = Trie::new();
        trie.insert("a");
        trie.insert("b");

        let mut all = trie.words_with_prefix("");
        all.sort();
        assert_eq!(all, vec!["a", "b"]);
    }

    #[test]
    fn test_remove() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("help");

        assert!(trie.search("hello"));
        assert!(trie.remove("hello"));
        assert!(!trie.search("hello"));
        assert!(trie.search("help"));
    }

    #[test]
    fn test_count_words() {
        let mut trie = Trie::new();
        assert_eq!(trie.count_words(), 0);

        trie.insert("a");
        trie.insert("ab");
        trie.insert("abc");
        assert_eq!(trie.count_words(), 3);
    }

    #[test]
    fn test_count_with_prefix() {
        let mut trie = Trie::new();
        for word in ["apple", "app", "application", "apply", "banana"] {
            trie.insert(word);
        }

        assert_eq!(trie.count_with_prefix("app"), 4);
        assert_eq!(trie.count_with_prefix("ban"), 1);
        assert_eq!(trie.count_with_prefix("xyz"), 0);
    }

    #[test]
    fn test_all_words() {
        let mut trie = Trie::new();
        trie.insert("rust");
        trie.insert("ruby");

        let mut words = trie.all_words();
        words.sort();
        assert_eq!(words, vec!["ruby", "rust"]);
    }

    #[test]
    fn test_is_empty() {
        let mut trie = Trie::new();
        assert!(trie.is_empty());

        trie.insert("test");
        assert!(!trie.is_empty());
    }

    #[test]
    fn test_unicode_support() {
        let mut trie = Trie::new();
        trie.insert("日本語");
        trie.insert("日本");

        assert!(trie.search("日本語"));
        assert!(trie.search("日本"));
        assert!(trie.starts_with("日"));
    }
}
