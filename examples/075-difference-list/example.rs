/// # Difference List — O(1) Append
///
/// A difference list represents a list as a function from "rest of list" to "full list".
/// This makes append O(1) — just function composition — instead of O(n).
///
/// In Rust, this pattern is less common because Vec already has O(1) amortized push_back.
/// But it's a great exercise in higher-order functions and closures.

/// A difference list is a function: given a tail, prepend our elements.
/// We use Box<dyn Fn> since closures have unique types.
pub struct DList<T> {
    f: Box<dyn Fn(Vec<T>) -> Vec<T>>,
}

impl<T: 'static + Clone> DList<T> {
    /// Empty difference list — identity function.
    pub fn empty() -> Self {
        DList {
            f: Box::new(|rest| rest),
        }
    }

    /// Singleton — prepends one element.
    pub fn singleton(x: T) -> Self {
        DList {
            f: Box::new(move |mut rest| {
                rest.insert(0, x.clone());
                rest
            }),
        }
    }

    /// From a Vec.
    pub fn from_vec(v: Vec<T>) -> Self {
        DList {
            f: Box::new(move |rest| {
                let mut result = v.clone();
                result.extend(rest);
                result
            }),
        }
    }

    /// O(1) append — just compose the two functions.
    pub fn append(self, other: DList<T>) -> DList<T> {
        DList {
            f: Box::new(move |rest| (self.f)((other.f)(rest))),
        }
    }

    /// Convert to Vec — apply the function to an empty list.
    pub fn to_vec(&self) -> Vec<T> {
        (self.f)(vec![])
    }
}

/// Alternative: just use Vec! Rust's Vec already has O(1) amortized push.
/// This shows that the difference list pattern, while elegant in OCaml/Haskell,
/// is often unnecessary in Rust.
pub fn concat_with_vec(lists: &[Vec<i32>]) -> Vec<i32> {
    let total_len: usize = lists.iter().map(|l| l.len()).sum();
    let mut result = Vec::with_capacity(total_len);
    for list in lists {
        result.extend(list);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let dl: DList<i32> = DList::empty();
        assert_eq!(dl.to_vec(), vec![]);
    }

    #[test]
    fn test_singleton() {
        let dl = DList::singleton(42);
        assert_eq!(dl.to_vec(), vec![42]);
    }

    #[test]
    fn test_from_vec() {
        let dl = DList::from_vec(vec![1, 2, 3]);
        assert_eq!(dl.to_vec(), vec![1, 2, 3]);
    }

    #[test]
    fn test_append() {
        let a = DList::from_vec(vec![1, 2, 3]);
        let b = DList::from_vec(vec![4, 5, 6]);
        let c = DList::singleton(7);
        let result = a.append(b).append(c);
        assert_eq!(result.to_vec(), vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_multiple_appends() {
        let dl = DList::from_vec(vec![1])
            .append(DList::from_vec(vec![2]))
            .append(DList::from_vec(vec![3]))
            .append(DList::from_vec(vec![4]));
        assert_eq!(dl.to_vec(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_concat_with_vec() {
        let lists = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(concat_with_vec(&lists), vec![1, 2, 3, 4, 5, 6]);
    }
}

fn main() {
    println!("{:?}", dl.to_vec(), vec![]);
    println!("{:?}", dl.to_vec(), vec![42]);
    println!("{:?}", dl.to_vec(), vec![1, 2, 3]);
}
