#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T> Tree<T> {
    pub fn node(left: Tree<T>, val: T, right: Tree<T>) -> Self {
        Tree::Node(Box::new(left), val, Box::new(right))
    }
}

#[derive(Debug, Clone)]
pub enum Crumb<T> {
    Left(T, Tree<T>),
    Right(Tree<T>, T),
}

#[derive(Debug, Clone)]
pub struct Zipper<T> {
    pub focus: Tree<T>,
    pub trail: Vec<Crumb<T>>,
}

pub fn of_tree<T>(tree: Tree<T>) -> Zipper<T> {
    Zipper {
        focus: tree,
        trail: Vec::new(),
    }
}

pub fn go_left<T>(mut z: Zipper<T>) -> Option<Zipper<T>> {
    match z.focus {
        Tree::Leaf => None,
        Tree::Node(l, v, r) => {
            z.trail.push(Crumb::Left(v, *r));
            Some(Zipper {
                focus: *l,
                trail: z.trail,
            })
        }
    }
}

pub fn go_right<T>(mut z: Zipper<T>) -> Option<Zipper<T>> {
    match z.focus {
        Tree::Leaf => None,
        Tree::Node(l, v, r) => {
            z.trail.push(Crumb::Right(*l, v));
            Some(Zipper {
                focus: *r,
                trail: z.trail,
            })
        }
    }
}

pub fn go_up<T>(mut z: Zipper<T>) -> Option<Zipper<T>> {
    match z.trail.pop() {
        None => None,
        Some(Crumb::Left(v, r)) => Some(Zipper {
            focus: Tree::node(z.focus, v, r),
            trail: z.trail,
        }),
        Some(Crumb::Right(l, v)) => Some(Zipper {
            focus: Tree::node(l, v, z.focus),
            trail: z.trail,
        }),
    }
}

pub fn set_value<T>(x: T, z: Zipper<T>) -> Zipper<T> {
    match z.focus {
        Tree::Leaf => z,
        Tree::Node(l, _, r) => Zipper {
            focus: Tree::node(*l, x, *r),
            trail: z.trail,
        },
    }
}

pub fn to_tree<T>(mut z: Zipper<T>) -> Tree<T> {
    loop {
        if z.trail.is_empty() {
            return z.focus;
        }
        z = go_up(z).expect("trail was non-empty");
    }
}

pub fn to_tree_recursive<T>(z: Zipper<T>) -> Tree<T> {
    if z.trail.is_empty() {
        return z.focus;
    }
    to_tree_recursive(go_up(z).expect("trail was non-empty"))
}

fn main() {
    // Build: Node(Node(Leaf,1,Leaf), 2, Node(Leaf,3,Leaf))
    let tree = Tree::node(
        Tree::node(Tree::Leaf, 1, Tree::Leaf),
        2,
        Tree::node(Tree::Leaf, 3, Tree::Leaf),
    );

    println!("Original tree: {:?}", tree);

    // Navigate to the left child and edit it
    let z = of_tree(tree);
    let z = go_left(z).expect("left child exists");
    println!("Focused on left child: {:?}", z.focus);

    let z = set_value(10, z);
    println!("After set_value(10): {:?}", z.focus);

    let result = to_tree(z);
    println!("Rebuilt tree: {:?}", result);

    // Show right navigation
    let tree2 = Tree::node(
        Tree::node(Tree::Leaf, 1, Tree::Leaf),
        2,
        Tree::node(Tree::Leaf, 3, Tree::Leaf),
    );
    let z2 = of_tree(tree2);
    let z2 = go_right(z2).expect("right child exists");
    let z2 = set_value(30, z2);
    let result2 = to_tree_recursive(z2);
    println!("After editing right child to 30 (recursive): {:?}", result2);
}

/* Output:
   Original tree: Node(Node(Leaf, 1, Leaf), 2, Node(Leaf, 3, Leaf))
   Focused on left child: Node(Leaf, 1, Leaf)
   After set_value(10): Node(Leaf, 10, Leaf)
   Rebuilt tree: Node(Node(Leaf, 10, Leaf), 2, Node(Leaf, 3, Leaf))
   After editing right child to 30 (recursive): Node(Node(Leaf, 1, Leaf), 2, Node(Leaf, 30, Leaf))
*/
