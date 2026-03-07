// Idiomatic Rust: use the iterator-based map directly from std
// This is how Rust developers write it — leveraging the standard library
pub fn map_idiomatic<T, U, F>(f: F, items: &[T]) -> Vec<U>
where
    F: Fn(T) -> U,
    T: Copy,
{
    items.iter().map(|&x| f(x)).collect()
}

// Functional/recursive: explicit recursion similar to OCaml
// Shows the abstraction principle: we extract the common pattern (apply f to each element)
pub fn map_recursive<T, U, F>(f: F, items: &[T]) -> Vec<U>
where
    F: Fn(T) -> U,
    T: Copy,
{
    match items {
        [] => Vec::new(),
        [head, rest @ ..] => {
            let mut result = vec![f(*head)];
            result.extend(map_recursive(f, rest));
            result
        }
    }
}

// Generic map over slices — the fundamental abstraction
// This demonstrates partial application: we can bind this to create specialized functions
pub fn map<T, U, F>(f: F, items: &[T]) -> Vec<U>
where
    F: Fn(T) -> U,
    T: Copy,
{
    map_idiomatic(f, items)
}

// Partial application examples — creating specialized transformers by binding map with specific functions
pub fn add_one(items: &[i32]) -> Vec<i32> {
    map(|x| x + 1, items)
}

pub fn to_string_int(items: &[i32]) -> Vec<String> {
    map(|x| x.to_string(), items)
}

pub fn double(items: &[i32]) -> Vec<i32> {
    map(|x| x * 2, items)
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    // add_one: applies +1 to each element
    println!("add_one:     {:?}", add_one(&nums));

    // to_string_int: converts each element to string
    println!("to_string:   {:?}", to_string_int(&nums));

    // double: multiplies each element by 2
    println!("double:      {:?}", double(&nums));

    // Comparing idiomatic and recursive implementations
    println!(
        "idiomatic:   {:?}",
        map_idiomatic(|x| x + 10, &nums)
    );
    println!(
        "recursive:   {:?}",
        map_recursive(|x| x + 10, &nums)
    );
}

/* Output:
   add_one:     [2, 3, 4, 5, 6]
   to_string:   ["1", "2", "3", "4", "5"]
   double:      [2, 4, 6, 8, 10]
   idiomatic:   [11, 12, 13, 14, 15]
   recursive:   [11, 12, 13, 14, 15]
*/
