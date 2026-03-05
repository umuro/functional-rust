# Catamorphism — OCaml vs Rust Comparison

## Core Insight

A catamorphism is the "universal fold" over any algebraic data type. You replace each constructor with a function: `Leaf → leaf_value`, `Node(l,v,r) → node_fn(l_result, v, r_result)`. Once you have `cata`, any tree computation (size, sum, height, mirror) is just choosing the right leaf and node functions.

## OCaml Approach

Labeled arguments make the pattern crystal clear: `cata ~leaf:0 ~node:(fun l _ r -> 1 + l + r)` reads almost like a specification. The parametric polymorphism of the return type means `cata` can produce any type — integers, trees, lists. OCaml's GC handles all the intermediate allocations.

## Rust Approach

Uses `&dyn Fn(R, &T, R) -> R` for the node function — dynamic dispatch via trait objects. The `R: Clone` bound is needed because the leaf value must be cloned for each leaf in the tree. `mirror` can't easily use `cata` because it needs to produce `Tree<T>` which involves ownership — showing where Rust's ownership model creates friction with generic recursive patterns.

## Comparison Table

| Aspect        | OCaml                          | Rust                                 |
|---------------|--------------------------------|--------------------------------------|
| **Memory**    | GC handles all allocations     | Clone bound for leaf, Box for nodes  |
| **Null safety** | N/A                         | N/A                                  |
| **Errors**    | N/A                           | Ownership issues with tree-producing cata |
| **Iteration** | Recursive pattern match        | Recursive pattern match              |
| **Ergonomics**| Labeled args (`~leaf ~node`)   | Trait objects (`&dyn Fn`)            |

## Things Rust Learners Should Notice

1. **`&dyn Fn` vs generic `F: Fn`** — dyn dispatch avoids monomorphization bloat for recursive calls
2. **`R: Clone`** — the leaf value must be cloneable since it's used at every leaf position
3. **`Box<Tree<T>>`** — recursive types need indirection in Rust (Box) but not in OCaml
4. **Mirror breaks the pattern** — producing a `Tree` from `cata` requires Clone on `T` and fighting ownership
5. **Category theory connection** — catamorphisms are the theoretical foundation of `fold` / `reduce`

## Further Reading

- [Catamorphism (Wikipedia)](https://en.wikipedia.org/wiki/Catamorphism)
- [Recursion schemes](https://blog.sumtypeofway.com/posts/introduction-to-recursion-schemes.html)
- [Box for recursive types](https://doc.rust-lang.org/book/ch15-01-box.html)
