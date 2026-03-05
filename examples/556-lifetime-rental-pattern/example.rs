//! # 556. Rental / Self-Referential Pattern
//! Owning source data alongside borrowed views — avoiding copies.

/// Pattern: store owned data + indices/metadata (avoids self-referential issues)
struct ParsedDocument {
    source: String,
    // Store positions instead of &str references (avoids self-referential)
    token_spans: Vec<(usize, usize)>, // (start, end) byte positions
}

impl ParsedDocument {
    fn parse(text: &str) -> Self {
        let source = text.to_string();
        let mut token_spans = Vec::new();
        let mut in_word = false;
        let mut word_start = 0;

        for (i, b) in source.bytes().enumerate() {
            let is_space = b == b' ' || b == b'\n' || b == b'\t';
            if !is_space && !in_word {
                word_start = i;
                in_word = true;
            } else if is_space && in_word {
                token_spans.push((word_start, i));
                in_word = false;
            }
        }
        if in_word {
            token_spans.push((word_start, source.len()));
        }

        ParsedDocument { source, token_spans }
    }

    fn tokens(&self) -> impl Iterator<Item = &str> {
        self.token_spans.iter().map(|&(s, e)| &self.source[s..e])
    }

    fn token_count(&self) -> usize { self.token_spans.len() }

    fn get_token(&self, i: usize) -> Option<&str> {
        self.token_spans.get(i).map(|&(s, e)| &self.source[s..e])
    }
}

/// Alternative: use Arc<String> to share source data
use std::sync::Arc;

struct SharedDocument {
    source: Arc<String>,
    tokens: Vec<Arc<String>>, // cloned substrings (small overhead)
}

impl SharedDocument {
    fn parse(text: &str) -> Self {
        let source = Arc::new(text.to_string());
        let tokens = text.split_whitespace()
            .map(|t| Arc::new(t.to_string()))
            .collect();
        SharedDocument { source, tokens }
    }
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    let doc = ParsedDocument::parse(text);

    println!("Parsed '{}' — {} tokens:", text, doc.token_count());
    for (i, token) in doc.tokens().enumerate() {
        println!("  [{}] {:?}", i, token);
    }
    println!("token[3]: {:?}", doc.get_token(3));

    // SharedDocument
    let shared = SharedDocument::parse("hello world rust");
    println!("\nShared doc source: {}", shared.source);
    for t in &shared.tokens {
        print!("{} ", t);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_document() {
        let doc = ParsedDocument::parse("hello world");
        assert_eq!(doc.token_count(), 2);
        assert_eq!(doc.get_token(0), Some("hello"));
        assert_eq!(doc.get_token(1), Some("world"));
    }

    #[test]
    fn test_tokens_iter() {
        let doc = ParsedDocument::parse("a b c");
        let tokens: Vec<&str> = doc.tokens().collect();
        assert_eq!(tokens, ["a", "b", "c"]);
    }
}
