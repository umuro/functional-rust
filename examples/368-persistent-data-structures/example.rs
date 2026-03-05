use std::rc::Rc;

// Persistent linked list: structural sharing via Rc
#[derive(Clone)]
enum PList<T> {
    Nil,
    Cons(T, Rc<PList<T>>),
}

impl<T: Clone + std::fmt::Debug> PList<T> {
    fn nil() -> Rc<Self> { Rc::new(Self::Nil) }
    fn cons(head: T, tail: Rc<Self>) -> Rc<Self> { Rc::new(Self::Cons(head, tail)) }

    fn to_vec(list: &Rc<Self>) -> Vec<T> {
        let mut v = Vec::new();
        let mut cur = Rc::clone(list);
        loop {
            match cur.as_ref() {
                Self::Nil => break,
                Self::Cons(x, next) => { v.push(x.clone()); cur = Rc::clone(next); }
            }
        }
        v
    }
    fn len(list: &Rc<Self>) -> usize { Self::to_vec(list).len() }
}

// Persistent vector using path copying
#[derive(Clone, Debug)]
struct PVec<T: Clone> {
    data: Rc<Vec<T>>,
}

impl<T: Clone + std::fmt::Debug> PVec<T> {
    fn new() -> Self { Self { data: Rc::new(Vec::new()) } }
    fn push(&self, val: T) -> Self {
        let mut new_data = (*self.data).clone(); // copy-on-write
        new_data.push(val);
        Self { data: Rc::new(new_data) }
    }
    fn set(&self, i: usize, val: T) -> Self {
        let mut new_data = (*self.data).clone();
        new_data[i] = val;
        Self { data: Rc::new(new_data) }
    }
    fn get(&self, i: usize) -> Option<&T> { self.data.get(i) }
    fn len(&self) -> usize { self.data.len() }
}

fn main() {
    // Persistent list
    let list1 = PList::cons(3, PList::cons(2, PList::cons(1, PList::nil())));
    let list2 = PList::cons(4, Rc::clone(&list1)); // shares tail
    let list3 = if let PList::Cons(_, tail) = list1.as_ref() { Rc::clone(tail) } else { PList::nil() };

    println!("list1: {:?}", PList::to_vec(&list1));
    println!("list2: {:?}", PList::to_vec(&list2));
    println!("list3: {:?}", PList::to_vec(&list3));

    // Persistent vector
    let v0: PVec<i32> = PVec::new();
    let v1 = v0.push(1);
    let v2 = v1.push(2);
    let v3 = v2.set(0, 99); // v1 is unchanged
    println!("v1: {:?}", v1.data);
    println!("v2: {:?}", v2.data);
    println!("v3: {:?}", v3.data); // [99, 2]
    println!("v1 unchanged: {:?}", v1.data); // [1]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn persistent_list() {
        let l1 = PList::cons(2, PList::cons(1, PList::nil()));
        let l2 = PList::cons(3, Rc::clone(&l1));
        assert_eq!(PList::to_vec(&l1), vec![2,1]);
        assert_eq!(PList::to_vec(&l2), vec![3,2,1]);
        assert_eq!(PList::to_vec(&l1), vec![2,1]); // l1 unchanged
    }
    #[test] fn persistent_vec() {
        let v0: PVec<i32> = PVec::new();
        let v1 = v0.push(1); let v2 = v1.push(2);
        let v3 = v2.set(0, 99);
        assert_eq!(*v1.data, vec![1]);
        assert_eq!(*v3.data, vec![99,2]);
    }
}
