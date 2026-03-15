#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RbTree<T> {
    E,
    T(Color, Box<RbTree<T>>, T, Box<RbTree<T>>),
}

impl<T: Ord + Clone> RbTree<T> {
    pub fn empty() -> Self {
        RbTree::E
    }

    pub fn insert(&self, x: T) -> Self {
        let result = self.ins(&x);
        match result {
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
                    balance(color.clone(), a.ins(x), y.clone(), *b.clone())
                } else if x > y {
                    balance(color.clone(), *a.clone(), y.clone(), b.ins(x))
                } else {
                    self.clone()
                }
            }
        }
    }

    pub fn mem(&self, x: &T) -> bool {
        match self {
            RbTree::E => false,
            RbTree::T(_, a, y, b) => x == y || if x < y { a.mem(x) } else { b.mem(x) },
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self {
            RbTree::E => vec![],
            RbTree::T(_, a, v, b) => {
                let mut out = a.to_vec();
                out.push(v.clone());
                out.extend(b.to_vec());
                out
            }
        }
    }

    pub fn build(iter: impl IntoIterator<Item = T>) -> Self {
        iter.into_iter()
            .fold(RbTree::empty(), |tree, x| tree.insert(x))
    }
}

fn balance<T: Clone>(color: Color, left: RbTree<T>, z: T, right: RbTree<T>) -> RbTree<T> {
    use Color::{Black, Red};
    use RbTree::T;

    if color != Black {
        return RbTree::T(color, Box::new(left), z, Box::new(right));
    }

    if let T(Red, ll, y_val, c_val) = &left {
        if let T(Red, a, x_val, b) = ll.as_ref() {
            return RbTree::T(
                Red,
                Box::new(T(Black, a.clone(), x_val.clone(), b.clone())),
                y_val.clone(),
                Box::new(T(Black, c_val.clone(), z, Box::new(right))),
            );
        }
        if let T(Red, b, y2_val, c2_val) = c_val.as_ref() {
            return RbTree::T(
                Red,
                Box::new(T(Black, ll.clone(), y_val.clone(), b.clone())),
                y2_val.clone(),
                Box::new(T(Black, c2_val.clone(), z, Box::new(right))),
            );
        }
    }

    if let T(Red, rl, y_val, d_val) = &right {
        if let T(Red, b, y2_val, c_val) = rl.as_ref() {
            return RbTree::T(
                Red,
                Box::new(T(Black, Box::new(left), z, b.clone())),
                y2_val.clone(),
                Box::new(T(Black, c_val.clone(), y_val.clone(), d_val.clone())),
            );
        }
        if let T(Red, c_val, z2_val, d2_val) = d_val.as_ref() {
            return RbTree::T(
                Red,
                Box::new(T(Black, Box::new(left), z, rl.clone())),
                y_val.clone(),
                Box::new(T(Black, c_val.clone(), z2_val.clone(), d2_val.clone())),
            );
        }
    }

    RbTree::T(color, Box::new(left), z, Box::new(right))
}

fn main() {
    let t = RbTree::build([5, 3, 7, 1, 4, 6, 8, 2, 9]);
    println!("sorted:     {:?}", t.to_vec());
    println!("mem(5):     {}", t.mem(&5));
    println!("mem(10):    {}", t.mem(&10));

    let asc = RbTree::build(1..=10i32);
    println!("ascending:  {:?}", asc.to_vec());

    let desc = RbTree::build((1..=10i32).rev());
    println!("descending: {:?}", desc.to_vec());
}

/* Output:
   sorted:     [1, 2, 3, 4, 5, 6, 7, 8, 9]
   mem(5):     true
   mem(10):    false
   ascending:  [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
   descending: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
*/
