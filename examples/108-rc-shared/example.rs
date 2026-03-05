// Example 108: Rc<T> — Shared Ownership
// Rc<T> = Reference Counted pointer. Multiple owners, single-threaded.

use std::rc::Rc;

#[derive(Debug)]
enum Tree {
    Leaf,
    Node(Rc<Tree>, i32, Rc<Tree>),
}

fn tree_sum(t: &Tree) -> i32 {
    match t {
        Tree::Leaf => 0,
        Tree::Node(l, v, r) => tree_sum(l) + v + tree_sum(r),
    }
}

#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T: Copy> List<T> {
    fn nil() -> Rc<Self> {
        Rc::new(List::Nil)
    }
    fn cons(head: T, tail: Rc<Self>) -> Rc<Self> {
        Rc::new(List::Cons(head, tail))
    }
    fn to_vec(list: &Rc<Self>) -> Vec<T> {
        let mut acc = Vec::new();
        let mut cur = Rc::clone(list);
        loop {
            match cur.as_ref() {
                List::Nil => break,
                List::Cons(h, t) => {
                    acc.push(*h);
                    cur = Rc::clone(t);
                }
            }
        }
        acc
    }
}

fn main() {
    // --- Shared tree nodes ---
    let shared = Rc::new(Tree::Node(
        Rc::new(Tree::Leaf),
        42,
        Rc::new(Tree::Leaf),
    ));
    let tree1 = Tree::Node(Rc::clone(&shared), 1, Rc::new(Tree::Leaf));
    let tree2 = Tree::Node(Rc::new(Tree::Leaf), 2, Rc::clone(&shared));
    println!("tree1 sum = {}", tree_sum(&tree1)); // 43
    println!("tree2 sum = {}", tree_sum(&tree2)); // 44
    println!("shared strong_count = {}", Rc::strong_count(&shared)); // 3

    // --- Shared-tail cons list ---
    let tail = {
        let nil = List::nil();
        let t1 = List::cons(1, nil);
        let t2 = List::cons(2, t1);
        List::cons(3, t2)
    };
    let list_a = List::cons(10, Rc::clone(&tail));
    let list_b = List::cons(20, Rc::clone(&tail));
    println!("list_a = {:?}", List::to_vec(&list_a)); // [10, 3, 2, 1]
    println!("list_b = {:?}", List::to_vec(&list_b)); // [20, 3, 2, 1]
    println!("tail strong_count = {}", Rc::strong_count(&tail)); // 3

    // --- Drop semantics ---
    let a = Rc::new("hello");
    let b = Rc::clone(&a);
    println!("count with b alive = {}", Rc::strong_count(&a)); // 2
    drop(b);
    println!("count after drop(b) = {}", Rc::strong_count(&a)); // 1
}

/* Output:
   tree1 sum = 43
   tree2 sum = 44
   shared strong_count = 3
   list_a = [10, 3, 2, 1]
   list_b = [20, 3, 2, 1]
   tail strong_count = 3
   count with b alive = 2
   count after drop(b) = 1
*/
