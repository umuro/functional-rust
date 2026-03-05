// Traversable in Rust

fn traverse_option<A, B, F>(xs: Vec<A>, mut f: F) -> Option<Vec<B>>
where F: FnMut(A) -> Option<B> {
    let mut result = Vec::new();
    for x in xs {
        result.push(f(x)?);
    }
    Some(result)
}

fn sequence_option<A>(xs: Vec<Option<A>>) -> Option<Vec<A>> {
    traverse_option(xs, |x| x)
}

fn main() {
    // Parse all numbers or fail
    let strings = vec!["1", "2", "3"];
    let parsed = traverse_option(strings, |s| s.parse::<i32>().ok());
    println!("Parsed: {:?}", parsed);
    
    // With invalid input
    let strings = vec!["1", "not a number", "3"];
    let parsed = traverse_option(strings, |s| s.parse::<i32>().ok());
    println!("Failed: {:?}", parsed);
}
