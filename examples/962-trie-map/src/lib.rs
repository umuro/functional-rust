// 962: Trie Map
// OCaml: mutable record with CharMap; Rust: struct with HashMap children

use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct TrieNode<V> {
    value: Option<V>,
    children: HashMap<char, TrieNode<V>>,
}

pub struct Trie<V> {
    root: TrieNode<V>,
}

impl<V> Trie<V> {
    pub fn new() -> Self {
        Trie { root: TrieNode::default() }
    }

    pub fn insert(&mut self, key: &str, value: V) {
        let mut node = &mut self.root;
        for c in key.chars() {
            node = node.children.entry(c).or_insert_with(TrieNode::default);
        }
        node.value = Some(value);
    }

    pub fn search(&self, key: &str) -> Option<&V> {
        let mut node = &self.root;
        for c in key.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return None,
            }
        }
        node.value.as_ref()
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return false,
            }
        }
        true
    }

    // Approach 2: Collect all keys with a given prefix
    pub fn keys_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(child) => node = child,
                None => return vec![],
            }
        }
        let mut results = vec![];
        Self::collect_keys(node, &mut prefix.to_string(), &mut results);
        results
    }

    fn collect_keys(node: &TrieNode<V>, prefix: &mut String, results: &mut Vec<String>) {
        if node.value.is_some() {
            results.push(prefix.clone());
        }
        let mut children: Vec<(&char, &TrieNode<V>)> = node.children.iter().collect();
        children.sort_by_key(|(c, _)| *c);
        for (c, child) in children {
            prefix.push(*c);
            Self::collect_keys(child, prefix, results);
            prefix.pop();
        }
    }
}

impl<V> Default for Trie<V> {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_trie() -> Trie<i32> {
        let mut t = Trie::new();
        t.insert("apple", 1);
        t.insert("app", 2);
        t.insert("application", 3);
        t.insert("banana", 4);
        t.insert("band", 5);
        t
    }

    #[test]
    fn test_search_found() {
        let t = make_trie();
        assert_eq!(t.search("apple"), Some(&1));
        assert_eq!(t.search("app"), Some(&2));
        assert_eq!(t.search("application"), Some(&3));
        assert_eq!(t.search("banana"), Some(&4));
        assert_eq!(t.search("band"), Some(&5));
    }

    #[test]
    fn test_search_not_found() {
        let t = make_trie();
        assert_eq!(t.search("ap"), None);
        assert_eq!(t.search("apricot"), None);
        assert_eq!(t.search(""), None);
        assert_eq!(t.search("xyz"), None);
    }

    #[test]
    fn test_starts_with() {
        let t = make_trie();
        assert!(t.starts_with("app"));
        assert!(t.starts_with("ban"));
        assert!(t.starts_with("apple"));
        assert!(!t.starts_with("xyz"));
        assert!(!t.starts_with("apricot"));
    }

    #[test]
    fn test_update() {
        let mut t = make_trie();
        t.insert("apple", 99);
        assert_eq!(t.search("apple"), Some(&99));
    }

    #[test]
    fn test_prefix_keys() {
        let t = make_trie();
        let mut keys = t.keys_with_prefix("app");
        keys.sort();
        assert_eq!(keys, vec!["app", "apple", "application"]);

        let ban_keys = t.keys_with_prefix("ban");
        assert_eq!(ban_keys.len(), 2);
        assert!(ban_keys.contains(&"banana".to_string()));
        assert!(ban_keys.contains(&"band".to_string()));
    }
}
