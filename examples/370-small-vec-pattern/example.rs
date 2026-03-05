// SmallVec: inline for ≤N items, heap for more
#[derive(Debug)]
enum SmallVec<T, const N: usize> {
    Inline { data: [Option<T>; N], len: usize },
    Heap(Vec<T>),
}

impl<T: Clone + Default + std::fmt::Debug, const N: usize> SmallVec<T, N> {
    fn new() -> Self {
        Self::Inline { data: std::array::from_fn(|_| None), len: 0 }
    }

    fn push(&mut self, val: T) {
        match self {
            Self::Inline { data, len } if *len < N => {
                data[*len] = Some(val);
                *len += 1;
            }
            Self::Inline { data, len } => {
                // Overflow: move to heap
                let mut v: Vec<T> = data[..*len].iter_mut().filter_map(|x| x.take()).collect();
                v.push(val);
                *self = Self::Heap(v);
            }
            Self::Heap(v) => v.push(val),
        }
    }

    fn get(&self, i: usize) -> Option<&T> {
        match self {
            Self::Inline { data, len } => if i < *len { data[i].as_ref() } else { None },
            Self::Heap(v) => v.get(i),
        }
    }

    fn len(&self) -> usize {
        match self { Self::Inline{len,..} => *len, Self::Heap(v) => v.len() }
    }

    fn is_inline(&self) -> bool { matches!(self, Self::Inline{..}) }

    fn as_slice(&self) -> Vec<&T> {
        match self {
            Self::Inline{data,len} => data[..*len].iter().filter_map(|x|x.as_ref()).collect(),
            Self::Heap(v) => v.iter().collect(),
        }
    }
}

fn main() {
    let mut sv: SmallVec<i32, 4> = SmallVec::new();
    println!("Empty, inline: {}", sv.is_inline());

    for i in 0..4 { sv.push(i); }
    println!("4 items, inline: {}", sv.is_inline());
    println!("Items: {:?}", sv.as_slice());

    sv.push(4); // overflows to heap
    println!("5 items, inline: {}", sv.is_inline());
    println!("Items: {:?}", sv.as_slice());
    println!("len: {}", sv.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn inline_small() {
        let mut sv: SmallVec<i32,4> = SmallVec::new();
        for i in 0..4 { sv.push(i); }
        assert!(sv.is_inline()); assert_eq!(sv.len(), 4);
    }
    #[test] fn heap_on_overflow() {
        let mut sv: SmallVec<i32,2> = SmallVec::new();
        sv.push(1); sv.push(2); sv.push(3);
        assert!(!sv.is_inline()); assert_eq!(sv.len(), 3);
    }
    #[test] fn get_items() {
        let mut sv: SmallVec<i32,4> = SmallVec::new();
        for i in 0..6 { sv.push(i); }
        assert_eq!(sv.get(0), Some(&0)); assert_eq!(sv.get(5), Some(&5));
    }
}
