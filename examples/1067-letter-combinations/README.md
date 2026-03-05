# 1067: Phone Keypad Letter Combinations

**Difficulty:** Intermediate
**Category:** Backtracking
**Concept:** Generate all possible letter combinations from a phone number's digit mapping
**Key Insight:** Each digit expands into 3-4 branches. The iterative queue approach builds combinations level by level; the fold approach is the most concise — each digit transforms all existing prefixes by appending each possible letter.
