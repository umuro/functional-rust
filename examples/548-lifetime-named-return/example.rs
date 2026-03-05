//! # 548. Named Lifetime in Return Types
//! Explicitly connecting output reference lifetime to input.

/// Named lifetime: output borrows from input slice
fn max_element<'a>(slice: &'a [i32]) -> Option<&'a i32> {
    slice.iter().max()
}

/// Named lifetime: output from first arg (not second)
fn longest_word<'a>(text: &'a str, _separator: &str) -> &'a str {
    text.split_whitespace()
        .max_by_key(|w| w.len())
        .unwrap_or("")
}

/// Complex: output from either input — must use same lifetime
fn first_non_empty<'a>(a: &'a str, b: &'a str) -> &'a str {
    if !a.is_empty() { a } else { b }
}

/// Struct method returning reference to field
struct Document {
    title: String,
    content: String,
    tags: Vec<String>,
}

impl Document {
    /// Returns &str from self — rule 3 (elision works here)
    fn title(&self) -> &str { &self.title }
    fn content(&self) -> &str { &self.content }

    /// Returns from tags — elision works (rule 3)
    fn first_tag(&self) -> Option<&str> {
        self.tags.first().map(|s| s.as_str())
    }

    /// Named: explicitly returning from argument, not self
    fn find_in_content<'doc, 'pattern>(
        &'doc self,
        pattern: &'pattern str,
    ) -> Option<&'doc str> {
        // Returns a slice of self.content (tied to 'doc, not 'pattern)
        self.content.find(pattern).map(|i| &self.content[i..i+pattern.len()])
    }
}

/// Named lifetime in generic function
fn get_field<'a, T>(container: &'a [T], index: usize) -> Option<&'a T> {
    container.get(index)
}

/// Output tied to first arg (the "view" arg), not config
fn apply_view<'view>(
    data: &'view [i32],
    _config: &str, // not in output
) -> &'view [i32] {
    // Return a subslice — still borrows from data
    let end = data.len().min(5);
    &data[..end]
}

fn main() {
    // max_element: output from slice
    let nums = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("max: {:?}", max_element(&nums));

    // longest_word: output from first arg
    let text = String::from("the quick brown fox jumps");
    let config = String::from("separator_config");
    let word = longest_word(&text, &config);
    drop(config); // config gone — but word borrows from text, not config!
    println!("longest: {}", word);

    // first_non_empty: both must live at least as long as result
    let a = String::from("hello");
    let b = String::from("world");
    println!("first_non_empty: {}", first_non_empty(&a, &b));
    println!("first_non_empty: {}", first_non_empty("", &b));

    // Document methods
    let doc = Document {
        title: "Rust Lifetimes".to_string(),
        content: "Lifetimes are annotations that...".to_string(),
        tags: vec!["rust".to_string(), "programming".to_string()],
    };
    println!("title: {}", doc.title());
    println!("first_tag: {:?}", doc.first_tag());
    println!("find 'are': {:?}", doc.find_in_content("are"));

    // apply_view
    let big_data: Vec<i32> = (0..20).collect();
    let config = "config_string".to_string();
    let view = apply_view(&big_data, &config);
    drop(config); // config dropped — view borrows big_data, not config
    println!("view: {:?}", view);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_element() {
        let v = vec![5, 2, 8, 1, 9, 3];
        assert_eq!(max_element(&v), Some(&9));
    }

    #[test]
    fn test_longest_word() {
        let text = String::from("the quick brown fox");
        let r = longest_word(&text, ",");
        assert_eq!(r, "quick");
    }

    #[test]
    fn test_first_non_empty() {
        assert_eq!(first_non_empty("a", "b"), "a");
        assert_eq!(first_non_empty("", "b"), "b");
    }

    #[test]
    fn test_document() {
        let doc = Document {
            title: "T".to_string(),
            content: "hello world".to_string(),
            tags: vec!["tag".to_string()],
        };
        assert_eq!(doc.find_in_content("world"), Some("world"));
    }
}
