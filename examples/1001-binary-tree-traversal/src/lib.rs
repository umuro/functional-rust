#![allow(clippy::all)]
//! Binary Tree — Size, Membership, Traversal
//! See example.ml for OCaml reference

pub enum Tree<T> {
    Leaf,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

// Placeholder - conversions will be completed via Claude Code agents
pub fn example() {
    unimplemented!("TODO: Implement binary tree operations")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "not implemented")]
    fn placeholder() {
        example();
    }
}
