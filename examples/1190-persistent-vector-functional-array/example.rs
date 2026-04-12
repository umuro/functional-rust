#![allow(dead_code)]

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum PVec<T> {
    Nil,
    One(T),
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T: Clone> PVec<T> {
    pub fn empty() -> Self {
        PVec::Nil
    }

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
                    l.set(i, v)
                        .map(|new_l| PVec::Two(Rc::new(new_l), Rc::clone(r)))
                } else {
                    r.set(i - ls, v)
                        .map(|new_r| PVec::Two(Rc::clone(l), Rc::new(new_r)))
                }
            }
        }
    }

    pub fn from_slice(items: &[T]) -> Self {
        match items {
            [] => PVec::Nil,
            [x] => PVec::One(x.clone()),
            _ => {
                let mid = items.len() / 2;
                PVec::Two(
                    Rc::new(Self::from_slice(&items[..mid])),
                    Rc::new(Self::from_slice(&items[mid..])),
                )
            }
        }
    }
}

fn main() {
    let v = PVec::from_slice(&[10, 20, 30, 40, 50]);
    println!("v[2] = {:?}", v.get(2));

    let v2 = v.set(2, 99).unwrap();
    println!("v2[2] = {:?}", v2.get(2));
    println!("v[2]  = {:?} (original unchanged)", v.get(2));

    // Demonstrate multiple independent versions from the same root
    let base = PVec::from_slice(&[1, 2, 3, 4, 5]);
    let va = base.set(0, 100).unwrap();
    let vb = base.set(4, 500).unwrap();
    println!("base[0]={:?}, va[0]={:?}, vb[0]={:?}", base.get(0), va.get(0), vb.get(0));
    println!("base[4]={:?}, va[4]={:?}, vb[4]={:?}", base.get(4), va.get(4), vb.get(4));
}

/* Output:
   v[2] = Some(30)
   v2[2] = Some(99)
   v[2]  = Some(30) (original unchanged)
   base[0]=Some(1), va[0]=Some(100), vb[0]=Some(1)
   base[4]=Some(5), va[4]=Some(5), vb[4]=Some(500)
*/
