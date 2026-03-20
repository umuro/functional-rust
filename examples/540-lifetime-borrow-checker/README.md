📖 **[View on hightechmind.io →](https://hightechmind.io/rust/540-lifetime-borrow-checker)**

---

# Borrow Checker Internals

## Problem Statement

The borrow checker enforces two rules that together eliminate data races and use-after-free bugs: (1) you can have multiple shared references (`&T`) or exactly one mutable reference (`&mut T`), never both simultaneously; (2) references cannot outlive the data they point to. These rules are based on the "aliasing XOR mutability" principle from the research community. Understanding why these rules exist — not just what they are — makes it easier to design APIs that work with the borrow checker rather than fighting it.

## Learning Outcomes

- The aliasing XOR mutability rule: shared borrows are fine together, but not with mutable borrows
- How NLL allows multiple borrows to exist sequentially in the same block
- How reborrowing creates a temporary shared borrow from a mutable one
- Why preventing aliased mutation eliminates a class of memory safety bugs
- How `rule_exclusive_mutable` demonstrates that sequential pushes are always safe

## Rust Application

`rule_shared_vs_mutable` takes two `&v` borrows simultaneously — legal because both are shared. After NLL ends both borrows, `v.push(4)` is legal. `rule_exclusive_mutable(v: &mut Vec<i32>)` shows sequential pushes — each `v.push` briefly borrows mutably, but they do not overlap. `reborrow_demo` takes a `&mut Vec<i32>`, calls `v.len()` (creating a temporary shared reborrow), then calls `v.push` after the reborrow ends. `ownership_rules` shows that `drop(s)` is explicit ownership transfer.

Key patterns:
- `let r1 = &v; let r2 = &v;` — multiple shared borrows simultaneously — legal
- Sequential `v.push(1); v.push(2);` — borrows overlap in time but the first ends before the second begins
- `v.len()` inside a `&mut` context — temporary shared reborrow from mutable

## OCaml Approach

OCaml has no borrow checker. Aliased mutation is possible and used freely:

```ocaml
let v = ref [1; 2; 3] in
let r1 = v and r2 = v in  (* two references to same list *)
r1 := 42 :: !r1;           (* mutate through r1 *)
Printf.printf "%d\n" (List.length !r2)  (* r2 sees the change *)
```

OCaml programs must use discipline and careful design to avoid bugs that Rust catches at compile time.

## Key Differences

1. **Aliased mutation**: Rust makes aliased mutation a compile-time error; OCaml allows it — correct concurrent programs in OCaml require careful locking discipline.
2. **Data race prevention**: Rust's aliasing XOR mutability rule eliminates data races statically; OCaml 5.x uses domain locks, but race conditions in user code are still possible.
3. **Reborrowing**: Rust's reborrow rules (shared from mutable is safe) are precisely defined; OCaml has no reborrow concept since all references are uniform.
4. **Teaching tool**: Understanding why the borrow checker rejects code helps design better APIs; OCaml developers rely on code review and testing for the same safety properties.

## Exercises

1. **Demonstrate the rule**: Write code that creates two `&v` borrows, uses them, then pushes — add comments showing exactly where each borrow begins and ends per NLL.
2. **Iterator aliasing**: Attempt to create a `&v` iterator and simultaneously push to `v` — observe the error message and explain why it protects against iterator invalidation.
3. **Split borrow**: Implement `fn split_first_rest(v: &mut Vec<i32>) -> (&mut i32, &mut [i32])` using `v.split_first_mut()` and explain why this is safe despite having two mutable references.
