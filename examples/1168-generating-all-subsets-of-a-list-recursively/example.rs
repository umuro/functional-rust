#![allow(dead_code)]

/// Generate all subsets (power set) of a slice.
pub fn subsets<T: Clone>(items: &[T]) -> Vec<Vec<T>> {
    match items.split_first() {
        None => vec![vec![]],
        Some((first, rest)) => {
            let without = subsets(rest);
            let with_first: Vec<Vec<T>> = without.iter().map(|s| {
                let mut new_s = vec![first.clone()];
                new_s.extend_from_slice(s);
                new_s
            }).collect();
            let mut result = without;
            result.extend(with_first);
            result
        }
    }
}

/// Generate only subsets of size k.
pub fn subsets_of_size<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    subsets(items).into_iter().filter(|s| s.len() == k).collect()
}

fn main() {
    let items = [1, 2, 3_i32];
    let all = subsets(&items);
    println!("subsets([1,2,3]): {} total", all.len());
    let mut sorted = all.clone();
    sorted.sort();
    for s in &sorted {
        println!("  {:?}", s);
    }

    let pairs = subsets_of_size(&[1_i32, 2, 3, 4], 2);
    println!("2-element subsets of [1,2,3,4]: {} (C(4,2)=6)", pairs.len());
}

/* Output:
   subsets([1,2,3]): 8 total
     []
     [1]
     [1, 2]
     [1, 2, 3]
     [1, 3]
     [2]
     [2, 3]
     [3]
   2-element subsets of [1,2,3,4]: 6 (C(4,2)=6)
*/
