#![allow(clippy::all)]
// Example 218: Anamorphism — Unfold to Build Recursive Structures

// ana : (seed -> F<seed>) -> seed -> Fix<F>

#[derive(Debug, Clone)]
enum ListF<A> {
    NilF,
    ConsF(i64, A),
}

impl<A> ListF<A> {
    fn map<B>(self, f: impl Fn(A) -> B) -> ListF<B> {
        match self {
            ListF::NilF => ListF::NilF,
            ListF::ConsF(x, a) => ListF::ConsF(x, f(a)),
        }
    }
    fn map_ref<B>(&self, f: impl Fn(&A) -> B) -> ListF<B> {
        match self {
            ListF::NilF => ListF::NilF,
            ListF::ConsF(x, a) => ListF::ConsF(*x, f(a)),
        }
    }
}

#[derive(Debug, Clone)]
struct FixList(Box<ListF<FixList>>);

fn ana<S>(coalg: &dyn Fn(S) -> ListF<S>, seed: S) -> FixList {
    FixList(Box::new(coalg(seed).map(|s| ana(coalg, s))))
}

fn cata<A>(alg: &dyn Fn(ListF<A>) -> A, FixList(f): &FixList) -> A {
    alg(f.map_ref(|child| cata(alg, child)))
}

fn to_vec(fl: &FixList) -> Vec<i64> {
    cata(
        &|l| match l {
            ListF::NilF => vec![],
            ListF::ConsF(x, mut acc) => {
                acc.insert(0, x);
                acc
            }
        },
        fl,
    )
}

// Approach 1: Range [lo..=hi]
fn range(lo: i64, hi: i64) -> FixList {
    ana(
        &|s: (i64, i64)| {
            if s.0 > s.1 {
                ListF::NilF
            } else {
                ListF::ConsF(s.0, (s.0 + 1, s.1))
            }
        },
        (lo, hi),
    )
}

// Approach 2: Countdown
fn countdown(n: i64) -> FixList {
    ana(
        &|s| {
            if s <= 0 {
                ListF::NilF
            } else {
                ListF::ConsF(s, s - 1)
            }
        },
        n,
    )
}

// Approach 3: Collatz sequence
fn collatz(n: i64) -> FixList {
    ana(
        &|s| {
            if s <= 0 {
                ListF::NilF
            } else if s == 1 {
                ListF::ConsF(1, 0)
            } else if s % 2 == 0 {
                ListF::ConsF(s, s / 2)
            } else {
                ListF::ConsF(s, 3 * s + 1)
            }
        },
        n,
    )
}

// Tree anamorphism
#[derive(Debug, Clone)]
enum TreeF<A> {
    LeafF(i64),
    BranchF(A, A),
}

impl<A> TreeF<A> {
    fn map<B>(self, f: impl Fn(A) -> B) -> TreeF<B> {
        match self {
            TreeF::LeafF(n) => TreeF::LeafF(n),
            TreeF::BranchF(l, r) => TreeF::BranchF(f(l), f(r)),
        }
    }
    fn map_ref<B>(&self, f: impl Fn(&A) -> B) -> TreeF<B> {
        match self {
            TreeF::LeafF(n) => TreeF::LeafF(*n),
            TreeF::BranchF(l, r) => TreeF::BranchF(f(l), f(r)),
        }
    }
}

#[derive(Debug, Clone)]
struct FixTree(Box<TreeF<FixTree>>);

fn ana_tree<S>(coalg: &dyn Fn(S) -> TreeF<S>, seed: S) -> FixTree {
    FixTree(Box::new(coalg(seed).map(|s| ana_tree(coalg, s))))
}

fn balanced_tree(depth: u32) -> FixTree {
    ana_tree(
        &|s: (u32, i64)| {
            if s.0 == 0 {
                TreeF::LeafF(s.1)
            } else {
                TreeF::BranchF((s.0 - 1, s.1), (s.0 - 1, s.1 + (1i64 << (s.0 - 1))))
            }
        },
        (depth, 1),
    )
}

fn tree_to_vec(t: &FixTree) -> Vec<i64> {
    fn go(t: &FixTree) -> Vec<i64> {
        match t.0.as_ref() {
            TreeF::LeafF(n) => vec![*n],
            TreeF::BranchF(l, r) => {
                let mut v = go(l);
                v.extend(go(r));
                v
            }
        }
    }
    go(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        assert_eq!(to_vec(&range(1, 3)), vec![1, 2, 3]);
    }

    #[test]
    fn test_countdown() {
        assert_eq!(to_vec(&countdown(3)), vec![3, 2, 1]);
    }

    #[test]
    fn test_collatz_1() {
        assert_eq!(to_vec(&collatz(1)), vec![1]);
    }

    #[test]
    fn test_balanced_tree() {
        assert_eq!(tree_to_vec(&balanced_tree(1)), vec![1, 2]);
    }
}
