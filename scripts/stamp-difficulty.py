#!/usr/bin/env python3
"""
stamp-difficulty.py — Write **Difficulty:** tags into every README.md that lacks one.

Reads each example's README. If it has no **Difficulty:** tag, determines the
level using the generator's slug-keyword rules and injects the metadata block
right after the H1 title line.

Usage:
    python3 scripts/stamp-difficulty.py [--dry-run]
"""

import re
import sys
from pathlib import Path

# ---- Config ----
EXAMPLES_DIR = Path(__file__).parent.parent / "examples"
DRY_RUN = "--dry-run" in sys.argv

LEVELS = {
    "fundamental":  "⭐",
    "intermediate": "⭐⭐",
    "advanced":     "⭐⭐⭐",
    "expert":       "⭐⭐⭐⭐",
}

# ---- Keyword tables (mirrors generate-rust-site.py) ----
EXPERT_SLUG_KEYWORDS = [
    "monad", "comonad", "adjunction", "profunctor", "kan-extension",
    "free-", "church", "scott-encoding", "gadt",
    "catamorphism", "anamorphism", "hylomorphism", "apomorphism",
    "paramorphism", "histomorphism", "zygomorphism", "mutumorphism", "prepromorphism",
    "y-combinator", "yoneda", "cofree",
    "type-level-bool", "peano-", "church-numeral", "lambda-calculus",
    "algebraic-effect", "effect-system", "free-monad",
    "category-basics", "curry-howard", "day-convolution", "limits-colimits",
    "grand-synthesis", "propositions-types",
    "effect-intro", "effect-handler", "effect-exception", "effect-state",
    "effect-async", "delimited-cont",
    "existential-types", "rank2-types", "heterogeneous-list", "heterogeneous-vec",
    "type-safe-printf", "singleton-types", "type-equality",
    "van-laarhoven", "waker-context", "fix-point",
]

ADVANCED_SLUG_KEYWORDS = [
    "red-black", "balanced-insert", "avl-tree", "finger-tree", "b-tree-custom",
    "persistent-", "arena-allocation",
    "dijkstra", "shortest-path", "bellman-ford", "a-star-", "articulation-point",
    "topological-sort", "bipartite", "hamiltonian", "minimum-spanning", "aho-corasick",
    "recursive-descent", "parser-combinator", "arithmetic-parser", "char-parser",
    "satisfy-parser", "many-parser", "flatmap-parser", "digit-parser",
    "identifier-parser", "choice-parser", "sequence-parser", "map-parser",
    "optional-parser", "whitespace-parser",
    "huffman", "suffix-array", "convex-hull", "boyer-moore",
    "branch-and-bound", "backtracking-framework", "bloom-filter",
    "approximation-set-cover", "01-knapsack",
    "phantom-type", "phantom-safety", "type-level", "const-generic", "type-witness",
    "higher-kinded", "blanket-implementation", "auto-trait",
    "lifetime-self", "refcell", "unsafe-", "avoid-allocation",
    "async-recursion", "async-stream", "async-sink", "async-generator",
    "async-trait", "async-mutex", "async-rwlock", "async-drop",
    "async-io", "async-join", "async-move", "blocking-in-async",
    "backpressure", "broadcast-channel", "barrier-sync",
    "difference-list", "zipper", "binary-heap-", "priority-queue",
    "builder-pattern", "typestate", "newtype-derive", "visitor-pattern",
    "lense", "optic", "traversal-optic",
    "knapsack", "subset-sum", "dynamic-programming", "memoiz",
    "matrix-chain", "rod-cutting", "partition-equal-subset",
    "egg-drop", "burst-balloons", "stone-game", "regex-matching",
    "wildcard-matching", "longest-path-dp", "interval-dp",
    "n-queens", "sudoku-solver", "subsets-power-set", "permutation-backtrack",
    "recursive-types", "zero-cost-abs",
    "cps", "continuation", "tail-recursive-map-cps", "tail-recursive-map-with",
    "unfold",
    "arc-mutex", "arc-rwlock", "arc-threads", "atomic-types", "actor-pattern",
    "branchless", "bitset", "binary-format", "binary-decimal-fold",
    "associated-type-bound", "associated-types-advanced",
    "adjunction", "bifunctor", "bimap",
    "affine-traversal", "optic-",
    "corecurs", "algorithm-complexity",
    "scatter-gather",
    "never-type", "doubly-linked", "cow-collection", "cow-clone",
    "arena-graph", "reactive-stream", "work-stealing", "cancellation-token",
    "pin-projection", "self-referential", "send-sync",
    "global-allocator", "custom-allocator",
    "dsl-macro", "proc-macro", "macro-rules-advanced",
    "lens-intro", "lens-laws", "lens-modify",
    "prism-", "iso-basics", "iso-laws",
    "traversal", "affine-optic",
    "gat-basics", "gat-collections", "opaque-types", "type-erasure",
    "variance", "phantom-units",
    "type-safe-builder", "coherence-rules", "coherence-orphan",
    "future-trait", "join-futures", "select-futures", "spawn-tasks",
    "pin-unpin", "executor-basics", "semaphore-async",
    "structured-concurrency", "runtime-context",
    "retry-async", "coroutines-gen", "trampoline",
    "rope-data-structure", "slab-allocator", "skip-list", "radix-tree",
    "segment-tree", "fenwick-tree",
    "number-parser", "keyword-parser", "separated-list", "recursive-parser",
    "expression-parser", "operator-precedence", "error-recovery",
    "ini-parser", "lisp-parser", "json-parser",
    "lru-cache",
    "retry-backoff", "circuit-breaker", "rate-limiter", "bulkhead",
    "health-check", "timeout-pattern",
]

