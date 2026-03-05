#[derive(Debug)]
enum Tree { Leaf, Node { val: i32, left: Box<Tree>, right: Box<Tree> } }

impl Tree {
    fn leaf() -> Box<Self> { Box::new(Tree::Leaf) }
    fn node(val: i32, left: Box<Tree>, right: Box<Tree>) -> Box<Self> {
        Box::new(Tree::Node { val, left, right })
    }
}

// Rust auto-derefs through Box in patterns
fn depth(t: &Tree) -> usize {
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, right, .. } => 1 + depth(left).max(depth(right)),
    }
}

fn contains(t: &Tree, v: i32) -> bool {
    match t {
        Tree::Leaf => false,
        Tree::Node { val, left, right } => match v.cmp(val) {
            std::cmp::Ordering::Equal   => true,
            std::cmp::Ordering::Less    => contains(left, v),
            std::cmp::Ordering::Greater => contains(right, v),
        },
    }
}

fn insert(t: Box<Tree>, v: i32) -> Box<Tree> {
    match *t {
        Tree::Leaf => Tree::node(v, Tree::leaf(), Tree::leaf()),
        Tree::Node { val, left, right } => {
            if      v < val { Tree::node(val, insert(left,v), right) }
            else if v > val { Tree::node(val, left, insert(right,v)) }
            else            { Tree::node(val, left, right) }
        }
    }
}

fn main() {
    let mut t = Tree::leaf();
    for x in [5,3,7,1,4] { t = insert(t, x); }
    println!("depth={}", depth(&t));
    println!("has 3={} has 6={}", contains(&t,3), contains(&t,6));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_contains() {
        let mut t = Tree::leaf();
        for x in [5,3,7] { t = insert(t, x); }
        assert!(contains(&t,3)); assert!(!contains(&t,6));
    }
}
