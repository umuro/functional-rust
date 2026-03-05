//! # 541. Lifetime Elision Rules
//! When and how Rust infers lifetimes automatically.

// ============================================================
// RULE 1: Each input reference gets its own lifetime
// fn foo(x: &str) → fn foo<'a>(x: &'a str)
// fn bar(x: &str, y: &str) → fn bar<'a, 'b>(x: &'a str, y: &'b str)
// ============================================================

/// Elision rule 1: one input, inferred
fn strlen(s: &str) -> usize { // &str gets lifetime 'a implicitly
    s.len()
}

/// Multiple inputs each get own lifetime — no output ref needed
fn print_both(a: &str, b: &str) {
    println!("{} and {}", a, b);
}

// ============================================================
// RULE 2: If exactly one input lifetime, it flows to output
// fn foo(x: &str) -> &str → fn foo<'a>(x: &'a str) -> &'a str
// ============================================================

/// Elision rule 2: one input &, output tied to it
fn first_word(s: &str) -> &str { // output gets same lifetime as s
    s.split_whitespace().next().unwrap_or("")
}

/// Trim with rule 2
fn trim_prefix<'a>(s: &'a str, prefix: &str) -> &'a str {
    // Two input refs — must annotate: output from s, not prefix
    s.strip_prefix(prefix).unwrap_or(s)
}

// ============================================================
// RULE 3: If &self or &mut self, output tied to self's lifetime
// ============================================================

struct Cache {
    data: Vec<String>,
}

impl Cache {
    /// Rule 3: output tied to self — no annotation needed
    fn get(&self, index: usize) -> Option<&str> { // &str has self's lifetime
        self.data.get(index).map(|s| s.as_str())
    }

    fn first(&self) -> Option<&str> { // same — rule 3
        self.data.first().map(|s| s.as_str())
    }

    /// Returns owned String — avoids lifetime conflict between self and default
    fn get_or(&self, index: usize, default: &str) -> String {
        // When output needs to come from different sources with different lifetimes,
        // returning an owned String is the clean solution
        self.data.get(index)
            .map(|s| s.clone())
            .unwrap_or_else(|| default.to_string())
    }
}

/// When elision DOESN'T apply — must annotate
fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Two input refs, one output ref — which does output borrow from?
    // Elision rule 2 only works with ONE input ref.
    // Rule 3 doesn't apply (no self). Must annotate.
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    // Rule 1: no output ref
    println!("strlen: {}", strlen("hello world"));
    print_both("foo", "bar");

    // Rule 2: one input, output same lifetime
    let s = String::from("hello world foo");
    let word = first_word(&s);
    println!("first_word: {}", word);

    let prefixed = String::from("prefix_value");
    let trimmed = trim_prefix(&prefixed, "prefix_");
    println!("trimmed: {}", trimmed);

    // Rule 3: methods — output tied to self
    let mut cache = Cache { data: vec!["apple".to_string(), "banana".to_string()] };
    println!("first: {:?}", cache.first());
    println!("get(1): {:?}", cache.get(1));

    // When you need explicit annotations
    let s1 = String::from("long string");
    let result;
    {
        let s2 = String::from("s2");
        result = longer(s1.as_str(), s2.as_str());
        println!("longer: {}", result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word_elision() {
        let s = String::from("hello world");
        assert_eq!(first_word(&s), "hello");
    }

    #[test]
    fn test_trim_prefix() {
        let s = String::from("http://example.com");
        assert_eq!(trim_prefix(&s, "http://"), "example.com");
    }

    #[test]
    fn test_cache_rule3() {
        let c = Cache { data: vec!["x".to_string(), "y".to_string()] };
        assert_eq!(c.get(0), Some("x"));
        assert_eq!(c.first(), Some("x"));
        assert_eq!(c.get(99), None);
    }

    #[test]
    fn test_longer_explicit() {
        assert_eq!(longer("hello", "hi"), "hello");
        assert_eq!(longer("ab", "abc"), "abc");
    }
}
