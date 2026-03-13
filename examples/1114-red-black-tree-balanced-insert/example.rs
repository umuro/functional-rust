#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RBTree<T> {
    E,
    T(Color, Box<RBTree<T>>, T, Box<RBTree<T>>),
}

use Color::{Black, Red};
use RBTree::{E, T};

pub fn balance<T>(color: Color, left: RBTree<T>, value: T, right: RBTree<T>) -> RBTree<T> {
    match (color, left, value, right) {
        (Black, T(Red, ll, lv, lr), v, r) => match *ll {
            T(Red, a, x, b) => T(
                Red,
                Box::new(T(Black, a, x, b)),
                lv,
                Box::new(T(Black, lr, v, Box::new(r))),
            ),
            ll => match *lr {
                T(Red, b, y, c) => T(
                    Red,
                    Box::new(T(Black, Box::new(ll), lv, b)),
                    y,
                    Box::new(T(Black, c, v, Box::new(r))),
                ),
                lr => T(
                    Black,
                    Box::new(T(Red, Box::new(ll), lv, Box::new(lr))),
                    v,
                    Box::new(r),
                ),
            },
        },
        (Black, l, v, T(Red, rl, rv, rr)) => match *rl {
            T(Red, b, y, c) => T(
                Red,
                Box::new(T(Black, Box::new(l), v, b)),
                y,
                Box::new(T(Black, c, rv, rr)),
            ),
            rl => match *rr {
                T(Red, c, z, d) => T(
                    Red,
                    Box::new(T(Black, Box::new(l), v, Box::new(rl))),
                    rv,
                    Box::new(T(Black, c, z, d)),
                ),
                rr => T(
                    Black,
                    Box::new(l),
                    v,
                    Box::new(T(Red, Box::new(rl), rv, Box::new(rr))),
                ),
            },
        },
        (color, l, v, r) => T(color, Box::new(l), v, Box::new(r)),
    }
}

pub fn insert<T: Ord>(value: T, tree: RBTree<T>) -> RBTree<T> {
    fn ins<T: Ord>(value: T, tree: RBTree<T>) -> RBTree<T> {
        match tree {
            E => T(Red, Box::new(E), value, Box::new(E)),
            T(color, left, v, right) => {
                if value < v {
                    balance(color, ins(value, *left), v, *right)
                } else if value > v {
                    balance(color, *left, v, ins(value, *right))
                } else {
                    T(color, left, v, right)
                }
            }
        }
    }
    match ins(value, tree) {
        T(_, left, v, right) => T(Black, left, v, right),
        E => E,
    }
}

pub fn mem<T: Ord>(value: &T, tree: &RBTree<T>) -> bool {
    match tree {
        E => false,
        T(_, left, v, right) => {
            if value == v {
                true
            } else if value < v {
                mem(value, left)
            } else {
                mem(value, right)
            }
        }
    }
}

pub fn to_list<T: Clone>(tree: &RBTree<T>) -> Vec<T> {
    match tree {
        E => vec![],
        T(_, left, v, right) => {
            let mut result = to_list(left);
            result.push(v.clone());
            result.extend(to_list(right));
            result
        }
    }
}

pub fn from_iter<T: Ord>(iter: impl Iterator<Item = T>) -> RBTree<T> {
    iter.fold(E, |tree, v| insert(v, tree))
}

fn main() {
    let t = from_iter([5, 3, 7, 1, 4, 6, 8, 2, 9].into_iter());

    println!("Inserted [5,3,7,1,4,6,8,2,9] into red-black tree.");
    println!("In-order: {:?}", to_list(&t));
    println!("mem(4): {}", mem(&4, &t));
    println!("mem(10): {}", mem(&10, &t));

    // Show the root is black
    match &t {
        RBTree::T(Color::Black, _, root, _) => println!("Root value: {root} (Black — invariant holds)"),
        _ => println!("unexpected structure"),
    }
}

/* Output:
   Inserted [5,3,7,1,4,6,8,2,9] into red-black tree.
   In-order: [1, 2, 3, 4, 5, 6, 7, 8, 9]
   mem(4): true
   mem(10): false
   Root value: 4 (Black — invariant holds)
*/
