// Example 208: Traversal — Focus on Zero or More Targets
//
// A Traversal generalises a Lens: instead of focusing on exactly one value
// inside `S`, it focuses on *zero or more* values of type `A`.
//
// Two primitive operations characterise a Traversal:
//   - `over`:    apply a function to every focused value, return updated `S`
//   - `collect`: gather every focused value into a `Vec<A>`
//
// Everything else — `length_of`, `all_of`, `any_of`, `sum_of` — is derived
// from those two primitives and works uniformly for *any* Traversal.

// ---------------------------------------------------------------------------
// Approach 1: Struct-based Traversal (mirrors the OCaml record directly)
// ---------------------------------------------------------------------------

type OverFn<S, A> = Box<dyn Fn(&dyn Fn(&A) -> A, &S) -> S>;
type ToListFn<S, A> = Box<dyn Fn(&S) -> Vec<A>>;

/// A Traversal that focuses on zero or more values of type `A` inside `S`.
pub struct Traversal<S, A> {
    over_fn: OverFn<S, A>,
    to_list_fn: ToListFn<S, A>,
}

impl<S: 'static, A: 'static> Traversal<S, A> {
    /// Build a Traversal from two functions.
    pub fn new(
        over: impl Fn(&dyn Fn(&A) -> A, &S) -> S + 'static,
        to_list: impl Fn(&S) -> Vec<A> + 'static,
    ) -> Self {
        Traversal {
            over_fn: Box::new(over),
            to_list_fn: Box::new(to_list),
        }
    }

    /// Apply `f` to every focused value, returning the updated structure.
    pub fn over(&self, f: impl Fn(&A) -> A, s: &S) -> S {
        (self.over_fn)(&f, s)
    }

    /// Collect all focused values into a `Vec`.
    pub fn collect_all(&self, s: &S) -> Vec<A> {
        (self.to_list_fn)(s)
    }

    /// Count how many values are focused.
    pub fn length_of(&self, s: &S) -> usize {
        self.collect_all(s).len()
    }

    /// `true` iff every focused value satisfies `pred`.
    pub fn all_of(&self, s: &S, pred: impl Fn(&A) -> bool) -> bool {
        self.collect_all(s).iter().all(pred)
    }

    /// `true` iff at least one focused value satisfies `pred`.
    pub fn any_of(&self, s: &S, pred: impl Fn(&A) -> bool) -> bool {
        self.collect_all(s).iter().any(pred)
    }
}

// ---------------------------------------------------------------------------
// Traversal over every element of a Vec
// ---------------------------------------------------------------------------

/// Traversal that focuses on every element of a `Vec<T>`.
///
/// OCaml equivalent:
/// ```ocaml
/// let each_traversal = { over = List.map; to_list = Fun.id }
/// ```
pub fn each_traversal<T: Clone + 'static>() -> Traversal<Vec<T>, T> {
    Traversal::new(
        |f, v: &Vec<T>| v.iter().map(f).collect(),
        |v: &Vec<T>| v.clone(),
    )
}

// ---------------------------------------------------------------------------
// Binary tree + leaf traversal
// ---------------------------------------------------------------------------

/// A simple binary tree whose leaves hold values of type `A`.
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<A> {
    Leaf(A),
    Branch(Box<Tree<A>>, Box<Tree<A>>),
}

fn tree_map<T>(f: &dyn Fn(&T) -> T, tree: &Tree<T>) -> Tree<T> {
    match tree {
        Tree::Leaf(x) => Tree::Leaf(f(x)),
        Tree::Branch(l, r) => Tree::Branch(Box::new(tree_map(f, l)), Box::new(tree_map(f, r))),
    }
}

fn tree_to_list<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Leaf(x) => vec![x.clone()],
        Tree::Branch(l, r) => tree_to_list(l).into_iter().chain(tree_to_list(r)).collect(),
    }
}

/// Traversal that focuses on every leaf of a `Tree<T>`.
///
/// OCaml equivalent:
/// ```ocaml
/// let each_leaf = { over = tree_over; to_list = tree_to_list }
/// ```
pub fn each_leaf_traversal<T: Clone + 'static>() -> Traversal<Tree<T>, T> {
    Traversal::new(tree_map, tree_to_list)
}

// ---------------------------------------------------------------------------
// Approach 2: Trait-based Traversal (compile-time dispatch, no allocations)
// ---------------------------------------------------------------------------

/// Implement this trait on any container to get traversal operations for free.
pub trait TraversableExt<A> {
    /// Apply `f` to every focused value, consuming and returning the structure.
    fn over_all(self, f: &impl Fn(A) -> A) -> Self;

    /// Collect all focused values by clone.
    fn collect_targets(&self) -> Vec<A>
    where
        A: Clone;
}

impl<A> TraversableExt<A> for Vec<A> {
    fn over_all(self, f: &impl Fn(A) -> A) -> Self {
        self.into_iter().map(f).collect()
    }

