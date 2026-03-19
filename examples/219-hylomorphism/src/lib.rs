// Example 219: Hylomorphism — Ana then Cata, Fused

// hylo: unfold a seed, then fold the result. No intermediate structure!

#[derive(Debug)]
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
}

fn hylo<S, A>(alg: &dyn Fn(ListF<A>) -> A, coalg: &dyn Fn(S) -> ListF<S>, seed: S) -> A {
    alg(coalg(seed).map(|s| hylo(alg, coalg, s)))
}

// Approach 1: Factorial
fn factorial(n: i64) -> i64 {
    hylo(
        &|l| match l {
            ListF::NilF => 1,
            ListF::ConsF(n, acc) => n * acc,
        },
        &|n| {
            if n <= 0 {
                ListF::NilF
            } else {
                ListF::ConsF(n, n - 1)
            }
        },
        n,
    )
}

// Approach 2: Sum of range
fn sum_range(n: i64) -> i64 {
    hylo(
        &|l| match l {
            ListF::NilF => 0,
            ListF::ConsF(x, acc) => x + acc,
        },
        &|n| {
            if n <= 0 {
                ListF::NilF
            } else {
                ListF::ConsF(n, n - 1)
            }
        },
        n,
    )
}

// Approach 3: Merge sort via tree hylo
#[derive(Debug)]
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
}

fn hylo_tree<S, A>(alg: &dyn Fn(TreeF<A>) -> A, coalg: &dyn Fn(S) -> TreeF<S>, seed: S) -> A {
    alg(coalg(seed).map(|s| hylo_tree(alg, coalg, s)))
}

fn merge(xs: &[i64], ys: &[i64]) -> Vec<i64> {
    let (mut i, mut j) = (0, 0);
    let mut result = Vec::with_capacity(xs.len() + ys.len());
    while i < xs.len() && j < ys.len() {
        if xs[i] <= ys[j] {
            result.push(xs[i]);
            i += 1;
        } else {
            result.push(ys[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&xs[i..]);
    result.extend_from_slice(&ys[j..]);
    result
}

fn merge_sort(xs: Vec<i64>) -> Vec<i64> {
    if xs.is_empty() {
        return vec![];
    }
    hylo_tree(
        &|t| match t {
            TreeF::LeafF(n) => vec![n],
            TreeF::BranchF(l, r) => merge(&l, &r),
        },
        &|xs: Vec<i64>| {
            if xs.len() <= 1 {
                TreeF::LeafF(xs[0])
            } else {
                let mid = xs.len() / 2;
                TreeF::BranchF(xs[..mid].to_vec(), xs[mid..].to_vec())
            }
        },
        xs,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(6), 720);
    }
    #[test]
    fn test_sum_range() {
        assert_eq!(sum_range(5), 15);
    }
    #[test]
    fn test_merge_sort() {
        assert_eq!(merge_sort(vec![9, 7, 5, 3, 1]), vec![1, 3, 5, 7, 9]);
    }
}
