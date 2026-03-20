📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1061-word-break)**

---

# 1061-word-break — Word Break

## Problem Statement

Given a string and a dictionary of words, can the string be segmented into a sequence of dictionary words? This is the fundamental problem in natural language tokenization, programming language lexers, and CJK (Chinese/Japanese/Korean) text segmentation where words are not separated by spaces.

The DP solution uses a boolean array where `dp[i]` = true means the first `i` characters form a valid segmentation.

## Learning Outcomes

- Implement word break using bottom-up DP with a `HashSet` dictionary
- Implement the memoized top-down variant
- Use BFS to find all valid segmentations, not just whether one exists
- Understand the connection to CJK text segmentation and tokenization
- Apply `dp[j] && dict.contains(s[j..i])` as the core recurrence

## Rust Application

`src/lib.rs` implements `word_break` with `dp[0] = true` (empty prefix is valid). For each position `i`, it checks all prefixes `s[j..i]` where `dp[j]` is true — if the substring is in the dictionary, set `dp[i] = true`. `word_break_memo` uses a recursive HashMap approach. `word_break_all` uses BFS to find all valid sentence segmentations, not just a yes/no answer.

CJK text segmentation uses this algorithm with statistical dictionaries containing tens of thousands of words. `jieba` (Chinese segmenter used in Elasticsearch) and `mecab` (Japanese) implement variants of this approach.

## OCaml Approach

```ocaml
module StringSet = Set.Make(String)

let word_break s words =
  let dict = List.fold_left (fun s w -> StringSet.add w s) StringSet.empty words in
  let n = String.length s in
  let dp = Array.make (n + 1) false in
  dp.(0) <- true;
  for i = 1 to n do
    for j = 0 to i - 1 do
      if dp.(j) && StringSet.mem (String.sub s j (i - j)) dict then
        dp.(i) <- true
    done
  done;
  dp.(n)
```

Identical structure. `StringSet.mem` is O(log n) while Rust's `HashSet::contains` is O(1) amortized.

## Key Differences

1. **HashSet vs Set**: Rust uses `HashSet<&str>` for O(1) dictionary lookup; OCaml's `StringSet.mem` is O(log n) — Rust's version has better asymptotic behavior.
2. **Substring creation**: Rust's `&s[j..i]` is a zero-copy slice; OCaml's `String.sub` allocates a new string — this matters in hot DP loops.
3. **BFS for all solutions**: Rust's `word_break_all` uses `VecDeque` for BFS; OCaml would use `Queue.t` for the equivalent.
4. **Real-world dictionaries**: Both implementations work, but for million-word dictionaries, the O(1) hash lookup in Rust matters significantly.

## Exercises

1. Add a `word_break_shortest(s: &str, words: &[&str]) -> Option<Vec<&str>>` that finds the segmentation using the fewest words.
2. Implement the word break problem where each word has a score, and you want to maximize the total score of the segmentation.
3. Write a lexer that uses word break DP to tokenize a programming language identifier list into known tokens.
