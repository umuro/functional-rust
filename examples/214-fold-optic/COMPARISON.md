# OCaml vs Rust: Fold Optic — Read-Only Multi-Focus Aggregation

## Side-by-Side Code

### OCaml
```ocaml
type ('s, 'a) fold_simple = { to_list : 's -> 'a list }

let sum_of f s = List.fold_left ( + ) 0 (f.to_list s)
let any_of f pred s = List.exists pred (f.to_list s)
let all_of f pred s = List.for_all pred (f.to_list s)
let find_of f pred s = List.find_opt pred (f.to_list s)

(* Composition via List.concat_map *)
let compose outer inner =
  { to_list = fun s -> List.concat_map inner.to_list (outer.to_list s) }

let team_scores = compose team_members member_scores
```

### Rust (idiomatic — struct-based)
```rust
pub struct Fold<S, A> {
    to_list: Box<dyn Fn(&S) -> Vec<A>>,
}

impl<S: 'static> Fold<S, i32> {
    pub fn sum_of(&self, s: &S) -> i32 {
        self.collect(s).into_iter().sum()
    }
    pub fn any_of(&self, s: &S, pred: impl Fn(&A) -> bool) -> bool {
        self.collect(s).iter().any(pred)
    }
}

// Composition: flat-map the inner fold over the outer fold's results
pub fn compose<B: 'static>(self, other: Fold<A, B>) -> Fold<S, B> {
    Fold::new(move |s| {
        self.collect(s).iter().flat_map(|a| other.collect(a)).collect()
    })
}

let team_scores = team_members_fold().compose(member_scores_fold());
```

### Rust (functional/recursive — trait-based)
```rust
pub trait FoldOptic<S, A> {
    fn fold_collect(&self, s: &S) -> Vec<A>;

    fn fold_length(&self, s: &S) -> usize {
        self.fold_collect(s).len()
    }
    fn fold_any(&self, s: &S, pred: impl Fn(&A) -> bool) -> bool {
        self.fold_collect(s).iter().any(pred)
    }
    fn fold_all(&self, s: &S, pred: impl Fn(&A) -> bool) -> bool {
        self.fold_collect(s).iter().all(pred)
    }
}

pub struct TeamMembersFold;

impl FoldOptic<Team, Member> for TeamMembersFold {
    fn fold_collect(&self, s: &Team) -> Vec<Member> { s.members.clone() }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Fold type | `('s, 'a) fold_simple = { to_list : 's -> 'a list }` | `Fold<S, A> { to_list: Box<dyn Fn(&S) -> Vec<A>> }` |
| Collect | `f.to_list s : 'a list` | `fold.collect(s) : Vec<A>` |
| Sum | `List.fold_left (+) 0 (f.to_list s)` | `.into_iter().sum()` |
| Composition | `List.concat_map inner.to_list (outer.to_list s)` | `.flat_map(\|a\| other.collect(a)).collect()` |
| Find | `List.find_opt pred (f.to_list s)` | `.into_iter().find(pred)` |
| Trait version | Module functor / first-class module | `trait FoldOptic<S, A>` |

## Key Insights

1. **Read-only simplicity**: A Fold has only one primitive — `to_list` / `collect` — compared to a Traversal which also requires `over`. This halves the implementation complexity while retaining all read combinators.

2. **Composition is flat-map**: OCaml's `List.concat_map` and Rust's `.flat_map()` are the same operation — the outer fold extracts a list of `A`, the inner fold maps each `A` to a list of `B`, and the results are concatenated. The Fold composition is the optic equivalent of monadic bind on lists.

3. **Specialised impls for numeric aggregation**: Rust's coherence rules require separate `impl<S> Fold<S, i32>` and `impl<S> Fold<S, f64>` blocks for `sum_of`, `max_of`, etc. OCaml handles this via type classes / polymorphic operators without specialisation.

4. **Two encodings — heap vs zero-cost**: The struct-based `Fold<S, A>` uses `Box<dyn Fn>` and pays a heap allocation per fold instance. The trait-based `FoldOptic<S, A>` uses marker structs with compile-time dispatch — no heap allocation, equivalent to C++ template specialisation.

5. **Aggregation for free**: Once `collect` (or `fold_collect`) is defined, every aggregation — `sum`, `max`, `min`, `any`, `all`, `find`, `first`, `last`, `count` — follows from standard iterator methods. The fold focuses; the iterator aggregates. The separation of concerns is the same in both languages.

## When to Use Each Style

**Use struct-based `Fold<S, A>` when:** You need runtime-constructed folds, composable at runtime, or when the focusing logic is passed as a closure. Good for building fold libraries and DSLs where the path is not known at compile time.

**Use trait-based `FoldOptic<S, A>` when:** The focusing logic is fixed at compile time and you want zero-cost abstraction. Marker structs like `TeamMembersFold` compile to the same code as hand-written field access — no indirection, no heap allocation.
