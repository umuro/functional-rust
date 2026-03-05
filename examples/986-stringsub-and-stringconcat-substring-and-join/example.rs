/// Idiomatic Rust: extract a substring by byte position and length.
///
/// Returns `None` if the range is out of bounds or not on a char boundary.
pub fn substring(s: &str, pos: usize, len: usize) -> Option<&str> {
    s.get(pos..pos + len)
}

/// Functional style: same operation built from char iterators.
///
/// Works on Unicode char boundaries rather than raw bytes.
pub fn substring_chars(s: &str, pos: usize, len: usize) -> String {
    s.chars().skip(pos).take(len).collect()
}

/// Idiomatic Rust: join an iterable of string slices with a separator.
///
/// Mirrors OCaml's `String.concat sep parts`.
pub fn join(sep: &str, parts: &[&str]) -> String {
    parts.join(sep)
}

/// Functional style: join using an iterator fold, making the reduction explicit.
pub fn join_iter(sep: &str, parts: &[&str]) -> String {
    let mut iter = parts.iter().copied();
    match iter.next() {
        None => String::new(),
        Some(first) => iter.fold(first.to_owned(), |mut acc, part| {
            acc.push_str(sep);
            acc.push_str(part);
            acc
        }),
    }
}

fn main() {
    let s = "Hello, World!";
    let hello = substring(s, 0, 5);
    let world = substring(s, 7, 5);
    println!("'{}' and '{}'", hello.unwrap(), world.unwrap());

    let parts = &["one", "two", "three"];
    let joined = join(" | ", parts);
    println!("Joined: {}", joined);

    let joined_iter = join_iter(" | ", parts);
    println!("Joined (fold): {}", joined_iter);

    // Unicode char-safe substring
    let caf = substring_chars("café", 0, 3);
    let accent = substring_chars("café", 3, 1);
    println!("Char substring: '{}' + '{}'", caf, accent);
}

/* Output:
   'Hello' and 'World'
   Joined: one | two | three
   Joined (fold): one | two | three
   Char substring: 'caf' + 'é'
*/
