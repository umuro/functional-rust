/// Red-Black Tree with Okasaki's Functional Balancing

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RBTree<T> {
    E,
    T(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{E, T};

fn balance<V: Ord>(color: Color, left: RBTree<V>, val: V, right: RBTree<V>) -> RBTree<V> {
    fn balanced<V>(
        a: RBTree<V>,
        x: V,
        b: RBTree<V>,
        y: V,
        c: RBTree<V>,
        z: V,
        d: RBTree<V>,
    ) -> RBTree<V> {
        T(
            Red,
            Box::new(T(Black, Box::new(a), x, Box::new(b))),
            y,
            Box::new(T(Black, Box::new(c), z, Box::new(d))),
        )
    }

    match (color, left, val, right) {
        (Black, T(Red, ll, y, c), z, d) if matches!(*ll, T(Red, ..)) => {
            let T(_, a, x, b) = *ll else { unreachable!() };
            balanced(*a, x, *b, y, *c, z, d)
        }
        (Black, T(Red, a, x, lr), z, d) if matches!(*lr, T(Red, ..)) => {
            let T(_, b, y, c) = *lr else { unreachable!() };
            balanced(*a, x, *b, y, *c, z, d)
        }
        (Black, a, x, T(Red, rl, z, d)) if matches!(*rl, T(Red, ..)) => {
            let T(_, b, y, c) = *rl else { unreachable!() };
            balanced(a, x, *b, y, *c, z, *d)
        }
        (Black, a, x, T(Red, b, y, rr)) if matches!(*rr, T(Red, ..)) => {
            let T(_, c, z, d) = *rr else { unreachable!() };
            balanced(a, x, *b, y, *c, z, *d)
        }
        (color, a, x, b) => T(color, Box::new(a), x, Box::new(b)),
    }
}

pub fn insert<V: Ord>(value: V, tree: RBTree<V>) -> RBTree<V> {
    fn ins<V: Ord>(x: V, tree: RBTree<V>) -> RBTree<V> {
        match tree {
            E => T(Red, Box::new(E), x, Box::new(E)),
            T(color, left, y, right) => {
                if x < y {
                    balance(color, ins(x, *left), y, *right)
                } else if x > y {
                    balance(color, *left, y, ins(x, *right))
                } else {
                    T(color, left, y, right)
                }
            }
        }
    }

    match ins(value, tree) {
        T(_, left, y, right) => T(Black, left, y, right),
        E => E,
    }
}

pub fn mem<V: Ord>(value: &V, tree: &RBTree<V>) -> bool {
    match tree {
        E => false,
        T(_, left, y, right) => {
            if value == y {
                true
            } else if value < y {
                mem(value, left)
            } else {
                mem(value, right)
            }
        }
    }
}

pub fn to_sorted_vec<V: Clone>(tree: &RBTree<V>) -> Vec<V> {
    fn collect<V: Clone>(tree: &RBTree<V>, acc: &mut Vec<V>) {
        match tree {
            E => {}
            T(_, left, v, right) => {
                collect(left, acc);
                acc.push(v.clone());
                collect(right, acc);
            }
        }
    }
    let mut result = Vec::new();
    collect(tree, &mut result);
    result
}

pub fn from_iter<V: Ord>(iter: impl IntoIterator<Item = V>) -> RBTree<V> {
    iter.into_iter().fold(E, |tree, x| insert(x, tree))
}

fn main() {
    // Build a tree from [5,3,7,1,4,6,8,2,9] — same as the OCaml example
    let tree = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9]);

    println!("In-order: {:?}", to_sorted_vec(&tree));

    println!("mem(5) = {}", mem(&5, &tree));
    println!("mem(0) = {}", mem(&0, &tree));

    // Insert duplicates — should not change the tree
    let tree2 = insert(5, insert(3, tree));
    println!("After dup inserts: {:?}", to_sorted_vec(&tree2));
}

/* Output:
   In-order: [1, 2, 3, 4, 5, 6, 7, 8, 9]
   mem(5) = true
   mem(0) = false
   After dup inserts: [1, 2, 3, 4, 5, 6, 7, 8, 9]
*/
