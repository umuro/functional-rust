//! # Suffix Tree
//! Compressed trie of all suffixes

use std::collections::HashMap;

pub struct SuffixTree {
    pub nodes: Vec<HashMap<char, usize>>,
}

impl SuffixTree {
    pub fn new(s: &str) -> Self {
        let mut tree = SuffixTree { nodes: vec![HashMap::new()] };
        for i in 0..s.len() {
            tree.insert(&s[i..]);
        }
        tree
    }
    
    fn insert(&mut self, suffix: &str) {
        let mut node = 0;
        for c in suffix.chars() {
            if !self.nodes[node].contains_key(&c) {
                let new_node = self.nodes.len();
                self.nodes[node].insert(c, new_node);
                self.nodes.push(HashMap::new());
            }
            node = self.nodes[node][&c];
        }
    }
    
    pub fn contains(&self, pattern: &str) -> bool {
        let mut node = 0;
        for c in pattern.chars() {
            if let Some(&next) = self.nodes[node].get(&c) { node = next; }
            else { return false; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_suffix_tree() {
        let st = SuffixTree::new("banana");
        assert!(st.contains("ana"));
        assert!(!st.contains("xyz"));
    }
}
