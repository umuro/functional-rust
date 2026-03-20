#![allow(clippy::all)]
//! Radix Tree (Compressed Trie)
//!
//! Space-efficient prefix tree with edge compression.

use std::collections::HashMap;

/// Radix tree node with compressed edges
#[derive(Debug, Default)]
pub struct RadixNode {
    children: HashMap<String, RadixNode>,
    is_end: bool,
}

impl RadixNode {
    fn new() -> Self {
        Default::default()
    }
}

/// A radix tree (Patricia trie)
pub struct RadixTree {
    root: RadixNode,
}

impl RadixTree {
    pub fn new() -> Self {
        Self {
            root: RadixNode::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        Self::insert_node(&mut self.root, word);
    }

    fn insert_node(node: &mut RadixNode, remaining: &str) {
        if remaining.is_empty() {
            node.is_end = true;
            return;
        }

        // Find matching edge
        let key = node
            .children
            .keys()
            .find(|k| remaining.starts_with(k.as_str()) || k.starts_with(remaining))
            .cloned();

        match key {
            Some(edge) if remaining.starts_with(&edge) => {
                let rest = &remaining[edge.len()..];
                let child = node.children.get_mut(&edge).unwrap();
                Self::insert_node(child, rest);
            }
            Some(edge) => {
                // Split edge
                let common_len = edge
                    .chars()
                    .zip(remaining.chars())
                    .take_while(|(a, b)| a == b)
                    .count();
                let common: String = edge.chars().take(common_len).collect();
                let edge_rest: String = edge.chars().skip(common_len).collect();
                let word_rest: String = remaining.chars().skip(common_len).collect();

                let old_child = node.children.remove(&edge).unwrap();
                let mut new_node = RadixNode::new();

                new_node.children.insert(edge_rest, old_child);

                if word_rest.is_empty() {
                    new_node.is_end = true;
                } else {
                    let mut word_node = RadixNode::new();
                    word_node.is_end = true;
                    new_node.children.insert(word_rest, word_node);
                }

                node.children.insert(common, new_node);
            }
            None => {
                let mut child = RadixNode::new();
                child.is_end = true;
                node.children.insert(remaining.to_string(), child);
            }
        }
    }

    pub fn search(&self, word: &str) -> bool {
        Self::search_node(&self.root, word)
    }

    fn search_node(node: &RadixNode, remaining: &str) -> bool {
        if remaining.is_empty() {
            return node.is_end;
        }
        for (edge, child) in &node.children {
            if remaining.starts_with(edge.as_str()) {
                return Self::search_node(child, &remaining[edge.len()..]);
            }
        }
        false
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        Self::prefix_node(&self.root, prefix)
    }

    fn prefix_node(node: &RadixNode, remaining: &str) -> bool {
        if remaining.is_empty() {
            return true;
        }
        for (edge, child) in &node.children {
            if edge.starts_with(remaining) || remaining.starts_with(edge.as_str()) {
                if edge.starts_with(remaining) {
                    return true;
                }
                return Self::prefix_node(child, &remaining[edge.len()..]);
            }
        }
        false
    }
}

impl Default for RadixTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_search() {
        let mut rt = RadixTree::new();
        rt.insert("test");
        rt.insert("testing");
        rt.insert("team");

        assert!(rt.search("test"));
        assert!(rt.search("testing"));
        assert!(rt.search("team"));
        assert!(!rt.search("tea"));
    }

    #[test]
    fn test_prefix() {
        let mut rt = RadixTree::new();
        rt.insert("rust");
        rt.insert("ruby");

        assert!(rt.starts_with("ru"));
        assert!(rt.starts_with("rus"));
        assert!(!rt.starts_with("py"));
    }

    #[test]
    fn test_compression() {
        let mut rt = RadixTree::new();
        rt.insert("romane");
        rt.insert("romanus");
        rt.insert("romulus");

        assert!(rt.search("romane"));
        assert!(rt.search("romanus"));
        assert!(rt.search("romulus"));
    }

    #[test]
    fn test_single_char() {
        let mut rt = RadixTree::new();
        rt.insert("a");
        rt.insert("ab");
        rt.insert("abc");

        assert!(rt.search("a"));
        assert!(rt.search("ab"));
        assert!(rt.search("abc"));
    }

    #[test]
    fn test_empty_search() {
        let rt = RadixTree::new();
        assert!(!rt.search("anything"));
    }
}
