📖 **[View on hightechmind.io →](https://hightechmind.io/rust/882-iterator-consumers)**

---

# 882-iterator-consumers — Iterator Consumers
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Iterator adapters transform sequences lazily, but something must drive the evaluation to completion. Iterator consumers "pull" all values out: `.collect()` materializes into a container, `.sum()` accumulates into a number, `.find()` short-circuits at the first match, and `.fold()` generalizes all of these. OCaml's equivalent consumers are `List.fold_left`, `List.find`, `List.for_all`, `List.exists`, `List.length`. Understanding the consumer landscape determines which one to reach for: `.fold()` is the universal but verbose fallback; specific consumers like `.max()`, `.count()`, and `.any()` communicate intent clearly.

## Learning Outcomes

- Use `.fold()` as the universal consumer for custom accumulation
- Recognize when `.sum()`, `.product()`, `.max()`, `.min()` are more expressive than fold
- Use `.find()` and `.position()` for search with early termination
- Use `.any()` and `.all()` for short-circuiting predicates
- Build a frequency map using `.fold()` into a `HashMap`

## Rust Application

The code implements `sum`, `product`, `concat_strs`, and `running_average` using `.sum()`, `.product()`, `.collect()`, and `.fold()` respectively. `find_first` wraps `.find()`, `find_position` wraps `.position()`, and `count_matching` chains `.filter().count()`. The `group_by_count` function uses `.fold()` into a `HashMap<_, usize>` for frequency counting — the pattern of using `*map.entry(k).or_insert(0) += 1` inside fold is standard Rust for histogram building.

## OCaml Approach

OCaml's `List.fold_left f init xs` is the primary consumer. `List.find pred xs` finds the first matching element (raising `Not_found` rather than returning `option` — `List.find_opt` is the safe version). `List.for_all` and `List.exists` are the all/any equivalents. Frequency maps use `Hashtbl`: `Hashtbl.replace tbl k (1 + try Hashtbl.find tbl k with Not_found -> 0)`. OCaml lacks `List.sum` — it's expressed as `List.fold_left (+) 0`.

## Key Differences

1. **Failure handling**: Rust `.find()` returns `Option<T>` (never panics); OCaml `List.find` raises `Not_found` (use `find_opt` for safety).
2. **Specialized consumers**: Rust has `.sum()`, `.product()`, `.count()`, `.min()`, `.max()` as first-class consumers; OCaml expresses all via `fold_left`.
3. **HashMap accumulation**: Rust `.entry().or_insert()` pattern avoids double-lookup; OCaml's `Hashtbl` requires separate find + replace.
4. **Short-circuiting**: Rust `.any()`, `.all()`, `.find()` short-circuit; OCaml's equivalents `List.for_all`, `List.exists`, `List.find` also short-circuit.

## Exercises

1. Use `.fold()` to implement `max_by_key<T, K: Ord, F: Fn(&T) -> K>` without using `.max_by_key()`.
2. Implement a `word_count_fold` that builds a `HashMap<String, usize>` using a single `.fold()` call over tokenized words.
3. Write `partition_by` using a single `.fold()` that splits a slice into two `Vec`s based on a predicate.
