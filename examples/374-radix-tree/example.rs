use std::collections::HashMap;

#[derive(Debug, Default)]
struct RadixNode {
    children: HashMap<String, RadixNode>, // edge label -> child
    is_end: bool,
}

impl RadixNode {
    fn new() -> Self { Default::default() }
}

struct RadixTree { root: RadixNode }

impl RadixTree {
    fn new() -> Self { Self { root: RadixNode::new() } }

    fn insert(&mut self, word: &str) { Self::insert_node(&mut self.root, word); }

    fn insert_node(node: &mut RadixNode, remaining: &str) {
        if remaining.is_empty() { node.is_end = true; return; }

        // Find a matching edge
        let key = node.children.keys()
            .find(|k| remaining.starts_with(k.as_str()) || k.starts_with(remaining))
            .cloned();

        match key {
            Some(edge) if remaining.starts_with(&edge[..]) => {
                // remaining has edge as prefix: go deeper
                let rest = &remaining[edge.len()..];
                let child = node.children.get_mut(&edge).unwrap();
                Self::insert_node(child, rest);
            }
            Some(edge) => {
                // Split the edge
                let common_len = edge.chars().zip(remaining.chars()).take_while(|(a,b)| a==b).count();
                let common = edge[..common_len].to_string();
                let edge_rest = edge[common_len..].to_string();
                let word_rest = remaining[common_len..].to_string();
                let mut old_child = node.children.remove(&edge).unwrap();
                let mut new_node = RadixNode::new();
                if word_rest.is_empty() { new_node.is_end = true; }
                new_node.children.insert(edge_rest, old_child);
                if !word_rest.is_empty() {
                    let mut word_node = RadixNode::new();
                    word_node.is_end = true;
                    new_node.children.insert(word_rest, word_node);
                }
                node.children.insert(common, new_node);
            }
            None => {
                // New edge
                let mut child = RadixNode::new();
                child.is_end = true;
                node.children.insert(remaining.to_string(), child);
            }
        }
    }

    fn search(&self, word: &str) -> bool { Self::search_node(&self.root, word) }

    fn search_node(node: &RadixNode, remaining: &str) -> bool {
        if remaining.is_empty() { return node.is_end; }
        for (edge, child) in &node.children {
            if remaining.starts_with(edge.as_str()) {
                return Self::search_node(child, &remaining[edge.len()..]);
            }
        }
        false
    }

    fn starts_with(&self, prefix: &str) -> bool { Self::prefix_node(&self.root, prefix) }

    fn prefix_node(node: &RadixNode, remaining: &str) -> bool {
        if remaining.is_empty() { return true; }
        for (edge, child) in &node.children {
            if edge.starts_with(remaining) { return true; }
            if remaining.starts_with(edge.as_str()) {
                return Self::prefix_node(child, &remaining[edge.len()..]);
            }
        }
        false
    }
}

fn main() {
    let mut rt = RadixTree::new();
    for w in ["car","card","care","cat","cars","app","apple","application"] { rt.insert(w); }

    for w in ["car","card","care","cat","ca","app","apple","xyz"] {
        println!("{w}: search={}, prefix={}", rt.search(w), rt.starts_with(w));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn insert_search() {
        let mut rt = RadixTree::new();
        rt.insert("hello"); rt.insert("world");
        assert!(rt.search("hello")); assert!(!rt.search("hell"));
    }
    #[test] fn prefix_search() {
        let mut rt = RadixTree::new();
        for w in ["car","card","care"] { rt.insert(w); }
        assert!(rt.starts_with("car")); assert!(rt.starts_with("ca")); assert!(!rt.starts_with("xyz"));
    }
    #[test] fn shared_prefix() {
        let mut rt = RadixTree::new();
        rt.insert("car"); rt.insert("card");
        assert!(rt.search("car")); assert!(rt.search("card")); assert!(!rt.search("carde"));
    }
}
