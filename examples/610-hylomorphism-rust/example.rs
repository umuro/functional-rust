// Hylomorphism = anamorphism followed by catamorphism
// hylo(coalg, alg, seed) = cata(alg, ana(coalg, seed))

// General hylo on Option-based unfold + fold
fn hylo<S: Clone, A, R>(
    seed: S,
    coalg: impl Fn(S) -> Option<(A, S)> + Copy,
    alg:   impl Fn(A, R) -> R,
    base:  R,
) -> R {
    match coalg(seed) {
        None             => base,
        Some((a, next)) => alg(a, hylo(next, coalg, alg, base)),
    }
}

// Factorial as hylo
fn factorial(n: u64) -> u64 {
    hylo(n,
        |k| if k <= 1 { None } else { Some((k, k-1)) },
        |k, acc| k * acc,
        1,
    )
}

// Sum via hylo
fn sum_to(n: u64) -> u64 {
    hylo(n,
        |k| if k == 0 { None } else { Some((k, k-1)) },
        |k, acc| k + acc,
        0,
    )
}

// Merge sort as hylomorphism over a binary tree
#[derive(Debug)]
enum SortTree<A> { Leaf(A), Branch(Box<SortTree<A>>, Box<SortTree<A>>) }

fn split_to_tree<A: Clone>(xs: &[A]) -> Option<SortTree<A>> {
    match xs {
        [] => None,
        [x] => Some(SortTree::Leaf(x.clone())),
        _ => {
            let mid = xs.len() / 2;
            let l = split_to_tree(&xs[..mid]);
            let r = split_to_tree(&xs[mid..]);
            match (l, r) {
                (Some(l), Some(r)) => Some(SortTree::Branch(Box::new(l), Box::new(r))),
                (Some(l), None) | (None, Some(l)) => Some(l),
                _ => None,
            }
        }
    }
}

fn merge_sorted<A: Ord + Clone>(mut a: Vec<A>, mut b: Vec<A>) -> Vec<A> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] { result.push(a[i].clone()); i+=1; }
        else             { result.push(b[j].clone()); j+=1; }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}

fn merge_sort<A: Ord + Clone>(xs: &[A]) -> Vec<A> {
    match split_to_tree(xs) {
        None => vec![],
        Some(tree) => {
            fn fold_sort<A: Ord+Clone>(t: SortTree<A>) -> Vec<A> {
                match t {
                    SortTree::Leaf(x)       => vec![x],
                    SortTree::Branch(l,r)   => merge_sorted(fold_sort(*l), fold_sort(*r)),
                }
            }
            fold_sort(tree)
        }
    }
}

fn main() {
    println!("5! = {}", factorial(5));
    println!("10! = {}", factorial(10));
    println!("sum 1..10 = {}", sum_to(10));

    let v = vec![3,1,4,1,5,9,2,6,5,3];
    println!("sorted: {:?}", merge_sort(&v));

    let words = vec!["banana","apple","cherry","date","elderberry"];
    println!("sorted words: {:?}", merge_sort(&words));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fact5()    { assert_eq!(factorial(5), 120); }
    #[test] fn sum10()    { assert_eq!(sum_to(10), 55); }
    #[test] fn sort_test(){ assert_eq!(merge_sort(&[3,1,2]), vec![1,2,3]); }
}
