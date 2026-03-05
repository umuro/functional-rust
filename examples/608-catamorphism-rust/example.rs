// General catamorphism over a tree-shaped type

#[derive(Debug,Clone)]
enum Tree<A> { Leaf(A), Branch(Box<Tree<A>>, Box<Tree<A>>) }

// Catamorphism over Tree<A>
fn cata_tree<A: Clone, R>(
    tree: &Tree<A>,
    leaf_alg:   impl Fn(A) -> R + Copy,
    branch_alg: impl Fn(R, R) -> R + Copy,
) -> R {
    match tree {
        Tree::Leaf(a)        => leaf_alg(a.clone()),
        Tree::Branch(l, r)   => branch_alg(cata_tree(l, leaf_alg, branch_alg),
                                            cata_tree(r, leaf_alg, branch_alg)),
    }
}

// Different algebras on the same structure
fn tree_sum(t: &Tree<i64>) -> i64 { cata_tree(t, |x|x, |l,r|l+r) }
fn tree_max(t: &Tree<i64>) -> i64 { cata_tree(t, |x|x, i64::max) }
fn tree_depth(t: &Tree<i64>) -> usize { cata_tree(t, |_|0, |l,r|1+l.max(r)) }
fn tree_to_list(t: &Tree<i64>) -> Vec<i64> { cata_tree(t, |x|vec![x], |mut l,r|{l.extend(r);l}) }

// Nat catamorphism
fn cata_nat<R>(zero: R, succ: impl Fn(R) -> R + Clone, n: u64) -> R {
    if n == 0 { zero } else { succ(cata_nat(zero, succ.clone(), n-1)) }
}

fn main() {
    let t = Tree::Branch(
        Box::new(Tree::Branch(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)))),
        Box::new(Tree::Branch(Box::new(Tree::Leaf(3)), Box::new(Tree::Leaf(4)))),
    );
    println!("sum   = {}", tree_sum(&t));
    println!("max   = {}", tree_max(&t));
    println!("depth = {}", tree_depth(&t));
    println!("list  = {:?}", tree_to_list(&t));

    // Nat cata
    let five_plus_three = cata_nat(3u64, |n|n+1, 5);
    println!("5+3   = {}", five_plus_three);

    // List fold as cata
    let xs = vec![1i64,2,3,4,5];
    let sum: i64 = xs.iter().copied().fold(0, |acc,x| acc+x);
    let prod: i64 = xs.iter().copied().fold(1, |acc,x| acc*x);
    println!("sum   = {}", sum);
    println!("prod  = {}", prod);
}

#[cfg(test)]
mod tests {
    use super::*;
    fn sample() -> Tree<i64> {
        Tree::Branch(Box::new(Tree::Leaf(3)), Box::new(Tree::Leaf(4)))
    }
    #[test] fn sum()   { assert_eq!(tree_sum(&sample()), 7); }
    #[test] fn max()   { assert_eq!(tree_max(&sample()), 4); }
    #[test] fn depth() { assert_eq!(tree_depth(&sample()), 1); }
}
