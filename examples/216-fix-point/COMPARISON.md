# OCaml vs Rust: Fix Point — How Recursive Types Work Under the Hood

## Side-by-Side Code

### OCaml

```ocaml
(* Shape of one list layer — NOT recursive, A is the child slot *)
type 'a list_f = NilF | ConsF of int * 'a

(* Functorial map over the child slot *)
let map_list_f f = function
  | NilF -> NilF
  | ConsF (x, rest) -> ConsF (x, f rest)

(* Fix point: wrap ListF around itself to obtain full recursion *)
type fix_list = FixL of fix_list list_f

let nil = FixL NilF
let cons x xs = FixL (ConsF (x, xs))
let unfix_l (FixL f) = f

(* Catamorphism: only place recursion lives *)
let rec cata_list alg (FixL f) =
  alg (map_list_f (cata_list alg) f)

(* Sum algebra — no recursion in the algebra itself *)
let sum = cata_list (function NilF -> 0 | ConsF (x, acc) -> x + acc)
```

### Rust (idiomatic — concrete fix-point types)

```rust
// Shape of one list layer — A is the child slot
pub enum ListF<A> { NilF, ConsF(i64, A) }

impl<A> ListF<A> {
    pub fn map<B>(self, f: impl FnOnce(A) -> B) -> ListF<B> {
        match self {
            ListF::NilF => ListF::NilF,
            ListF::ConsF(x, rest) => ListF::ConsF(x, f(rest)),
        }
    }
}

// Fix<ListF>: ListF wrapping itself via Box to break infinite size
pub struct FixList(Box<ListF<FixList>>);

impl FixList {
    pub fn unfix(self) -> ListF<FixList> { *self.0 }
    pub fn nil() -> Self { FixList(Box::new(ListF::NilF)) }
    pub fn cons(x: i64, xs: FixList) -> Self {
        FixList(Box::new(ListF::ConsF(x, xs)))
    }
}

// Catamorphism — all recursion here, algebras stay local
pub fn cata_list<A>(list: FixList, alg: &impl Fn(ListF<A>) -> A) -> A {
    alg(list.unfix().map(|child| cata_list(child, alg)))
}
```

### Rust (binary tree — same pattern, two child slots)

```rust
pub enum TreeF<A> { LeafF(i64), BranchF(A, A) }

impl<A> TreeF<A> {
    pub fn map<B>(self, mut f: impl FnMut(A) -> B) -> TreeF<B> {
        match self {
            TreeF::LeafF(n) => TreeF::LeafF(n),
            TreeF::BranchF(l, r) => TreeF::BranchF(f(l), f(r)),
        }
    }
}

pub struct FixTree(Box<TreeF<FixTree>>);

pub fn cata_tree<A>(tree: FixTree, alg: &impl Fn(TreeF<A>) -> A) -> A {
    alg(tree.unfix().map(|child| cata_tree(child, alg)))
}

// Depth algebra — zero recursion in the user code
let depth = cata_tree(tree, &|node: TreeF<usize>| match node {
    TreeF::LeafF(_) => 0,
    TreeF::BranchF(l, r) => 1 + l.max(r),
});
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Shape functor | `type 'a list_f = NilF \| ConsF of int * 'a` | `enum ListF<A> { NilF, ConsF(i64, A) }` |
| Fix point | `type fix_list = FixL of fix_list list_f` | `struct FixList(Box<ListF<FixList>>)` |
| Peel one layer | `let unfix_l (FixL f) = f` | `fn unfix(self) -> ListF<FixList> { *self.0 }` |
| Algebra type | `list_f 'a -> 'a` | `impl Fn(ListF<A>) -> A` |
| Catamorphism | `('a list_f -> 'a) -> fix_list -> 'a` | `fn cata_list<A>(FixList, &impl Fn(ListF<A>) -> A) -> A` |
| Functorial map | `map_list_f : ('a -> 'b) -> 'a list_f -> 'b list_f` | `fn map<B>(self, f: impl FnOnce(A) -> B) -> ListF<B>` |

## Key Insights

1. **Shape vs recursion**: OCaml and Rust both use the same trick — define a non-recursive `F<A>` shape, then tie the knot by substituting `A = Fix<F>`. The shape and the recursion are cleanly separated in both languages.

2. **Box for size**: OCaml's `FixL of fix_list list_f` is naturally heap-allocated (OCaml values are boxed by default). In Rust, `Box<ListF<FixList>>` makes the type's size finite and explicit — without `Box`, the compiler rejects the definition because `FixList` would have infinite size.

3. **Passing algebras by reference**: OCaml closures are heap-allocated and implicitly shared, so `cata_list alg` can recurse freely. In Rust, passing `alg: &impl Fn(...)` by shared reference lets `cata_list` call itself recursively without moving or cloning the closure.

4. **`FnOnce` vs `FnMut`**: `ListF::map` can use `FnOnce` because the child slot appears exactly once. `TreeF::map` requires `FnMut` because `BranchF` has *two* children and the closure must be called twice.

5. **No HKT, but same power**: Rust lacks higher-kinded types, so we cannot write a single universal `Fix<F>` that works for any `F`. Instead we define `FixList` and `FixTree` as concrete newtype wrappers. The pattern is identical; only the types are spelled out. Each `cata_X` function is a standalone, type-safe catamorphism.

## When to Use Each Style

**Use a concrete Fix type (`FixList`, `FixTree`) when:** you want the separation-of-shape-from-recursion benefit without fighting the type system — common for embedded DSLs, expression trees, or any recursive data where you want reusable fold/map/render logic.

**Use a direct recursive type (`enum List { Nil, Cons(i64, Box<List>) }`) when:** the structure is simple, you don't need generic catamorphisms, and the extra indirection of Fix-point encoding would obscure rather than clarify the code.
