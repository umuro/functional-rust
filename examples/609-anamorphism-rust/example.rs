// Anamorphism: generate a structure from a seed

// List anamorphism (unfold)
fn unfold_list<S: Clone, A>(seed: S, coalg: impl Fn(S) -> Option<(A, S)>) -> Vec<A> {
    let mut result = Vec::new();
    let mut state  = seed;
    while let Some((item, next)) = coalg(state.clone()) {
        result.push(item);
        state = next;
    }
    result
}

// Tree anamorphism
#[derive(Debug)]
enum Tree<A> { Leaf, Node { val: A, left: Box<Tree<A>>, right: Box<Tree<A>> } }

fn unfold_tree<S: Clone, A>(seed: S, coalg: impl Fn(S) -> Option<(A, S, S)> + Copy) -> Tree<A> {
    match coalg(seed) {
        None => Tree::Leaf,
        Some((val, l_seed, r_seed)) => Tree::Node {
            val,
            left:  Box::new(unfold_tree(l_seed, coalg)),
            right: Box::new(unfold_tree(r_seed, coalg)),
        }
    }
}

fn tree_to_list<A: Clone>(t: &Tree<A>) -> Vec<A> {
    match t {
        Tree::Leaf => vec![],
        Tree::Node { val, left, right } => {
            let mut v = tree_to_list(left);
            v.push(val.clone());
            v.extend(tree_to_list(right));
            v
        }
    }
}

fn main() {
    // Range via unfold
    let range = unfold_list(1, |i| if i > 5 { None } else { Some((i, i+1)) });
    println!("range: {:?}", range);

    // Fibonacci via unfold
    let fibs = unfold_list((0u64,1u64), |(a,b)| if a > 100 { None } else { Some((a,(b,a+b))) });
    println!("fibs: {:?}", fibs);

    // Digits of a number
    let digits = unfold_list(1234u32, |n| if n == 0 { None } else { Some((n%10, n/10)) });
    println!("digits(1234) reversed: {:?}", digits);

    // BST construction via anamorphism
    let bst: Tree<i32> = unfold_tree(1..=7, |mut range| {
        let v: Vec<i32> = range.clone().collect();
        let mid = v.len() / 2;
        if v.is_empty() { None }
        else {
            let lv: Vec<_> = v[..mid].iter().copied().collect();
            let rv: Vec<_> = v[mid+1..].iter().copied().collect();
            Some((v[mid], lv.into_iter(), rv.into_iter()))
        }
    });
    println!("BST inorder: {:?}", tree_to_list(&bst));

    // Iterator as anamorphism
    let powers_of_2: Vec<u64> = std::iter::successors(Some(1u64), |&n| n.checked_mul(2))
        .take(10).collect();
    println!("powers of 2: {:?}", powers_of_2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn range_unfold() {
        let r = unfold_list(0, |i| if i>=5 {None} else {Some((i,i+1))});
        assert_eq!(r, vec![0,1,2,3,4]);
    }
    #[test] fn fib_count() {
        let f = unfold_list((0u64,1u64), |(a,b)| if a>50{None}else{Some((a,(b,a+b)))});
        assert!(f.len() > 0 && f[0] == 0);
    }
}
