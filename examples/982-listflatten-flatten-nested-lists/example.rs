// List.flatten — Flatten Nested Lists
// Concatenate a list of lists into a single list

// Solution 1: Idiomatic Rust — flatten() iterator adapter
pub fn flatten_idiomatic<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    nested.iter().flatten().cloned().collect()
}

// Solution 2: Functional/recursive — mirrors OCaml List.flatten structure
pub fn flatten_recursive<T: Clone>(nested: &[Vec<T>]) -> Vec<T> {
    match nested {
        [] => vec![],
        [head, rest @ ..] => {
            let mut result = head.clone();
            result.extend(flatten_recursive(rest));
            result
        }
    }
}

// Solution 3: concat_map — mirrors OCaml List.concat_map
// Applies a function to each element and flattens the results
pub fn concat_map<T, U, F>(items: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> Vec<U>,
{
    items.iter().flat_map(f).collect()
}

fn main() {
    let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6], vec![7, 8, 9, 10]];

    println!(
        "flatten_idiomatic([1,2],[3,4,5],[6],[7..10]) = {:?}",
        flatten_idiomatic(&nested)
    );
    println!(
        "flatten_recursive([1,2],[3,4,5],[6],[7..10]) = {:?}",
        flatten_recursive(&nested)
    );

    // Mirrors OCaml: List.concat_map (fun x -> [x; x * 10]) [1; 2; 3]
    let pairs = concat_map(&[1, 2, 3], |x| vec![*x, x * 10]);
    println!("concat_map(x -> [x; x*10], [1;2;3]) = {:?}", pairs);

    // Filter-and-expand with concat_map
    let evens_doubled = concat_map(&[1, 2, 3, 4], |x| {
        if x % 2 == 0 {
            vec![*x, *x]
        } else {
            vec![]
        }
    });
    println!("keep_and_double_evens([1..4]) = {:?}", evens_doubled);
}

/* Output:
   flatten_idiomatic([1,2],[3,4,5],[6],[7..10]) = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
   flatten_recursive([1,2],[3,4,5],[6],[7..10]) = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
   concat_map(x -> [x; x*10], [1;2;3]) = [1, 10, 2, 20, 3, 30]
   keep_and_double_evens([1..4]) = [2, 2, 4, 4]
*/
