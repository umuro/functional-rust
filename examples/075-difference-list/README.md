📖 **[View on hightechmind.io →](https://hightechmind.io/rust/075-difference-list)**

---

# 075 — Difference List

## Problem Statement

A difference list represents a list as a function from "remaining suffix" to "complete list". Appending two difference lists is O(1) — just function composition — instead of O(n). This is important in functional programming where repeated left-fold appends create O(n²) behavior.

Difference lists originate in Prolog (1984) and appear in Haskell's `ShowS` type for efficient string building. The key insight: `[1,2,3]` as a difference list is `fun suffix -> [1,2,3] ++ suffix`. Appending is `fun suffix -> dl1(dl2(suffix))`. `ShowS = String -> String` in Haskell is exactly a difference list for strings.

## Learning Outcomes

- Understand difference lists as function composition for O(1) append
- Implement `DList::empty()`, `singleton(x)`, `from_vec(v)`, `append(self, other)`, `to_vec()`
- Recognize that append is function composition: `(dl1.append(dl2)).run(tail) = dl1.run(dl2.run(tail))`
- Connect to `ShowS` (Haskell) and `Buffer` (OCaml) as practical applications
- Understand when difference lists matter: left-associative appends in recursive algorithms

## Rust Application

`DList<T>` wraps `Box<dyn Fn(Vec<T>) -> Vec<T>>`. `empty()` is the identity function. `singleton(x)` prepends `x`. `append(self, other)` composes the two functions: `move |tail| self.f(other.f(tail))`. `to_vec()` calls `self.f(vec![])`. The composition makes append O(1) — no data is moved until `to_vec()` is called.

## OCaml Approach

OCaml's difference list: `type 'a dlist = 'a list -> 'a list`. `let empty = fun xs -> xs`. `let singleton x = fun xs -> x :: xs`. `let append dl1 dl2 = fun xs -> dl1 (dl2 xs)`. `let to_list dl = dl []`. `let from_list lst = fun xs -> lst @ xs`. This is exactly the same pattern — function composition.

## Key Differences

1. **`Box<dyn Fn>` vs plain function**: Rust requires `Box<dyn Fn>` because each closure has a unique type. OCaml's closures have a uniform representation — `'a list -> 'a list` is a first-class type without boxing.
2. **`FnOnce` vs `Fn`**: Rust's `DList` here uses `FnOnce` — each difference list can only be used once (because `append` consumes both). A version using `Fn` requires `T: Clone`. OCaml's closures can be called multiple times.
3. **Practical use in Rust**: Rust's `String::push_str` and `Vec::extend` are already amortized O(1), so difference lists are less critical than in OCaml with its immutable lists. The concept is pedagogically important.
4. **`ShowS` in Haskell**: Haskell's `type ShowS = String -> String` is a difference list for strings. `showsPrec` uses it to build `Show` instances efficiently.

## Exercises

1. **String difference list**: Define `type Dstr = Box<dyn FnOnce(String) -> String>` and implement `dstr_singleton(c: char)`, `dstr_append`, and `dstr_to_string`. Compare with repeated `String::push` for deep append trees.
2. **Performance test**: Build a list `[1..n]` using left-associative appends with both `Vec` and `DList`. Measure the time difference for n=100,000. Explain the O(n²) vs O(n) complexity.
3. **From DList to iterator**: Instead of `to_vec()`, implement `DList::into_iter(self) -> impl Iterator<Item=T>` that yields elements lazily without materializing the full Vec.
