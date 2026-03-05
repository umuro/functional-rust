// Example 067: Foldable Trait
// Custom fold over tree/list structures

// Foldable trait
trait Foldable {
    type Item;
    fn fold_left<B, F: FnMut(B, &Self::Item) -> B>(&self, init: B, f: F) -> B;
    fn fold_right<B, F: FnMut(&Self::Item, B) -> B>(&self, init: B, f: F) -> B;

    // Derived operations
    fn to_vec(&self) -> Vec<Self::Item> where Self::Item: Clone {
        self.fold_right(Vec::new(), |x, mut acc| { acc.insert(0, x.clone()); acc })
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
        match self {
            MyList::Nil => init,
            MyList::Cons(x, xs) => {
                let acc = f(init, x);
                xs.fold_left(acc, f)
            }
        }
    }
    fn fold_right<B, F: FnMut(&T, B) -> B>(&self, init: B, mut f: F) -> B {
        match self {
            MyList::Nil => init,
            MyList::Cons(x, xs) => f(x, xs.fold_right(init, f)),
        }
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
        match self {
            Tree::Leaf => init,
            Tree::Node(l, v, r) => {
                let acc = l.fold_left(init, &mut f);
                let acc = f(acc, v);
                r.fold_left(acc, f)
            }
        }
    }
    fn fold_right<B, F: FnMut(&T, B) -> B>(&self, init: B, mut f: F) -> B {
        match self {
            Tree::Leaf => init,
            Tree::Node(l, v, r) => {
                let acc = r.fold_right(init, &mut f);
                let acc = f(v, acc);
                l.fold_right(acc, f)
            }
        }
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
        Tree::node(Tree::node(Tree::Leaf, 1, Tree::Leaf), 2, Tree::node(Tree::Leaf, 3, Tree::Leaf))
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