    fn collect_targets(&self) -> Vec<A>
    where
        A: Clone,
    {
        self.clone()
    }
}

fn tree_over_owned<A>(tree: Tree<A>, f: &impl Fn(A) -> A) -> Tree<A> {
    match tree {
        Tree::Leaf(x) => Tree::Leaf(f(x)),
        Tree::Branch(l, r) => Tree::Branch(
            Box::new(tree_over_owned(*l, f)),
            Box::new(tree_over_owned(*r, f)),
        ),
    }
}

impl<A: Clone> TraversableExt<A> for Tree<A> {
    fn over_all(self, f: &impl Fn(A) -> A) -> Self {
        tree_over_owned(self, f)
    }

    fn collect_targets(&self) -> Vec<A> {
        tree_to_list(self)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ── Vec traversal (struct-based) ──────────────────────────────────────

    #[test]
    fn test_each_over_doubles_all_elements() {
        let trav = each_traversal::<i32>();
        let result = trav.over(|x| x * 2, &vec![1, 2, 3, 4]);
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_each_over_empty_vec_is_empty() {
        let trav = each_traversal::<i32>();
        let result = trav.over(|x| x * 2, &vec![]);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_each_collect_all_returns_elements() {
        let trav = each_traversal::<i32>();
        assert_eq!(trav.collect_all(&vec![10, 20, 30]), vec![10, 20, 30]);
    }

    #[test]
    fn test_each_length_of_counts_elements() {
        let trav = each_traversal::<i32>();
        assert_eq!(trav.length_of(&vec![1, 2, 3]), 3);
        assert_eq!(trav.length_of(&vec![]), 0);
    }

    #[test]
    fn test_each_all_of_even_numbers() {
        let trav = each_traversal::<i32>();
        assert!(trav.all_of(&vec![2, 4, 6], |x| x % 2 == 0));
        assert!(!trav.all_of(&vec![2, 3, 6], |x| x % 2 == 0));
    }

    #[test]
    fn test_each_any_of_finds_even() {
        let trav = each_traversal::<i32>();
        assert!(trav.any_of(&vec![1, 3, 4], |x| x % 2 == 0));
        assert!(!trav.any_of(&vec![1, 3, 5], |x| x % 2 == 0));
    }

    // ── Tree leaf traversal (struct-based) ───────────────────────────────

    /// Builds: Branch(Leaf(1), Branch(Leaf(2), Leaf(3)))
    fn sample_tree() -> Tree<i32> {
        Tree::Branch(
            Box::new(Tree::Leaf(1)),
            Box::new(Tree::Branch(
                Box::new(Tree::Leaf(2)),
                Box::new(Tree::Leaf(3)),
            )),
        )
    }

    #[test]
    fn test_tree_over_doubles_all_leaves() {
        let trav = each_leaf_traversal::<i32>();
        let result = trav.over(|x| x * 2, &sample_tree());
        assert_eq!(
            result,
            Tree::Branch(
                Box::new(Tree::Leaf(2)),
                Box::new(Tree::Branch(
                    Box::new(Tree::Leaf(4)),
                    Box::new(Tree::Leaf(6)),
                )),
            )
        );
    }

    #[test]
    fn test_tree_collect_all_leaves_in_order() {
        let trav = each_leaf_traversal::<i32>();
        assert_eq!(trav.collect_all(&sample_tree()), vec![1, 2, 3]);
    }

    #[test]
    fn test_tree_length_of_counts_leaves() {
        let trav = each_leaf_traversal::<i32>();
        assert_eq!(trav.length_of(&sample_tree()), 3);
        assert_eq!(trav.length_of(&Tree::Leaf(42)), 1);
    }

    #[test]
    fn test_tree_single_leaf_over_and_collect() {
        let trav = each_leaf_traversal::<i32>();
        let t = Tree::Leaf(5);
        assert_eq!(trav.over(|x| x + 10, &t), Tree::Leaf(15));
        assert_eq!(trav.collect_all(&t), vec![5]);
    }

    // ── Trait-based traversal ─────────────────────────────────────────────

    #[test]
    fn test_trait_vec_over_all_triples() {
        let result = vec![1, 2, 3].over_all(&|x| x * 3);
        assert_eq!(result, vec![3, 6, 9]);
    }

    #[test]
    fn test_trait_vec_collect_targets_clones() {
        let v = vec![7, 8, 9];
        assert_eq!(v.collect_targets(), vec![7, 8, 9]);
    }

    #[test]
    fn test_trait_tree_over_all_adds_ten() {
        let t = Tree::Branch(Box::new(Tree::Leaf(1)), Box::new(Tree::Leaf(2)));
        let result = t.over_all(&|x| x + 10);
        assert_eq!(
            result,
            Tree::Branch(Box::new(Tree::Leaf(11)), Box::new(Tree::Leaf(12)))
        );
    }

    #[test]
    fn test_trait_tree_collect_targets_matches_leaves() {
        assert_eq!(sample_tree().collect_targets(), vec![1, 2, 3]);
    }
}
