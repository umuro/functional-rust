// String — Trim, Uppercase, Contains
// Common string operations: trim whitespace, change case, find substrings.

// Solution 1: Idiomatic Rust — method chaining on &str
pub fn trim_and_upper(s: &str) -> String {
    s.trim().to_uppercase()
}

pub fn trim_and_lower(s: &str) -> String {
    s.trim().to_lowercase()
}

/// Returns true if `haystack` contains `needle` (case-sensitive).
pub fn contains_substring(haystack: &str, needle: &str) -> bool {
    haystack.contains(needle)
}

// Solution 2: Functional/recursive — mirrors the OCaml manual search
pub fn contains_recursive(s: &str, needle: &str) -> bool {
    fn find(s: &str, needle: &str, i: usize) -> bool {
        if i + needle.len() > s.len() {
            false
        } else if s[i..].starts_with(needle) {
            true
        } else {
            find(s, needle, i + 1)
        }
    }
    find(s, needle, 0)
}

// Solution 3: Iterator window — slides over bytes
pub fn contains_windowed(s: &str, needle: &str) -> bool {
    let n = needle.len();
    if n == 0 {
        return true;
    }
    (0..=s.len().saturating_sub(n)).any(|i| s[i..].starts_with(needle))
}

fn main() {
    let s = "  Hello, World!  ";

    println!("original  : {:?}", s);
    println!("trimmed   : {:?}", s.trim());
    println!("upper     : {:?}", trim_and_upper(s));
    println!("lower     : {:?}", trim_and_lower(s));
    println!(
        "contains 'World' (idiomatic) : {}",
        contains_substring(s, "World")
    );
    println!(
        "contains 'World' (recursive) : {}",
        contains_recursive(s, "World")
    );
    println!(
        "contains 'World' (windowed)  : {}",
        contains_windowed(s, "World")
    );
    println!("contains 'Rust'  (idiomatic) : {}", contains_substring(s, "Rust"));
}

/* Output:
   original  : "  Hello, World!  "
   trimmed   : "Hello, World!"
   upper     : "HELLO, WORLD!"
   lower     : "hello, world!"
   contains 'World' (idiomatic) : true
   contains 'World' (recursive) : true
   contains 'World' (windowed)  : true
   contains 'Rust'  (idiomatic) : false
*/
