//! List mapping examples: idiomatic Rust iterators and functional recursion.
//!
//! This module demonstrates two approaches to applying a function to each element
//! of a list:
//! - **Idiomatic Rust**: Using iterators with `.iter().map().collect()`
//! - **Functional/Recursive**: Tail-recursive style, similar to OCaml's List.map

/// Apply a function to each element using iterators (idiomatic Rust).
///
/// This is the preferred approach in Rust. It uses iterator adapters which are
/// lazy evaluated, composable, and optimized by the compiler.
///
/// # Arguments
/// * `xs` - A slice of elements
/// * `f` - A function to apply to each element
///
/// # Example
/// ```
/// let numbers = vec![1, 2, 3, 4, 5];
/// let doubled = map_iter(&numbers, |x| x * 2);
/// assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
/// ```
pub fn map_iter<T, U, F>(xs: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    xs.iter().map(f).collect()
}

/// Apply a function to each element using recursion (functional style).
///
/// This approach mirrors OCaml's List.map more closely, using tail recursion.
/// In Rust, this is less idiomatic but demonstrates functional programming patterns.
///
/// # Arguments
/// * `xs` - A vector of elements (owned, consumed)
/// * `f` - A function to apply to each element
///
/// # Example
/// ```
/// let numbers = vec![1, 2, 3, 4, 5];
/// let doubled = map_recursive(numbers, |x| x * 2);
/// assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
/// ```
pub fn map_recursive<T, U, F>(xs: Vec<T>, f: F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    fn go<T, U, F>(mut xs: Vec<T>, f: &F, mut acc: Vec<U>) -> Vec<U>
    where
        F: Fn(T) -> U,
    {
        if xs.is_empty() {
            acc
        } else {
            let head = xs.remove(0);
            acc.push(f(head));
            go(xs, f, acc)
        }
    }

    go(xs, &f, Vec::new())
}

/// Alternative recursive implementation using pattern matching on the vector directly.
/// This version is more elegant but requires consuming the vector via conversion.
pub fn map_recursive_match<T, U, F>(mut xs: Vec<T>, f: &F) -> Vec<U>
where
    F: Fn(T) -> U,
{
    if xs.is_empty() {
        Vec::new()
    } else {
        let head = xs.remove(0);
        let mut tail_result = map_recursive_match(xs, f);
        let mut result = vec![f(head)];
        result.append(&mut tail_result);
        result
    }
}

fn main() {
    // OCaml original:
    // let numbers = [1; 2; 3; 4; 5]
    // let doubled = List.map (fun x -> x * 2) numbers
    // let () = List.iter (fun x -> Printf.printf "%d " x) doubled
    // Output: 2 4 6 8 10

    println!("=== Idiomatic Rust (Iterators) ===");
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled = map_iter(&numbers, |x| x * 2);
    print!("Doubled: ");
    for n in &doubled {
        print!("{} ", n);
    }
    println!();

    println!("\n=== Functional Rust (Recursive) ===");
    let numbers2 = vec![1, 2, 3, 4, 5];
    let doubled2 = map_recursive(numbers2, |x| x * 2);
    print!("Doubled: ");
    for n in &doubled2 {
        print!("{} ", n);
    }
    println!();

    println!("\n=== Alternative Recursive Implementation ===");
    let numbers3 = vec![1, 2, 3, 4, 5];
    let doubled3 = map_recursive_match(numbers3, &|x| x * 2);
    print!("Doubled: ");
    for n in &doubled3 {
        print!("{} ", n);
    }
    println!();

    println!("\n=== Type Conversion Example ===");
    let numbers4 = vec![1, 2, 3, 4, 5];
    let as_strings = map_iter(&numbers4, |x| x.to_string());
    print!("As strings: ");
    for s in &as_strings {
        print!("{} ", s);
    }
    println!();

    println!("\n=== Squaring Example ===");
    let numbers5 = vec![1, 2, 3, 4, 5];
    let squared = map_iter(&numbers5, |x| x * x);
    print!("Squared: ");
    for n in &squared {
        print!("{} ", n);
    }
    println!();
}
