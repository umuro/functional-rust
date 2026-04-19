use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PVec<T> {
    #[default]
    Nil,
    One(T),
    Two(Rc<PVec<T>>, Rc<PVec<T>>),
}

impl<T: Clone> PVec<T> {
    pub fn new() -> Self {
        PVec::Nil
    }

    pub fn len(&self) -> usize {
        match self {
            PVec::Nil => 0,
            PVec::One(_) => 1,
            PVec::Two(l, r) => l.len() + r.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, PVec::Nil)
    }

    pub fn get(&self, i: usize) -> Option<&T> {
        match self {
            PVec::Nil => None,
            PVec::One(v) => (i == 0).then_some(v),
            PVec::Two(l, r) => {
                let left_len = l.len();
                if i < left_len {
                    l.get(i)
                } else {
                    r.get(i - left_len)
                }
            }
        }
    }

    pub fn push(&self, x: T) -> Self {
        match self {
            PVec::Nil => PVec::One(x),
            _ => PVec::Two(Rc::new(self.clone()), Rc::new(PVec::One(x))),
        }
    }

    pub fn pop(&self) -> Option<(T, Self)> {
        match self {
            PVec::Nil => None,
            PVec::One(v) => Some((v.clone(), PVec::Nil)),
            PVec::Two(l, r) => {
                let (v, new_r) = r.pop()?;
                let rest = match new_r {
                    PVec::Nil => (**l).clone(),
                    other => PVec::Two(l.clone(), Rc::new(other)),
                };
                Some((v, rest))
            }
        }
    }

    pub fn from_slice(items: &[T]) -> Self {
        items.iter().fold(PVec::Nil, |acc, x| acc.push(x.clone()))
    }

    pub fn to_vec(&self) -> Vec<T> {
        let mut out = Vec::with_capacity(self.len());
        self.extend_into(&mut out);
        out
    }

    fn extend_into(&self, out: &mut Vec<T>) {
        match self {
            PVec::Nil => {}
            PVec::One(v) => out.push(v.clone()),
            PVec::Two(l, r) => {
                l.extend_into(out);
                r.extend_into(out);
            }
        }
    }
}

fn main() {
    // Mirror the OCaml driver: push 1..=5, then pop everything in LIFO order.
    let mut v = PVec::from_slice(&[1, 2, 3, 4, 5]);
    print!("drain: ");
    while let Some((x, rest)) = v.pop() {
        print!("{x} ");
        v = rest;
    }
    println!();

    // Persistence: v1 is unchanged after v2 is derived from it.
    let v1 = PVec::from_slice(&[10, 20, 30]);
    let v2 = v1.push(40);
    println!("v1 = {:?}", v1.to_vec());
    println!("v2 = {:?}", v2.to_vec());
}

/* Output:
   drain: 5 4 3 2 1
   v1 = [10, 20, 30]
   v2 = [10, 20, 30, 40]
*/
