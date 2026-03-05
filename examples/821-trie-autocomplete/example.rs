/// Trie (Prefix Tree) for autocomplete and prefix search.
///
/// Each node holds a HashMap of children and an `is_end` flag.
/// Insert and lookup are O(k) where k is the key length.

use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, Box<TrieNode>>,
    is_end: bool,
}

#[derive(Default, Debug)]
struct Trie {
    root: TrieNode,
}

impl Trie {
    fn new() -> Self {
        Self::default()
    }

    /// Insert a word into the trie.
    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            node = node.children.entry(ch).or_insert_with(|| Box::new(TrieNode::default()));
        }
        node.is_end = true;
    }

    /// Return true if the exact word exists.
    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return false,
            }
        }
        node.is_end
    }

    /// Return true if any word starts with this prefix.
    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return false,
            }
        }
        true
    }

    /// Return all words that start with the given prefix.
    fn autocomplete(&self, prefix: &str) -> Vec<String> {
        // Navigate to the prefix node
        let mut node = &self.root;
        for ch in prefix.chars() {
            match node.children.get(&ch) {
                Some(child) => node = child,
                None => return vec![],
            }
        }
        // Collect all words in this subtrie
        let mut results = Vec::new();
        Self::collect(node, &mut prefix.to_string(), &mut results);
        results.sort();
        results
    }

    /// DFS: collect all complete words in the subtrie.
    fn collect(node: &TrieNode, current: &mut String, results: &mut Vec<String>) {
        if node.is_end {
            results.push(current.clone());
        }
        // Sort children for deterministic output
        let mut chars: Vec<char> = node.children.keys().copied().collect();
        chars.sort();
        for ch in chars {
            current.push(ch);
            Self::collect(&node.children[&ch], current, results);
            current.pop();
        }
    }
}

fn main() {
    let mut trie = Trie::new();
    let words = ["apple", "app", "application", "apply", "apt", "bat", "ball", "band"];
    for w in &words {
        trie.insert(w);
    }

    println!("autocomplete('app'): {:?}", trie.autocomplete("app"));
    println!("autocomplete('ba'):  {:?}", trie.autocomplete("ba"));
    println!("autocomplete('apt'): {:?}", trie.autocomplete("apt"));
    println!("autocomplete('xyz'): {:?}", trie.autocomplete("xyz"));
    println!("search('app'):       {}", trie.search("app"));
    println!("search('ap'):        {}", trie.search("ap"));
    println!("starts_with('ap'):   {}", trie.starts_with("ap"));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_trie() -> Trie {
        let mut t = Trie::new();
        for w in &["apple", "app", "application", "apply", "apt", "bat", "ball", "band"] {
            t.insert(w);
        }
        t
    }

    #[test]
    fn test_search_exact() {
        let t = make_trie();
        assert!(t.search("app"));
        assert!(t.search("apple"));
        assert!(t.search("apt"));
        assert!(!t.search("ap"));
        assert!(!t.search("xyz"));
    }

    #[test]
    fn test_starts_with() {
        let t = make_trie();
        assert!(t.starts_with("ap"));
        assert!(t.starts_with("ba"));
        assert!(!t.starts_with("xyz"));
    }

    #[test]
    fn test_autocomplete_app() {
        let t = make_trie();
        let mut r = t.autocomplete("app");
        r.sort();
        assert_eq!(r, vec!["app", "apple", "application", "apply"]);
    }

    #[test]
    fn test_autocomplete_ba() {
        let t = make_trie();
        assert_eq!(t.autocomplete("ba"), vec!["ball", "band", "bat"]);
    }

    #[test]
    fn test_autocomplete_missing() {
        let t = make_trie();
        assert_eq!(t.autocomplete("xyz"), Vec::<String>::new());
    }

    #[test]
    fn test_autocomplete_all() {
        let t = make_trie();
        let r = t.autocomplete("");
        assert_eq!(r.len(), 8);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut t = Trie::new();
        t.insert("rust");
        t.insert("rust");
        assert_eq!(t.autocomplete("rust"), vec!["rust"]);
    }
}
