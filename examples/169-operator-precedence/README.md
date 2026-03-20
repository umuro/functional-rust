📖 **[View on hightechmind.io →](https://hightechmind.io/rust/169-operator-precedence)**

---

# Operator Precedence

## Problem Statement

Operator precedence determines how `1 + 2 * 3 - 4 / 2` is parsed: multiplication and division bind tighter than addition and subtraction, so the result is `1 + (2*3) - (4/2) = 5`. Associativity determines how `1 - 2 - 3` groups: left-associativity gives `(1-2)-3 = -4`, right-associativity gives `1-(2-3) = 2`. Encoding both correctly in a parser requires a systematic approach: this example shows the classic table-driven, multi-level recursive descent method.

## Learning Outcomes

- Understand precedence levels and how they create a parse hierarchy
- Learn left vs. right associativity and how to encode each in recursive descent
- See the `Assoc` enum and precedence table as the structured alternative to Pratt's binding power
- Practice parsing real arithmetic with correct operator grouping

## Rust Application

The parser defines `(precedence: u8, Assoc: Left|Right)` for each operator. `parse_expr(input, min_prec)` uses a loop: parse a number, then while the next operator has precedence `>= min_prec`, consume it and recursively parse the right side. For right-associative operators, the recursive call uses `min_prec` (same level); for left-associative, it uses `min_prec + 1` (strictly higher). This single loop correctly handles all precedence levels without separate functions per level.

## OCaml Approach

OCaml's Menhir generator handles this declaratively. Hand-written OCaml parsers use either recursive descent (one `parse_X` function per level) or the precedence-climbing algorithm (same as this example). The `%left`/`%right`/`%nonassoc` declarations in Menhir are compiled into identical shift-reduce decisions in the generated LALR table.

## Key Differences

1. **Precedence climbing vs. Pratt**: Example 168 uses Pratt (binding powers as numbers); this example uses precedence climbing (precedence + associativity table) — both are equivalent in expressive power.
2. **Generator vs. manual**: OCaml Menhir encodes precedence declaratively; hand-written parsers in both languages use procedural algorithms.
3. **Associativity encoding**: Left-associativity uses `prec + 1` for recursive right-side parsing; right-associativity uses `prec` — the same in Pratt.
4. **Non-associativity**: Some operators are non-associative (`a < b < c` is an error in Python); this requires checking after parsing and emitting an error.

## Exercises

1. Add `**` (exponentiation) as right-associative with higher precedence than `*` and `/`.
2. Verify `1 - 2 - 3` parses as `(1 - 2) - 3 = -4` (left-associative), not `1 - (2 - 3) = 2`.
3. Implement comparison operators (`<`, `>`, `==`) as non-associative — `a < b < c` should be a parse error.
