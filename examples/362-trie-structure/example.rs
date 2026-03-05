use std::collections::HashMap;

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

struct Trie { root: TrieNode }

impl Trie {
    fn new() -> Self { Self { root: TrieNode::default() } }

    fn insert(&mut self, word: &str) {
        let mut node = &mut self.root;
        for c in word.chars() { node = node.children.entry(c).or_default(); }
        node.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            match node.children.get(&c) { None => return false, Some(n) => node = n }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) { None => return false, Some(n) => node = n }
        }
        true
    }

    fn words_with_prefix(&self, prefix: &str) -> Vec<String> {
        let mut node = &self.root;
        for c in prefix.chars() {
            match node.children.get(&c) { None => return vec![], Some(n) => node = n }
        }
        let mut result = Vec::new();
        Self::collect(node, prefix.to_string(), &mut result);
        result
    }

    fn collect(node: &TrieNode, current: String, result: &mut Vec<String>) {
        if node.is_end { result.push(current.clone()); }
        for (c, child) in &node.children {
            let mut next = current.clone(); next.push(*c);
            Self::collect(child, next, result);
        }
    }
}

fn main() {
    let mut t = Trie::new();
    for w in ["apple","app","application","apply","banana","band","bandit"] { t.insert(w); }

    for w in ["apple","app","ap","application","banana","cat"] {
        println!("{w}: search={}, prefix={}", t.search(w), t.starts_with(w));
    }

    let mut words = t.words_with_prefix("app");
    words.sort();
    println!("Words with 'app': {words:?}");
    let mut words2 = t.words_with_prefix("ban");
    words2.sort();
    println!("Words with 'ban': {words2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn insert_search() {
        let mut t = Trie::new();
        t.insert("hello"); t.insert("world");
        assert!(t.search("hello")); assert!(!t.search("hell"));
    }
    #[test] fn prefix() {
        let mut t = Trie::new(); t.insert("apple"); t.insert("app");
        assert!(t.starts_with("app")); assert!(!t.starts_with("xyz"));
    }
    #[test] fn words_with_prefix() {
        let mut t = Trie::new();
        for w in ["cat","car","card","care","cart"] { t.insert(w); }
        let mut r = t.words_with_prefix("car"); r.sort();
        assert_eq!(r, vec!["car","card","care","cart"]);
    }
}
