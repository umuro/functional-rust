// Example 067: Foldable Trait
// Custom fold over tree/list structures

// Foldable trait
trait Foldable {
    type Item;
    fn fold_left<B, F: FnMut(B, &Self::Item) -> B>(&self, init: B, f: F) -> B;
    fn fold_right<B, F: FnMut(&Self::Item, B) -> B>(&self, init: B, f: F) -> B;

    // Derived operations
    fn to_vec(&self) -> Vec<Self::Item>
    where
        Self::Item: Clone,
    {
        self.fold_right(Vec::new(), |x, mut acc| {
            acc.insert(0, x.clone());
            acc
        })
    }

    fn length(&self) -> usize {
        self.fold_left(0, |acc, _| acc + 1)
    }
}

// Approach 1: Foldable for custom list
#[derive(Debug)]
enum MyList<T> {
    Nil,
    Cons(T, Box<MyList<T>>),
}

impl<T> MyList<T> {
    fn cons(x: T, xs: MyList<T>) -> Self {
        MyList::Cons(x, Box::new(xs))
    }
}

impl<T> Foldable for MyList<T> {
    type Item = T;
    fn fold_left<B, F: FnMut(B, &T) -> B>(&self, init: B, mut f: F) -> B {
        let mut acc = init;
        let mut current = self;
        loop {
            match current {
                MyList::Nil => return acc,
                MyList::Cons(x, xs) => {
                    acc = f(acc, x);
                    current = xs;
                }
            }
        }
    }
    fn fold_right<B, F: FnMut(&T, B) -> B>(&self, init: B, mut f: F) -> B {
        // Collect references, then fold from right
        let mut items = Vec::new();
        let mut current = self;
        loop {
            match current {
                MyList::Nil => break,
                MyList::Cons(x, xs) => {
                    items.push(x);
                    current = xs;
                }
            }
        }
        let mut acc = init;
        for x in items.into_iter().rev() {
            acc = f(x, acc);
        }
        acc
    }
}

// Approach 2: Foldable for binary tree
#[derive(Debug)]
enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T> Tree<T> {
    fn node(l: Tree<T>, v: T, r: Tree<T>) -> Self {
        Tree::Node(Box::new(l), v, Box::new(r))
    }
}

impl<T> Foldable for Tree<T> {
    type Item = T;
    fn fold_left<B, F: FnMut(B, &T) -> B>(&self, init: B, mut f: F) -> B {
        // In-order traversal using explicit stack
        let mut stack = Vec::new();
        let mut current = self;
        let mut acc = init;

        enum Action<'a, T> {
            Visit(&'a Tree<T>),
            Process(&'a T),
        }

        stack.push(Action::Visit(current));
        while let Some(action) = stack.pop() {
            match action {
                Action::Visit(Tree::Leaf) => {}
                Action::Visit(Tree::Node(l, v, r)) => {
                    stack.push(Action::Visit(r));
                    stack.push(Action::Process(v));
                    stack.push(Action::Visit(l));
                }
                Action::Process(v) => {
                    acc = f(acc, v);
                }
            }
        }
        acc
    }
    fn fold_right<B, F: FnMut(&T, B) -> B>(&self, init: B, mut f: F) -> B {
        // Collect in-order, then fold from right
        let mut items = Vec::new();
        fn collect<'a, T>(tree: &'a Tree<T>, items: &mut Vec<&'a T>) {
            match tree {
                Tree::Leaf => {}
                Tree::Node(l, v, r) => {
                    collect(l, items);
                    items.push(v);
                    collect(r, items);
                }
            }
        }
        collect(self, &mut items);
        let mut acc = init;
        for x in items.into_iter().rev() {
            acc = f(x, acc);
        }
        acc
    }
}

// Approach 3: Generic functions over any Foldable
fn sum<F: Foldable<Item = i32>>(foldable: &F) -> i32 {
    foldable.fold_left(0, |acc, x| acc + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_list() -> MyList<i32> {
        MyList::cons(1, MyList::cons(2, MyList::cons(3, MyList::Nil)))
    }

    fn sample_tree() -> Tree<i32> {
        Tree::node(
            Tree::node(Tree::Leaf, 1, Tree::Leaf),
            2,
            Tree::node(Tree::Leaf, 3, Tree::Leaf),
        )
    }

    #[test]
    fn test_list_fold_left_sum() {
        assert_eq!(sum(&sample_list()), 6);
    }

    #[test]
    fn test_list_length() {
        assert_eq!(sample_list().length(), 3);
    }

    #[test]
    fn test_list_to_vec() {
        assert_eq!(sample_list().to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_tree_fold_left_sum() {
        assert_eq!(sum(&sample_tree()), 6);
    }

    #[test]
    fn test_tree_length() {
        assert_eq!(sample_tree().length(), 3);
    }

    #[test]
    fn test_tree_to_vec() {
        assert_eq!(sample_tree().to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(sum(&MyList::<i32>::Nil), 0);
        assert_eq!(sum(&Tree::<i32>::Leaf), 0);
    }
}
