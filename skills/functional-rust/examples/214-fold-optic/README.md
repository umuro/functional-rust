# 214: Fold Optic

**Difficulty:** 3  **Level:** Advanced

A read-only optic that focuses on multiple values and aggregates them — without modifying the structure.

## The Problem This Solves

A Lens focuses on exactly one field. A Traversal focuses on multiple fields and can update them. But sometimes you only need to *read* multiple values — sum them, count them, find the maximum, check whether any satisfy a condition. You don't need the write half.

A Fold is a read-only, multi-focus optic. It extracts a collection of values from a structure and provides combinators to aggregate them: `sum_of`, `length_of`, `any_of`, `all_of`, `find_of`. You build the fold once, and all aggregations share the same focusing logic.

The real power emerges with composition: a `Fold<Team, i32>` focused on member scores can be built by composing a `Fold<Team, Member>` with a `Fold<Member, i32>`. One composed fold, all aggregations across nested data.

## The Intuition

A Fold is essentially "give me a list of all the values you focus on, and I'll aggregate them for you." The `to_list` function inside the Fold does the focusing; the combinators (`sum_of`, `max_of`, etc.) do the aggregation.

Think of a SQL `SELECT` with `WHERE` and `GROUP BY`. The Fold is the `SELECT ... FROM structure` part — it finds the values. The combinators are `SUM()`, `COUNT()`, `MAX()`. You write the path once; you get all the aggregate functions for free.

Composition is flat-map: if focusing on a `Team` gives you `Member`s, and focusing on a `Member` gives you `i32` scores, composing gives you all scores in a team.

## How It Works in Rust

```rust
struct Fold<S, A> {
    to_list: Box<dyn Fn(&S) -> Vec<A>>,  // extract all focused values
}

impl<S: 'static, A: 'static> Fold<S, A> {
    fn collect(&self, s: &S) -> Vec<A> { (self.to_list)(s) }
    fn length_of(&self, s: &S) -> usize { self.collect(s).len() }

    fn any_of(&self, pred: impl Fn(&A) -> bool, s: &S) -> bool {
        self.collect(s).iter().any(|a| pred(a))
    }

    fn find_of(&self, pred: impl Fn(&A) -> bool, s: &S) -> Option<A>
    where A: Clone {
        self.collect(s).into_iter().find(|a| pred(a))
    }

    // Compose: Fold<S,A> + Fold<A,B> → Fold<S,B>
    fn compose_fold<B: 'static>(self, inner: Fold<A, B>) -> Fold<S, B>
    where A: Clone {
        Fold::new(move |s| {
            self.collect(s).iter()
                .flat_map(|a| inner.collect(a))  // focus into each A, collect Bs
                .collect()
        })
    }
}

// Numeric aggregators as free functions
fn sum_of(fold: &Fold<impl Sized, i32>, s: &impl Sized) -> i32 {
    fold.collect(s).iter().sum()
}

fn max_of<S>(fold: &Fold<S, i32>, s: &S) -> Option<i32> {
    fold.collect(s).into_iter().max()
}

// Usage: sum scores across a team
let member_scores: Fold<Member, i32> = Fold::new(|m| m.scores.clone());
let team_members: Fold<Team, Member> = Fold::new(|t| t.members.clone());
let team_scores = team_members.compose_fold(member_scores);
let total = sum_of(&team_scores, &my_team);
```

## What This Unlocks

- **Aggregation pipelines** — sum, count, max, any, all over nested collections without manual nested loops.
- **Read-only traversals** — when you need to inspect but not modify a structure, a Fold is lighter than a full Traversal.
- **Analytics over domain models** — compose folds from team → member → score → bonus; each level reuses existing fold definitions.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| `to_list` | Returns OCaml list | Returns `Vec<A>` |
| Aggregation | `List.fold_left`, `List.exists` | `.iter().sum()`, `.iter().any()` |
| Composition | `List.concat_map` | `.flat_map()` |
| Max/Min | Manual fold with `compare` | `.into_iter().max()` returns `Option` |
| Empty handling | Empty list → neutral element | `max()` returns `None` on empty |