INTERMEDIATE_SLUG_KEYWORDS = [
    "binary-tree", "bst", "binary-search-tree", "map-fold-tree", "tree-traversal",
    "symmetric-tree", "complete-tree", "expression-tree", "at-level",
    "tree-string", "tree-preorder", "tree-inorder", "dotstring-tree", "dotstring-parse",
    "collect-leaves", "internal-nodes", "count-leaves",
    "stack-module", "stack-", "-stack", "queue", "deque", "circular-buffer",
    "fold-left", "fold-right", "listfold", "fold-tree", "fold-optic", "try-fold",
    "currying", "partial-app", "partial-application",
    "applying-a-function-twice", "applying-function-twice",
    "function-composition", "function-compose", "pipeline-op",
    "higher-order", "-hof", "function-pointer", "boxing-closure",
    "map-from-scratch", "filter-from-scratch", "list-map-from-scratch",
    "list-filter-from-scratch", "list-map-transform", "list-map",
    "list-filter", "list-fold", "list-operations",
    "caesar", "atbash", "pangram", "isogram", "anagram", "palindrome",
    "hamming", "nucleotide", "word-count", "reverse-word", "word-break",
    "roman-numeral", "phone-number", "edit-distance", "levenshtein",
    "balanced-parentheses", "balanced-paren", "frequency-anal", "frequency-counter",
    "run-length", "-rle", "rle-", "pack-consecutive", "modified-rle",
    "decode-rle", "direct-rle", "eliminate-duplicates", "duplicate-element",
    "flatten-nested", "replicate-n", "rotate-left", "rotate-", "split-list",
    "drop-every", "zip-unzip", "zip-and-unzip", "zip-pair", "zip-with",
    "list-partition", "listpartition", "listmap", "listfilter", "listsort",
    "listflatten", "accumulate-a-result",
    "remove-kth", "insert-at",
    "random-select", "random-permutation", "lotto-draw",
    "combinations", "permutations",
    "error-propagation", "error-handling", "validated", "io-error",
    "parse-error", "multiple-error", "railway", "collecting-result",
    "partition-result", "recover-from", "try-operator",
    "error-conversion", "error-context", "error-accumulation", "error-display",
    "error-combinators", "error-chain", "typed-errors", "result-combinators",
    "result-chaining", "option-to-result", "panic-vs-result",
    "fallible-iterator", "sentinel-vs-result", "parse-int-safe",
    "file-errors", "network-errors", "validation-error",
    "option-map", "option-bind", "option-filter", "option-basics",
    "result-map", "result-bind", "result-basics",
    "iterator", "custom-iterator", "infinite-iterator",
    "iterator-window", "windows-and-chunks", "scan-accumulate",
    "lazy-sequence", "lazy-fib", "double-ended", "exact-size", "step-by",
    "take-while", "skip-while", "peekable", "group-by", "chunk",
    "generic-bounds", "where-clauses", "type-aliases", "type-alias",
    "display-trait", "from-into-traits", "from-into",
    "newtype-pattern", "newtype-",
    "lifetime-basics", "lifetime-elision", "lifetime-",
    "move-semantics", "clone-copy",
    "trait-objects", "trait-object", "dynamic-dispatch",
    "associated-types",
    "insertion-sort", "merge-sort", "quicksort", "bubble-sort",
    "sort-with-custom", "sort-by", "sort-with", "custom-comparator",
    "fibonacci", "sieve-of", "sieve-prime", "sieve",
    "prime-factor", "gcd-lcm",
    "modular-arithmetic", "modular-exp", "binary-search", "greatest-common",
    "euclid", "collatz", "difference-of-squares", "space-age",
    "adjacency-list", "adjacency-matrix", "graph-traversal", "graph-color",
    "minimum-path", "minimum-vertex",
    "monoid-pattern", "monoid-basic", "monoid-generic",
    "accumulate", "scan-", "tail-recursive-accumulator", "point-free",
    "association-list", "btreemap", "btreeset", "hashmap-pattern",
    "hashmap-entry", "hashmap-groupby",
    "sliding-window", "two-pointer", "matrix-ops",
    "csv-parsing", "bench-harness", "benchmarking-harness", "csv-parser",
    "async-basics", "async-block", "async-sequence", "async-map",
    "event-loop", "mpsc-channel", "producer-consumer", "buffered-stream",
    "box-heap", "borrowing-shared", "borrowing-mutable",
    "arraymake", "arrayblit", "stringsplit",
    "bob", "allergies", "clock-module", "luhn", "scrabble",
    "triangle-check", "grains", "beer-song", "perfect-number",
    "isbn-verify", "series-", "robot-simulator",
    "two-bucket", "connect-game", "poker", "dominoes",
    "range",
    "nested-pattern", "or-pattern", "pattern-matching",
    "records",
    "applicative-validation", "applicative-basics", "applicative-laws",
    "lenses",
    "abstract-type",
    "multimap", "interval-map", "sorted-vec", "flat-tree", "trie",
    "linked-list", "singly-linked",
    "small-vec",
    "coin-change", "longest-common-subseq", "lcs-",
    "longest-increasing", "edit-distance-dp",
    "custom-error", "option-result",
    "hashmap-counting", "hashmap-",
    "string-str", "slice-patterns", "vec-patterns",
    "closure-types", "closure-capture", "closure-",
    "deref-coercions", "impl-trait", "cell-interior", "move-closure",
    "object-safe-traits", "sealed-trait", "extension-trait",
    "trait-bounds", "supertrait", "default-methods",
    "trait-specialization", "marker-traits", "trait-dispatch",
    "deref-coercion", "index-trait",
    "hash-eq-ord", "display-debug-traits",
    "option-combinators", "question-mark-operator", "error-trait-impl",
    "from-trait-errors", "anyhow-pattern", "result-transpose",
    "option-transpose", "collect-results", "infallible-conversions",
    "result-ok-err", "error-in-tests", "downcasting", "try-trait",
    "error-downcast",
    "dyn-trait", "const-functions", "maze-solver",
    "entry-api", "indexmap-ordered",
    "disjoint-set", "directed-acyclic-graph", "weighted-graph",
    "sum-types", "record-update",
    "oneshot-channel", "timeout-async", "channel-async",
    "reactive",
    "interning",
    "coin-change", "longest-common", "word-break",
    "permutations-backtrack", "combinations-sum", "letter-combinations",
    "graph-coloring",
]


