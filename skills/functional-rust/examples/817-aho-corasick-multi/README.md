# 817: Aho-Corasick Multi-Pattern Matching Automaton

**Difficulty:** 5  **Level:** Master

Search a text for all occurrences of multiple patterns simultaneously in O(|text| + Σ|patterns| + matches).

## The Problem This Solves

Naïve multi-pattern search runs each pattern separately: O(n × m) for n text characters and m total pattern length. Aho-Corasick builds a finite automaton from all patterns at once, then scans the text in a single pass — every character processed exactly once, regardless of how many patterns you're searching for.

Use Aho-Corasick for network intrusion detection (matching thousands of attack signatures against a packet stream), antivirus scanning (matching malware signatures against file content), log analysis (finding any of a set of error patterns in a log), and DNA sequence search (matching multiple gene markers simultaneously). The classic grep -F (fixed-string multi-pattern) uses exactly this algorithm.

The output is a list of `(pattern_id, end_position)` pairs for every match found. Overlapping matches across patterns are reported naturally.

## The Intuition

Three phases:
1. **Build a trie** from all patterns. Each pattern path ends at a terminal node marked with the pattern's id.
2. **Add failure links** (like KMP's failure function, but for a trie). The failure link of node `v` points to the longest proper suffix of `v`'s string that is also a prefix of some pattern. Computed via BFS from the root.
3. **Scan the text**: follow trie edges on match, follow failure links on mismatch. At each position, also follow output links to report all patterns that end here (including patterns that are suffixes of other patterns).

The result is a deterministic finite automaton where every state transition is O(1). Total construction: O(Σ|patterns| × |alphabet|). Scan: O(|text| + matches).

OCaml would represent this with a recursive trie node type and hash maps for children. Rust uses arena-allocated nodes with `HashMap<char, usize>` children — indices into a `Vec<Node>` instead of boxing.

## How It Works in Rust

```rust
use std::collections::{HashMap, VecDeque};

struct AhoCorasick {
    goto: Vec<HashMap<char, usize>>, // goto[state][char] = next_state
    fail: Vec<usize>,                // failure link
    output: Vec<Vec<usize>>,         // output[state] = list of pattern ids ending here
}

impl AhoCorasick {
    fn build(patterns: &[&str]) -> Self {
        let mut goto: Vec<HashMap<char, usize>> = vec![HashMap::new()];
        let mut output: Vec<Vec<usize>> = vec![vec![]];

        // Phase 1: build trie
        for (pid, pat) in patterns.iter().enumerate() {
            let mut state = 0;
            for ch in pat.chars() {
                let next = goto[state].get(&ch).copied();
                state = next.unwrap_or_else(|| {
                    let new = goto.len();
                    goto.push(HashMap::new());
                    output.push(vec![]);
                    goto[state].insert(ch, new);
                    new
                });
            }
            output[state].push(pid); // pattern pid ends at this state
        }

        // Phase 2: BFS to set failure links
        let n = goto.len();
        let mut fail = vec![0usize; n];
        let mut queue = VecDeque::new();

        // Root's children fail back to root
        for (&ch, &s) in &goto[0] {
            fail[s] = 0;
            queue.push_back(s);
        }

        while let Some(r) = queue.pop_front() {
            for (&ch, &s) in goto[r].clone().iter() {
                // Failure link of s: follow r's failure chain until we find
                // a state with a goto on ch, or fall back to root
                let mut f = fail[r];
                while f != 0 && !goto[f].contains_key(&ch) { f = fail[f]; }
                fail[s] = if goto[f].contains_key(&ch) && goto[f][&ch] != s {
                    goto[f][&ch]
                } else { 0 };
                // Merge output: patterns at fail[s] also match here
                let inherited = output[fail[s]].clone();
                output[s].extend(inherited);
                queue.push_back(s);
            }
        }
        AhoCorasick { goto, fail, output }
    }

    fn search(&self, text: &str) -> Vec<(usize, usize)> {
        let mut state = 0;
        let mut matches = vec![];
        for (i, ch) in text.chars().enumerate() {
            // Follow failure links until we find a goto or reach root
            while state != 0 && !self.goto[state].contains_key(&ch) {
                state = self.fail[state];
            }
            if let Some(&next) = self.goto[state].get(&ch) { state = next; }
            // Report all patterns ending at current position
            for &pid in &self.output[state] { matches.push((pid, i)); }
        }
        matches
    }
}
```

The arena `Vec<HashMap<char, usize>>` pattern avoids pointer chasing through boxed nodes. `goto[r].clone().iter()` during BFS is needed because we borrow `goto` mutably elsewhere — in a production implementation, separate the construction phase to avoid this clone.

## What This Unlocks

- **Network intrusion detection**: Snort/Suricata use Aho-Corasick variants to match thousands of attack signatures against each packet.
- **Antivirus / malware scanning**: match a database of binary signatures in a single pass over file content.
- **Bioinformatics**: simultaneous search for multiple DNA motifs or protein domains in genome sequences.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Trie node | Recursive variant type with `Hashtbl` children | `Vec<HashMap<char, usize>>` — arena, no boxing |
| Node allocation | `ref` cells or GC-managed heap | Index into `Vec` — explicit, cache-local |
| BFS queue | `Queue.t` | `VecDeque<usize>` |
| Output merging | List concatenation | `Vec::extend` — in-place |
| Clone during BFS | Not needed (GC copies) | `goto[r].clone()` to avoid borrow conflict during mutation |
