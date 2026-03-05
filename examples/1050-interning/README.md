# 1050: String Interning

**Difficulty:** Intermediate
**Category:** Data Structures
**Concept:** Deduplicate strings by mapping them to unique integer IDs (symbols)
**Key Insight:** String interning replaces expensive string comparisons (O(n)) with cheap integer comparisons (O(1)). The `Symbol` newtype is `Copy` — it can be passed around as cheaply as a `usize` while representing a rich string value.
