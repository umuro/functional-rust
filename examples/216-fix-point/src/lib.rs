#![allow(clippy::all)]
//! # Example 216: Fix Point — How Recursive Types Work Under the Hood
//!
//! Separates the *shape* of a recursive type from the *mechanism* of recursion.
//! A "base functor" `F<A>` describes one node with children of type `A`.
//! The fix point wraps `F` around itself so that `A = Fix<F>`, yielding
//! full, unbounded recursion from a non-recursive building block.
//!
//! Because shape and recursion are separate, a single `cata` (catamorphism)
//! captures all traversal logic — algebras only describe one local step.

// ============================================================
// Approach 1: Fix point for lists
// ============================================================
//
// `ListF<A>` is the shape of ONE list layer.
// Replace A with `FixList` and you get an ordinary recursive list.

/// One layer of a list — either empty, or an element plus a child slot `A`.
#[derive(Debug)]
pub enum ListF<A> {
    NilF,
    ConsF(i64, A),
}

impl<A> ListF<A> {
    /// Functorial map — apply `f` to the single child position.
    pub fn map<B>(self, f: impl FnOnce(A) -> B) -> ListF<B> {
        match self {
            ListF::NilF => ListF::NilF,
            ListF::ConsF(x, rest) => ListF::ConsF(x, f(rest)),
        }
    }
}

/// `Fix<ListF>`: one layer of `ListF` whose child slot holds another `FixList`.
///
/// `FixList ≅ ListF<FixList> ≅ NilF | ConsF(i64, FixList)`
#[derive(Debug)]
pub struct FixList(Box<ListF<FixList>>);

impl FixList {
    /// Peel off the outermost `Fix` wrapper, exposing `ListF<FixList>`.
    pub fn unfix(self) -> ListF<FixList> {
        *self.0
    }

    pub fn nil() -> Self {
        FixList(Box::new(ListF::NilF))
    }

    pub fn cons(x: i64, xs: FixList) -> Self {
        FixList(Box::new(ListF::ConsF(x, xs)))
    }
}

/// Catamorphism for `FixList`: fold bottom-up using a local algebra `alg`.
///
/// `alg` only handles *one layer*; all recursion lives here.
pub fn cata_list<A>(list: FixList, alg: &impl Fn(ListF<A>) -> A) -> A {
    alg(list.unfix().map(|child| cata_list(child, alg)))
}

// ============================================================
// Approach 2: Fix point for binary trees
// ============================================================

/// One layer of a binary tree — a leaf value or a branch with two child slots.
#[derive(Debug)]
pub enum TreeF<A> {
    LeafF(i64),
    BranchF(A, A),
}

impl<A> TreeF<A> {
    /// Functorial map — apply `f` to all child positions (two for BranchF).
    pub fn map<B>(self, mut f: impl FnMut(A) -> B) -> TreeF<B> {
        match self {
            TreeF::LeafF(n) => TreeF::LeafF(n),
            TreeF::BranchF(l, r) => TreeF::BranchF(f(l), f(r)),
        }
    }
}

/// `Fix<TreeF>`: recursive binary tree built from a non-recursive shape.
#[derive(Debug)]
pub struct FixTree(Box<TreeF<FixTree>>);

impl FixTree {
    pub fn unfix(self) -> TreeF<FixTree> {
        *self.0
    }

    pub fn leaf(n: i64) -> Self {
        FixTree(Box::new(TreeF::LeafF(n)))
    }

    pub fn branch(l: FixTree, r: FixTree) -> Self {
        FixTree(Box::new(TreeF::BranchF(l, r)))
    }
}

/// Catamorphism for `FixTree`: fold bottom-up using a local algebra `alg`.
pub fn cata_tree<A>(tree: FixTree, alg: &impl Fn(TreeF<A>) -> A) -> A {
    alg(tree.unfix().map(|child| cata_tree(child, alg)))
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- FixList tests ----------

    #[test]
    fn test_list_sum_empty() {
        let sum = cata_list(FixList::nil(), &|node| match node {
            ListF::NilF => 0i64,
            ListF::ConsF(x, acc) => x + acc,
        });
        assert_eq!(sum, 0);
    }

    #[test]
    fn test_list_sum() {
        // [1, 2, 3] → 6
        let list = FixList::cons(1, FixList::cons(2, FixList::cons(3, FixList::nil())));
        let sum = cata_list(list, &|node| match node {
            ListF::NilF => 0i64,
            ListF::ConsF(x, acc) => x + acc,
        });
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_list_length() {
        let list = FixList::cons(10, FixList::cons(20, FixList::nil()));
        let len: usize = cata_list(list, &|node| match node {
            ListF::NilF => 0,
            ListF::ConsF(_, rest) => 1 + rest,
        });
        assert_eq!(len, 2);
    }

    #[test]
    fn test_list_to_vec() {
        // fold into a Vec, preserving original order
        let list = FixList::cons(1, FixList::cons(2, FixList::cons(3, FixList::nil())));
        let v: Vec<i64> = cata_list(list, &|node: ListF<Vec<i64>>| match node {
            ListF::NilF => vec![],
            ListF::ConsF(x, mut tail) => {
                tail.insert(0, x);
                tail
            }
        });
        assert_eq!(v, [1, 2, 3]);
    }

    // ---------- FixTree tests ----------

    /// Helper:  branch(branch(leaf 1, leaf 2), leaf 3)
    fn sample_tree() -> FixTree {
        FixTree::branch(
            FixTree::branch(FixTree::leaf(1), FixTree::leaf(2)),
            FixTree::leaf(3),
        )
    }

    #[test]
    fn test_tree_single_leaf() {
        let sum = cata_tree(FixTree::leaf(42), &|node| match node {
            TreeF::LeafF(n) => n,
            TreeF::BranchF(l, r) => l + r,
        });
        assert_eq!(sum, 42);
    }

    #[test]
    fn test_tree_sum() {
        // leaf(1) + leaf(2) + leaf(3) = 6
        let sum = cata_tree(sample_tree(), &|node| match node {
            TreeF::LeafF(n) => n,
            TreeF::BranchF(l, r) => l + r,
        });
        assert_eq!(sum, 6);
    }

    #[test]
    fn test_tree_depth() {
        // leaf → 0
        let d0: usize = cata_tree(FixTree::leaf(0), &|node: TreeF<usize>| match node {
            TreeF::LeafF(_) => 0,
            TreeF::BranchF(l, r) => 1 + l.max(r),
        });
        assert_eq!(d0, 0);

        // branch(branch(_, _), _) → depth 2
        let d2: usize = cata_tree(sample_tree(), &|node: TreeF<usize>| match node {
            TreeF::LeafF(_) => 0,
            TreeF::BranchF(l, r) => 1 + l.max(r),
        });
        assert_eq!(d2, 2);
    }

    #[test]
    fn test_tree_count_leaves() {
        let count: usize = cata_tree(sample_tree(), &|node| match node {
            TreeF::LeafF(_) => 1,
            TreeF::BranchF(l, r) => l + r,
        });
        assert_eq!(count, 3);
    }

    #[test]
    fn test_cata_two_algebras_same_tree() {
        // sum and count on the same structure shape
        let tree = FixTree::branch(FixTree::leaf(10), FixTree::leaf(20));
        let sum = cata_tree(tree, &|node| match node {
            TreeF::LeafF(n) => n,
            TreeF::BranchF(l, r) => l + r,
        });
        assert_eq!(sum, 30);
    }
}
