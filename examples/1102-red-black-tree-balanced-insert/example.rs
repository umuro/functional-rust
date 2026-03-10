//! 1102: Red-Black Tree with Okasaki's Functional Balancing
//!
//! A purely functional red-black tree. Insert returns a new tree, sharing
//! structure with the old one. Balance is maintained by Okasaki's elegant
//! four-case pattern-matching rewrite rule.

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RbTree<T> {
    E,
    T(Color, Box<RbTree<T>>, T, Box<RbTree<T>>),
}

impl<T: Ord + Clone> RbTree<T> {
    pub fn empty() -> Self {
        RbTree::E
    }

    pub fn insert(&self, x: T) -> Self {
        match self.ins(&x) {
            RbTree::T(_, a, y, b) => RbTree::T(Color::Black, a, y, b),
            RbTree::E => RbTree::E,
        }
    }

    fn ins(&self, x: &T) -> Self {
        match self {
            RbTree::E => RbTree::T(
                Color::Red,
                Box::new(RbTree::E),
                x.clone(),
                Box::new(RbTree::E),
            ),
            RbTree::T(color, a, y, b) => {
                if x < y {
                    balance(color.clone(), a.ins(x), y.clone(), (**b).clone())
                } else if x > y {
                    balance(color.clone(), (**a).clone(), y.clone(), b.ins(x))
                } else {
                    self.clone()
                }
            }
        }
    }

    pub fn member(&self, x: &T) -> bool {
        match self {
            RbTree::E => false,
            RbTree::T(_, a, y, b) => {
                if x == y {
                    true
                } else if x < y {
                    a.member(x)
                } else {
                    b.member(x)
                }
            }
        }
    }

    pub fn to_sorted_vec(&self) -> Vec<T> {
        match self {
            RbTree::E => vec![],
            RbTree::T(_, a, v, b) => {
                let mut out = a.to_sorted_vec();
                out.push(v.clone());
                out.extend(b.to_sorted_vec());
                out
            }
        }
    }
}

impl<T: Ord + Clone> FromIterator<T> for RbTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().fold(RbTree::empty(), |t, x| t.insert(x))
    }
}

fn balance<T: Clone>(color: Color, left: RbTree<T>, val: T, right: RbTree<T>) -> RbTree<T> {
    use Color::{Black, Red};

    if color == Black {
        // Case 1: left-left
        if let RbTree::T(Red, ref ll, ref lv, ref lr) = left {
            if let RbTree::T(Red, ref a, ref x, ref b) = **ll {
                return RbTree::T(
                    Red,
                    Box::new(RbTree::T(Black, a.clone(), x.clone(), b.clone())),
                    lv.clone(),
                    Box::new(RbTree::T(Black, lr.clone(), val, Box::new(right))),
                );
            }
            // Case 2: left-right
            if let RbTree::T(Red, ref b, ref y, ref c) = **lr {
                return RbTree::T(
                    Red,
                    Box::new(RbTree::T(Black, ll.clone(), lv.clone(), b.clone())),
                    y.clone(),
                    Box::new(RbTree::T(Black, c.clone(), val, Box::new(right))),
                );
            }
        }
        // Case 3: right-left
        if let RbTree::T(Red, ref rl, ref rv, ref rr) = right {
            if let RbTree::T(Red, ref b, ref y, ref c) = **rl {
                return RbTree::T(
                    Red,
                    Box::new(RbTree::T(Black, Box::new(left), val, b.clone())),
                    y.clone(),
                    Box::new(RbTree::T(Black, c.clone(), rv.clone(), rr.clone())),
                );
            }
            // Case 4: right-right
            if let RbTree::T(Red, ref c, ref z, ref d) = **rr {
                return RbTree::T(
                    Red,
                    Box::new(RbTree::T(Black, Box::new(left), val, rl.clone())),
                    rv.clone(),
                    Box::new(RbTree::T(Black, c.clone(), z.clone(), d.clone())),
                );
            }
        }
    }

    RbTree::T(color, Box::new(left), val, Box::new(right))
}

fn main() {
    // Build tree by folding inserts — mirrors OCaml's List.fold_left
    let t: RbTree<i32> = [5, 3, 7, 1, 4, 6, 8, 2, 9].into_iter().collect();

    println!("Sorted: {:?}", t.to_sorted_vec());
    println!("member(4)  = {}", t.member(&4));
    println!("member(10) = {}", t.member(&10));

    // Ascending insert — worst case for naive BST, RB tree stays O(log n)
    let t2: RbTree<i32> = (1..=8).collect();
    println!("Ascending 1..=8: {:?}", t2.to_sorted_vec());
}

/* Output:
   Sorted: [1, 2, 3, 4, 5, 6, 7, 8, 9]
   member(4)  = true
   member(10) = false
   Ascending 1..=8: [1, 2, 3, 4, 5, 6, 7, 8]
*/
