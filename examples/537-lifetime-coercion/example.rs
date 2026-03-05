//! # 537. Lifetime Coercion and Subtyping
//! Longer lifetimes can be used where shorter ones are required.

/// Accepts a reference valid for at least 'short duration
fn use_briefly<'short>(s: &'short str) {
    println!("Brief use: {}", s);
}

/// Requires long-lived reference
fn store_long<'long: 'short, 'short>(
    storage: &mut Vec<&'short str>,
    item: &'long str, // 'long satisfies 'short requirement
) {
    storage.push(item); // coercion: &'long str -> &'short str
}

/// Demonstrate implicit coercion: longer lifetime used as shorter
fn coercion_demo() {
    let long_lived = String::from("I live a long time");
    let result;
    {
        // short_lived has a shorter scope
        let _short_lived = String::from("I'm short");
        // &long_lived has lifetime 'long > 'short
        // We can use it where 'short is required:
        result = use_briefly(long_lived.as_str()); // coercion happens here
        // result = use_briefly(_short_lived.as_str()); // also works inside scope
    }
    println!("After short scope: {}", long_lived); // still valid
    let _ = result;
}

/// Lifetime narrowing in function calls
fn narrowing_example<'a>(s: &'a str) -> &'a str {
    // Inside here, we can use s as &'shorter str freely
    // The compiler narrows as needed
    &s[..s.len()]
}

/// Collection that stores references — lifetime of stored refs matters
struct Cache<'a> {
    entries: Vec<&'a str>,
}

impl<'a> Cache<'a> {
    fn new() -> Self { Cache { entries: Vec::new() } }

    // Only accepts refs that live at least as long as 'a
    fn insert(&mut self, entry: &'a str) {
        self.entries.push(entry);
    }

    fn first(&self) -> Option<&&'a str> {
        self.entries.first()
    }
}

fn main() {
    coercion_demo();

    // Lifetime narrowing: &'longer used as &'shorter
    let data = String::from("Hello, World!");
    let word = narrowing_example(&data);
    println!("word: {}", word);

    // Cache: entries must outlive the cache
    let s1 = String::from("entry one");
    let s2 = String::from("entry two");

    let mut cache = Cache::new();
    cache.insert(&s1);
    cache.insert(&s2);
    println!("Cache first: {:?}", cache.first());

    // Coercion in assignment
    let long: &'static str = "static string";
    let shorter: &str = long; // &'static coerces to &'any
    println!("Coerced: {}", shorter);

    // Store long-lived refs in short-context
    let mut storage: Vec<&str> = Vec::new();
    let permanent = "permanent data"; // &'static
    store_long(&mut storage, permanent);
    println!("Storage: {:?}", storage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_coerces_to_any() {
        let s: &'static str = "hello";
        // &'static satisfies any lifetime requirement
        let _: &str = s; // coercion to 'shorter
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_cache_insert() {
        let a = String::from("a");
        let b = String::from("b");
        let mut cache = Cache::new();
        cache.insert(&a);
        cache.insert(&b);
        assert_eq!(cache.entries.len(), 2);
    }

    #[test]
    fn test_narrowing() {
        let s = String::from("test");
        let r = narrowing_example(&s);
        assert_eq!(r, "test");
    }
}
