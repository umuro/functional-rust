//! # Aho-Corasick Algorithm
//! Multi-pattern string matching

use std::collections::HashMap;

pub struct AhoCorasick {
    goto: Vec<HashMap<u8, usize>>,
    fail: Vec<usize>,
    output: Vec<Vec<usize>>,
}

impl AhoCorasick {
    pub fn new(patterns: &[&str]) -> Self {
        let mut ac = AhoCorasick { goto: vec![HashMap::new()], fail: vec![0], output: vec![vec![]] };
        for (idx, pat) in patterns.iter().enumerate() {
            let mut state = 0;
            for &c in pat.as_bytes() {
                if !ac.goto[state].contains_key(&c) {
                    ac.goto[state].insert(c, ac.goto.len());
                    ac.goto.push(HashMap::new());
                    ac.fail.push(0);
                    ac.output.push(vec![]);
                }
                state = ac.goto[state][&c];
            }
            ac.output[state].push(idx);
        }
        ac
    }
    
    pub fn search(&self, text: &str) -> Vec<(usize, usize)> {
        let mut state = 0;
        let mut results = vec![];
        for (i, &c) in text.as_bytes().iter().enumerate() {
            while state != 0 && !self.goto[state].contains_key(&c) { state = self.fail[state]; }
            state = *self.goto[state].get(&c).unwrap_or(&0);
            for &pat_idx in &self.output[state] { results.push((i, pat_idx)); }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ac() {
        let ac = AhoCorasick::new(&["he", "she", "his"]);
        let matches = ac.search("ushers");
        assert!(!matches.is_empty());
    }
}
