// Aho-Corasick Multi-Pattern Matching — O(Σ|pats| + n + matches)
use std::collections::{HashMap, VecDeque};

#[derive(Default, Clone)]
struct AcNode {
    children: HashMap<u8, usize>,
    fail:     usize,
    output:   Vec<usize>, // pattern indices ending here
}

struct AhoCorasick {
    nodes:    Vec<AcNode>,
    patterns: Vec<String>,
}

impl AhoCorasick {
    fn build(patterns: Vec<&str>) -> Self {
        let pats: Vec<String> = patterns.iter().map(|s| s.to_string()).collect();
        let mut nodes = vec![AcNode::default()];

        // Build trie
        for (pid, pat) in pats.iter().enumerate() {
            let mut cur = 0;
            for &c in pat.as_bytes() {
                if !nodes[cur].children.contains_key(&c) {
                    let nid = nodes.len();
                    nodes.push(AcNode::default());
                    nodes[cur].children.insert(c, nid);
                }
                cur = nodes[cur].children[&c];
            }
            nodes[cur].output.push(pid);
        }

        // BFS for failure links
        let mut queue = VecDeque::new();
        let root_children: Vec<(u8, usize)> = nodes[0].children.iter().map(|(&c,&n)|(c,n)).collect();
        for (_, child) in root_children { nodes[child].fail = 0; queue.push_back(child); }

        while let Some(u) = queue.pop_front() {
            // Collect children first to avoid borrow issues
            let children: Vec<(u8, usize)> = nodes[u].children.iter().map(|(&c,&n)|(c,n)).collect();
            for (c, v) in children {
                // failure link for v
                let mut f = nodes[u].fail;
                loop {
                    if let Some(&nf) = nodes[f].children.get(&c) {
                        if nf != v { nodes[v].fail = nf; break; }
                    }
                    if f == 0 { nodes[v].fail = 0; break; }
                    f = nodes[f].fail;
                }
                // Merge output
                let fail_output = nodes[nodes[v].fail].output.clone();
                nodes[v].output.extend(fail_output);
                queue.push_back(v);
            }
        }

        AhoCorasick { nodes, patterns: pats }
    }

    fn search(&self, text: &str) -> Vec<(usize, usize)> {
        let mut state   = 0;
        let mut matches = Vec::new();
        for (i, &c) in text.as_bytes().iter().enumerate() {
            loop {
                if let Some(&next) = self.nodes[state].children.get(&c) {
                    state = next; break;
                }
                if state == 0 { break; }
                state = self.nodes[state].fail;
            }
            for &pid in &self.nodes[state].output {
                let start = i + 1 - self.patterns[pid].len();
                matches.push((start, pid));
            }
        }
        matches
    }
}

fn main() {
    let patterns = vec!["he", "she", "his", "hers"];
    let text     = "ushers";
    let ac       = AhoCorasick::build(patterns.clone());
    let matches  = ac.search(text);

    println!("Text: {text:?}");
    println!("Patterns: {:?}", patterns);
    for (pos, pid) in &matches {
        println!("  Found {:?} at position {pos}", patterns[*pid]);
    }
}
