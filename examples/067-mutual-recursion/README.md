# Example 067: Mutual Recursion with `and`

**Difficulty:** ⭐
**Category:** Recursion
**Concept:** Functions that call each other require special syntax in OCaml (`let rec ... and ...`) but work naturally in Rust — any function in scope can call any other. Also demonstrates mutual recursion over algebraic data types with an expression evaluator.
**OCaml → Rust insight:** OCaml requires the `and` keyword for mutual recursion because definitions are processed top-to-bottom; Rust resolves all items in a module simultaneously, so mutual recursion needs no special syntax.
