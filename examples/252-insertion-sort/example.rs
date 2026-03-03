// Solution 1: Idiomatic Rust — in-place insertion sort using slice swaps
pub fn insertion_sort_inplace<T: Ord>(data: &mut [T]) {
    for i in 1..data.len() {
        let mut j = i;
        while j > 0 && data[j - 1] > data[j] {
            data.swap(j - 1, j);
            j -= 1;
        }
    }
}

// Solution 2: Functional — mirrors OCaml's fold structure
pub fn insertion_sort_functional<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    list.iter().cloned().fold(Vec::new(), |mut acc, x| {
        let pos = acc.partition_point(|h| h < &x);
        acc.insert(pos, x);
        acc
    })
}

// Solution 3: Recursive — mirrors OCaml's `insert` function
pub fn insert_rec<T: Ord + Clone>(x: T, list: &[T]) -> Vec<T> {
    match list {
        [] => vec![x],
        [h, rest @ ..] => {
            if x <= *h {
                let mut result = vec![x];
                result.extend_from_slice(list);
                result
            } else {
                let mut result = vec![h.clone()];
                result.extend(insert_rec(x, rest));
                result
            }
        }
    }
}

pub fn insertion_sort_recursive<T: Ord + Clone>(list: &[T]) -> Vec<T> {
    list.iter()
        .cloned()
        .fold(Vec::new(), |acc, x| insert_rec(x, &acc))
}

fn main() {
    let input = [5, 3, 1, 4, 2];

    // In-place
    let mut data = input;
    insertion_sort_inplace(&mut data);
    println!("inplace:    {:?}", data);

    // Functional fold
    let sorted = insertion_sort_functional(&input);
    println!("functional: {:?}", sorted);

    // Recursive
    let sorted = insertion_sort_recursive(&input);
    println!("recursive:  {:?}", sorted);

    // Works on strings too
    let words = ["banana", "apple", "cherry", "date"];
    println!("strings:    {:?}", insertion_sort_functional(&words));
}

/* Output:
   inplace:    [1, 2, 3, 4, 5]
   functional: [1, 2, 3, 4, 5]
   recursive:  [1, 2, 3, 4, 5]
   strings:    ["apple", "banana", "cherry", "date"]
*/
