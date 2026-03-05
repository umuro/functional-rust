//! # 557. Output Lifetimes in Traits
//! Lifetime parameters in trait method return types.

/// Trait with lifetime in output
trait Extractor<'a> {
    type Output;
    fn extract(&self, source: &'a str) -> Self::Output;
}

struct WordExtractor;
impl<'a> Extractor<'a> for WordExtractor {
    type Output = Vec<&'a str>;
    fn extract(&self, source: &'a str) -> Vec<&'a str> {
        source.split_whitespace().collect()
    }
}

/// Trait with self lifetime in output
trait AsRef2 {
    fn as_str(&self) -> &str;
}

struct Wrapper(String);
impl AsRef2 for Wrapper {
    fn as_str(&self) -> &str { &self.0 }
}

/// Generic function using trait with output lifetime
fn extract_and_print<'a>(extractor: &impl Extractor<'a, Output = Vec<&'a str>>, text: &'a str) {
    let words = extractor.extract(text);
    println!("extracted {} words: {:?}", words.len(), words);
}

fn main() {
    let text = String::from("hello world rust programming");
    let extractor = WordExtractor;
    extract_and_print(&extractor, &text);

    let w = Wrapper("wrapped string".to_string());
    println!("as_str: {}", w.as_str());

    // Lifetime in return position of trait method
    let s = String::from("output lifetime demo");
    let words = extractor.extract(&s);
    println!("words still valid: {:?}", &words[..2]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_word_extractor() {
        let text = "a b c d";
        let e = WordExtractor;
        let words = e.extract(text);
        assert_eq!(words, vec!["a", "b", "c", "d"]);
    }
    #[test]
    fn test_wrapper() {
        let w = Wrapper("hello".to_string());
        assert_eq!(w.as_str(), "hello");
    }
}
