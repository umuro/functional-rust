// Example 106: Lifetime Elision Rules
//
// Rust has three elision rules that let you skip lifetime annotations
// in the most common cases:
//
//   Rule 1: Each input reference gets its own distinct lifetime.
//   Rule 2: If there is exactly one input lifetime, every output
//           reference gets that same lifetime.
//   Rule 3: If one of the inputs is &self or &mut self, every output
//           reference gets self's lifetime.
//
// When the rules produce an unambiguous answer you write nothing.
// When they don't, the compiler asks you to be explicit.

// ── Approach 1: single input reference (rule 2 applies) ──────────────
// The compiler expands this to:
//   fn first_word<'a>(s: &'a str) -> &'a str
// You write nothing — the relationship is obvious.
pub fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or(s)
}

// Two input references → rule 2 cannot apply (ambiguous source).
// We must spell out which input the output borrows from.
pub fn pick_first<'a>(a: &'a str, _b: &str) -> &'a str {
    a
}

// ── Approach 2: method with &self (rule 3 applies) ───────────────────
// The compiler expands `fn get_content(&self) -> &str` to
//   fn get_content<'a>(&'a self) -> &'a str
pub struct TextBuffer {
    content: String,
}

impl TextBuffer {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_owned(),
        }
    }

    // Rule 3: output borrows from self — no annotation needed.
    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_length(&self) -> usize {
        self.content.len()
    }

    // Rule 3 still applies even when we also take another reference.
    // The returned slice is guaranteed to live as long as `self`.
    pub fn trim_to(&self, max: usize) -> &str {
        let end = max.min(self.content.len());
        &self.content[..end]
    }
}

// ── Approach 3: struct holding a reference (explicit lifetime required)
// Elision rules don't cover struct fields — you must write 'a.
pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    // Rule 3: output borrows from self, so the return gets self's lifetime.
    pub fn content(&self) -> &str {
        self.text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── first_word ────────────────────────────────────────────────────

    #[test]
    fn test_first_word_with_space() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn test_first_word_single_word() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn test_first_word_empty() {
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_first_word_multiple_words() {
        assert_eq!(first_word("one two three"), "one");
    }

    // ── pick_first (explicit lifetime) ───────────────────────────────

    #[test]
    fn test_pick_first_returns_a() {
        let a = String::from("abcde");
        let result;
        {
            let b = String::from("xy");
            // result borrows from `a` (lifetime 'a), so b can end here.
            result = pick_first(&a, &b);
        }
        assert_eq!(result, "abcde");
    }

    // ── TextBuffer ────────────────────────────────────────────────────

    #[test]
    fn test_text_buffer_get_content() {
        let buf = TextBuffer::new("Hello, World!");
        assert_eq!(buf.get_content(), "Hello, World!");
    }

    #[test]
    fn test_text_buffer_get_length() {
        let buf = TextBuffer::new("Rust");
        assert_eq!(buf.get_length(), 4);
    }

    #[test]
    fn test_text_buffer_trim_to() {
        let buf = TextBuffer::new("lifetime elision");
        assert_eq!(buf.trim_to(8), "lifetime");
    }

    #[test]
    fn test_text_buffer_trim_to_beyond_length() {
        let buf = TextBuffer::new("short");
        assert_eq!(buf.trim_to(100), "short");
    }

    // ── Excerpt ───────────────────────────────────────────────────────

    #[test]
    fn test_excerpt_content() {
        let text = String::from("We choose to go to the Moon.");
        let ex = Excerpt::new(&text);
        assert_eq!(ex.content(), "We choose to go to the Moon.");
    }

    #[test]
    fn test_excerpt_text_field() {
        let s = "four score and seven years";
        let ex = Excerpt::new(s);
        assert_eq!(ex.text, s);
    }
}