def classify_slug(slug: str) -> str:
    """Return level based on slug keywords."""
    s = slug.lower()
    for kw in EXPERT_SLUG_KEYWORDS:
        if kw in s:
            return "expert"
    for kw in ADVANCED_SLUG_KEYWORDS:
        if kw in s:
            return "advanced"
    for kw in INTERMEDIATE_SLUG_KEYWORDS:
        if kw in s:
            return "intermediate"
    return "fundamental"


def has_difficulty_tag(text: str) -> bool:
    return bool(re.search(r"\*\*Difficulty:\*\*", text))


def inject_tag(text: str, level: str) -> str:
    """Insert **Difficulty:** line after the first H1 heading."""
    stars = LEVELS[level]
    tag_line = f"\n**Difficulty:** {stars}  \n**Category:** Functional Programming  \n"
    # Find end of first H1 line
    m = re.search(r"^(#\s+.+)$", text, re.MULTILINE)
    if m:
        pos = m.end()
        return text[:pos] + tag_line + text[pos:]
    # No H1 found — prepend
    return tag_line.lstrip("\n") + "\n" + text


def main():
    stamped = 0
    skipped_has_tag = 0
    skipped_empty = 0

    for ex in sorted(EXAMPLES_DIR.iterdir()):
        readme = ex / "README.md"
        if not readme.exists():
            continue
        text = readme.read_text(encoding="utf-8", errors="replace")
        if not text.strip():
            skipped_empty += 1
            continue
        if has_difficulty_tag(text):
            skipped_has_tag += 1
            continue
        level = classify_slug(ex.name)
        new_text = inject_tag(text, level)
        if DRY_RUN:
            print(f"[DRY] {ex.name} → {level}")
        else:
            readme.write_text(new_text, encoding="utf-8")
            print(f"✓ {ex.name} → {level}")
        stamped += 1

    print(f"\nDone: {stamped} tagged, {skipped_has_tag} already had tags, {skipped_empty} empty")


if __name__ == "__main__":
    main()
