use std::rc::Rc;

/// Persistent vector using a balanced binary tree.
///
/// `Rc` enables structural sharing: `set` creates only O(log n) new nodes,
/// sharing the unchanged subtrees with the original version. The original
/// is never mutated — both old and new versions coexist.
#[derive(Debug, Clone)]
pub enum PVec<T> {
    Nil,
    One(T),
    /// Internal node: left subtree and right subtree, shared via Rc.
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T> PVec<T> {
    pub fn size(&self) -> usize {
        match self {
            PVec::Nil => 0,
            PVec::One(_) => 1,
            PVec::Two(l, r) => l.size() + r.size(),
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            PVec::Nil => None,
            PVec::One(x) => (i == 0).then_some(x),
            PVec::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.get(i)
                } else {
                    r.get(i - ls)
                }
            }
        }
    }

    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVec::Nil => None,
            PVec::One(_) => (i == 0).then(|| PVec::One(v)),
            PVec::Two(l, r) => {
                let ls = l.size();
                if i < ls {
                    let new_l = l.set(i, v)?;
                    Some(PVec::Two(Rc::new(new_l), Rc::clone(r)))
                } else {
                    let new_r = r.set(i - ls, v)?;
                    Some(PVec::Two(Rc::clone(l), Rc::new(new_r)))
                }
            }
        }
    }
}

impl<T: Clone> PVec<T> {
    pub fn of_list(lst: &[T]) -> Self {
        match lst {
            [] => PVec::Nil,
            [x] => PVec::One(x.clone()),
            lst => {
                let mid = lst.len() / 2;
                PVec::Two(
                    Rc::new(Self::of_list(&lst[..mid])),
                    Rc::new(Self::of_list(&lst[mid..])),
                )
            }
        }
    }
}

// Functional style free functions — mirrors OCaml argument order

pub fn pvec_get<T>(i: usize, v: &PVec<T>) -> Option<&T> {
    match v {
        PVec::Nil => None,
        PVec::One(x) => (i == 0).then_some(x),
        PVec::Two(l, r) => {
            let ls = l.size();
            if i < ls {
                pvec_get(i, l)
            } else {
                pvec_get(i - ls, r)
            }
        }
    }
}

pub fn pvec_set<T>(i: usize, val: T, v: &PVec<T>) -> Option<PVec<T>> {
    match v {
        PVec::Nil => None,
        PVec::One(_) => (i == 0).then(|| PVec::One(val)),
        PVec::Two(l, r) => {
            let ls = l.size();
            if i < ls {
                let new_l = pvec_set(i, val, l)?;
                Some(PVec::Two(Rc::new(new_l), Rc::clone(r)))
            } else {
                let new_r = pvec_set(i - ls, val, r)?;
                Some(PVec::Two(Rc::clone(l), Rc::new(new_r)))
            }
        }
    }
}

fn main() {
    // Build a persistent vector from a list
    let v = PVec::of_list(&[10, 20, 30, 40, 50]);
    println!("v[2] = {:?}", v.get(2));

    // set returns a NEW vector; v is unchanged
    let v2 = v.set(2, 99).unwrap();
    println!("v[2]  after set = {:?}", v.get(2));  // still 30
    println!("v2[2] after set = {:?}", v2.get(2)); // 99

    // Demonstrate functional free functions (OCaml argument order)
    let v3 = pvec_set(0, 100, &v2).unwrap();
    println!("v3[0] = {:?}", pvec_get(0, &v3));
    println!("v3[2] = {:?}", pvec_get(2, &v3));

    // Chain updates — each version is independent
    let v4 = v.set(1, 20).and_then(|v1| v1.set(3, 40));
    println!("v4 = {:?} {:?} {:?} {:?} {:?}",
        v4.as_ref().and_then(|x| x.get(0)),
        v4.as_ref().and_then(|x| x.get(1)),
        v4.as_ref().and_then(|x| x.get(2)),
        v4.as_ref().and_then(|x| x.get(3)),
        v4.as_ref().and_then(|x| x.get(4)),
    );
}

/* Output:
   v[2] = Some(30)
   v[2]  after set = Some(30)
   v2[2] after set = Some(99)
   v3[0] = Some(100)
   v3[2] = Some(99)
   v4 = Some(10) Some(20) Some(30) Some(40) Some(50)
*/
