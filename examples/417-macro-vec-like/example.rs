// Implementing vec!-like macros in Rust
use std::collections::{HashSet, HashMap, VecDeque};

// Clone of vec! to understand it
macro_rules! my_vec {
    // Empty
    () => { Vec::new() };
    // With elements — trailing comma optional
    ($($x:expr),+ $(,)?) => {
        {
            let mut v = Vec::with_capacity(count_args!($($x),+));
            $(v.push($x);)+
            v
        }
    };
}

// Helper for capacity
macro_rules! count_args {
    () => { 0usize };
    ($head:expr $(, $tail:expr)*) => { 1 + count_args!($($tail),*) };
}

// hashset! literal
macro_rules! hashset {
    ($($x:expr),* $(,)?) => {
        {
            let mut s = HashSet::new();
            $(s.insert($x);)*
            s
        }
    };
}

// hashmap! literal
macro_rules! hashmap {
    ($($k:expr => $v:expr),* $(,)?) => {
        {
            let mut m = HashMap::new();
            $(m.insert($k, $v);)*
            m
        }
    };
}

// deque! literal
macro_rules! deque {
    ($($x:expr),* $(,)?) => {
        {
            let mut d = VecDeque::new();
            $(d.push_back($x);)*
            d
        }
    };
}

// Custom type with literal macro
struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

macro_rules! matrix {
    [$([$($x:expr),+]),+ $(,)?] => {
        {
            let rows_data: Vec<Vec<f64>> = vec![$( vec![$($x as f64),+] ),+];
            let rows = rows_data.len();
            let cols = rows_data[0].len();
            Matrix {
                rows,
                cols,
                data: rows_data.into_iter().flatten().collect(),
            }
        }
    };
}

impl Matrix {
    fn get(&self, r: usize, c: usize) -> f64 { self.data[r * self.cols + c] }
}

fn main() {
    let v = my_vec![1, 2, 3, 4, 5];
    println!("my_vec: {:?}", v);

    let v2: Vec<&str> = my_vec!["hello", "world",];  // trailing comma ok
    println!("my_vec str: {:?}", v2);

    let mut s = hashset![1, 2, 3, 2, 1];
    s.insert(4);
    println!("hashset size: {} (no dups)", s.len());

    let m = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
    };
    println!("hashmap: {:?}", m["two"]);

    let d = deque![10, 20, 30];
    println!("deque front: {:?}", d.front());

    let mat = matrix![
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9],
    ];
    println!("matrix[1][2] = {}", mat.get(1, 2)); // 6.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_vec() {
        let v = my_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_hashset() {
        let s = hashset![1, 2, 3, 2, 1];
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn test_hashmap() {
        let m = hashmap!["a" => 1, "b" => 2];
        assert_eq!(m["a"], 1);
    }
}
