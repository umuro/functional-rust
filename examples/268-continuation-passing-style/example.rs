// Continuation-Passing Style (CPS)
// Each continuation is Box<dyn FnOnce(T) -> R> — consumed exactly once.

// ---------------------------------------------------------------------------
// Factorial
// ---------------------------------------------------------------------------

pub fn factorial_direct(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial_direct(n - 1)
    }
}

fn factorial_go(n: u64, k: Box<dyn FnOnce(u64) -> u64>) -> u64 {
    if n == 0 {
        k(1)
    } else {
        factorial_go(n - 1, Box::new(move |result| k(n * result)))
    }
}

pub fn factorial_cps(n: u64) -> u64 {
    factorial_go(n, Box::new(|x| x))
}

// ---------------------------------------------------------------------------
// Binary tree — CPS sum
// ---------------------------------------------------------------------------

pub enum Tree<T> {
    Leaf(T),
    Node(Box<Tree<T>>, Box<Tree<T>>),
}

fn sum_go<'a>(t: &'a Tree<i64>, k: Box<dyn FnOnce(i64) -> i64 + 'a>) -> i64 {
    match t {
        Tree::Leaf(x) => k(*x),
        Tree::Node(l, r) => sum_go(
            l,
            Box::new(move |sl| sum_go(r, Box::new(move |sr| k(sl + sr)))),
        ),
    }
}

pub fn sum_cps(t: &Tree<i64>) -> i64 {
    sum_go(t, Box::new(|x| x))
}

pub fn leaf<T>(v: T) -> Tree<T> {
    Tree::Leaf(v)
}

pub fn node<T>(l: Tree<T>, r: Tree<T>) -> Tree<T> {
    Tree::Node(Box::new(l), Box::new(r))
}

// ---------------------------------------------------------------------------
// Demo
// ---------------------------------------------------------------------------

fn main() {
    println!("factorial_direct(10) = {}", factorial_direct(10));
    println!("factorial_cps(10)    = {}", factorial_cps(10));

    // Node(Node(Leaf 1, Leaf 2), Node(Leaf 3, Leaf 4))
    let t = node(
        node(leaf(1_i64), leaf(2_i64)),
        node(leaf(3_i64), leaf(4_i64)),
    );
    println!("sum_cps(tree)        = {}", sum_cps(&t));
}

/* Output:
   factorial_direct(10) = 3628800
   factorial_cps(10)    = 3628800
   sum_cps(tree)        = 10
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_base_case() {
        assert_eq!(factorial_direct(0), 1);
    }

    #[test]
    fn test_direct_small() {
        assert_eq!(factorial_direct(1), 1);
        assert_eq!(factorial_direct(5), 120);
    }

    #[test]
    fn test_direct_ten() {
        assert_eq!(factorial_direct(10), 3_628_800);
    }

    #[test]
    fn test_cps_base_case() {
        assert_eq!(factorial_cps(0), 1);
    }

    #[test]
    fn test_cps_small() {
        assert_eq!(factorial_cps(1), 1);
        assert_eq!(factorial_cps(5), 120);
    }

    #[test]
    fn test_cps_ten() {
        assert_eq!(factorial_cps(10), 3_628_800);
    }

    #[test]
    fn test_cps_matches_direct() {
        for n in 0..=12 {
            assert_eq!(factorial_cps(n), factorial_direct(n));
        }
    }

    #[test]
    fn test_sum_single_leaf() {
        let t = leaf(42_i64);
        assert_eq!(sum_cps(&t), 42);
    }

    #[test]
    fn test_sum_two_leaves() {
        let t = node(leaf(3_i64), leaf(7_i64));
        assert_eq!(sum_cps(&t), 10);
    }

    #[test]
    fn test_sum_ocaml_example() {
        let t = node(
            node(leaf(1_i64), leaf(2_i64)),
            node(leaf(3_i64), leaf(4_i64)),
        );
        assert_eq!(sum_cps(&t), 10);
    }

    #[test]
    fn test_sum_asymmetric_tree() {
        let t = node(leaf(1_i64), node(leaf(2_i64), leaf(3_i64)));
        assert_eq!(sum_cps(&t), 6);
    }

    #[test]
    fn test_sum_negative_values() {
        let t = node(leaf(-5_i64), node(leaf(3_i64), leaf(-2_i64)));
        assert_eq!(sum_cps(&t), -4);
    }
}
