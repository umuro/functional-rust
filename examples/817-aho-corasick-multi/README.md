📖 **[View on hightechmind.io →](https://hightechmind.io/rust/817-aho-corasick-multi)**

---

# Aho-Corasick Multi-Pattern Search

## Problem Statement

When you need to find hundreds of keywords simultaneously in a large text — network intrusion detection scanning for attack signatures, spam filters checking for banned phrases, DNA analysis searching for multiple probes — running each pattern separately costs O(n * k) where k is the number of patterns. Aho-Corasick solves this by building a trie of all patterns and adding failure links so that mismatches fall back to the longest suffix that is also a prefix of some pattern. The result is O(n + m + z) where n is text length, m is total pattern length, and z is match count — regardless of how many patterns there are. This is the algorithm powering `fgrep -f patterns.txt`, network packet inspection, and antivirus scanning.

## Learning Outcomes

- Build a trie (prefix tree) from a set of patterns with nodes tracking pattern completions
- Compute failure links via BFS, connecting mismatch states to the longest proper suffix-prefix
- Understand why BFS is necessary for failure link computation (level-by-level guarantees parent links exist)
- Implement the search phase that follows failure links on mismatch rather than restarting
- Recognize output links that allow reporting multiple overlapping patterns ending at the same position

## Rust Application

```rust
pub struct AhoCorasick {
    goto: Vec<HashMap<char, usize>>,
    fail: Vec<usize>,
    output: Vec<Vec<String>>,
}
impl AhoCorasick {
    pub fn new(patterns: &[&str]) -> Self { /* BFS failure link build */ }
    pub fn search(&self, text: &str) -> Vec<(usize, &str)> { /* follow automaton */ }
}
```

Each node is a `HashMap<char, usize>` mapping characters to child state indices, naturally handling Unicode without a 256-element array. The `fail` vector stores the failure link for each state, computed during BFS construction. The `output` vector at each node accumulates all patterns that end at that state (including patterns reachable via output links). During search, on mismatch the automaton follows `fail` links until a transition exists or the root is reached. Rust's ownership model makes the multi-pattern result collection clean with `Vec<(usize, &str)>`.

## OCaml Approach

OCaml represents the Aho-Corasick automaton using arrays of `int option` for goto transitions or `Hashtbl` per node. Failure links are mutable `int ref` values set during BFS using a `Queue`. The `output` at each node is a `string list ref` grown by appending output-linked completions. OCaml's polymorphic variants can express node states cleanly. The `Buffer` module builds the goto structure, and `Array.make` creates fixed-size transition arrays for ASCII alphabets. The search function is naturally tail-recursive with the current state threaded through.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Goto table | `Vec<HashMap<char, usize>>` | `int array` or `Hashtbl` per node |
| Failure links | `Vec<usize>` indexed by state | `int array` or `int ref` per node |
| BFS queue | `std::collections::VecDeque` | `Queue.t` from standard library |
| Output links | `Vec<Vec<String>>` | `string list ref` per node |
| Unicode support | HashMap handles any char | Depends on implementation |
| Memory layout | Cache-friendly Vec storage | GC-managed node heap |

## Exercises

1. Add support for case-insensitive matching by lowercasing during trie construction.
2. Implement output links to efficiently report all patterns ending at each position, not just direct matches.
3. Measure the speedup over k separate Boyer-Moore searches for k=10, 100, 1000 patterns.
4. Serialize the compiled automaton to disk and reload it without rebuilding from patterns.
5. Extend to report overlapping matches where one pattern is a substring of another.
