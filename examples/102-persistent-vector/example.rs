#![allow(dead_code)]

use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum PVec<T> {
    Nil,
    Leaf(T),
    Branch(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T> PVec<T> {
    pub fn empty() -> Self {
        PVec::Nil
    }

    pub fn size(&self) -> usize {
        match self {
            PVec::Nil => 0,
            PVec::Leaf(_) => 1,
            PVec::Branch(l, r) => l.size() + r.size(),
        }
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            PVec::Nil => None,
            PVec::Leaf(x) => (i == 0).then_some(x),
            PVec::Branch(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.get(i)
                } else {
                    r.get(i - ls)
                }
            }
        }
    }
}

impl<T: Clone> PVec<T> {
    pub fn set(&self, i: usize, v: T) -> Option<Self> {
        match self {
            PVec::Nil => None,
            PVec::Leaf(_) => (i == 0).then(|| PVec::Leaf(v)),
            PVec::Branch(l, r) => {
                let ls = l.size();
                if i < ls {
                    l.set(i, v)
                        .map(|new_l| PVec::Branch(Rc::new(new_l), Rc::clone(r)))
                } else {
                    r.set(i - ls, v)
                        .map(|new_r| PVec::Branch(Rc::clone(l), Rc::new(new_r)))
                }
            }
        }
    }

    pub fn from_slice(items: &[T]) -> Self {
        match items {
            [] => PVec::Nil,
            [x] => PVec::Leaf(x.clone()),
            _ => {
                let mid = items.len() / 2;
                PVec::Branch(
                    Rc::new(Self::from_slice(&items[..mid])),
                    Rc::new(Self::from_slice(&items[mid..])),
                )
            }
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self {
            PVec::Nil => vec![],
            PVec::Leaf(x) => vec![x.clone()],
            PVec::Branch(l, r) => l.to_vec().into_iter().chain(r.to_vec()).collect(),
        }
    }
}

fn main() {
    let v1 = PVec::from_slice(&[10, 20, 30, 40, 50]);
    println!("v1[2] = {:?}", v1.get(2));

    let v2 = v1.set(2, 99).unwrap();
    println!("v2[2] = {:?}", v2.get(2));
    println!("v1[2] = {:?}", v1.get(2)); // original unchanged

    println!("v1 = {:?}", v1.to_vec());
    println!("v2 = {:?}", v2.to_vec());

    // Build a history of versions
    let v3 = v2.set(0, 0).unwrap();
    let v4 = v3.set(4, 0).unwrap();
    println!("v3 = {:?}", v3.to_vec());
    println!("v4 = {:?}", v4.to_vec());
}

/* Output:
   v1[2] = Some(30)
   v2[2] = Some(99)
   v1[2] = Some(30)
   v1 = [10, 20, 30, 40, 50]
   v2 = [10, 20, 99, 40, 50]
   v3 = [0, 20, 99, 40, 50]
   v4 = [0, 20, 99, 40, 0]
*/
