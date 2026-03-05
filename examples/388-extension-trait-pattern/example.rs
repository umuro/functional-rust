// Extension trait pattern in Rust
// Adds methods to types we don't own

trait StrExt {
    fn word_count(&self) -> usize;
    fn capitalize_words(&self) -> String;
    fn is_palindrome(&self) -> bool;
    fn count_char(&self, c: char) -> usize;
    fn truncate_with_ellipsis(&self, max_len: usize) -> String;
}

impl StrExt for str {
    fn word_count(&self) -> usize {
        self.split_whitespace().count()
    }

    fn capitalize_words(&self) -> String {
        self.split_whitespace()
            .map(|w| {
                let mut chars = w.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn is_palindrome(&self) -> bool {
        let chars: Vec<char> = self.chars().collect();
        let rev: Vec<char> = chars.iter().rev().copied().collect();
        chars == rev
    }

    fn count_char(&self, c: char) -> usize {
        self.chars().filter(|&ch| ch == c).count()
    }

    fn truncate_with_ellipsis(&self, max_len: usize) -> String {
        if self.len() <= max_len {
            self.to_string()
        } else {
            format!("{}…", &self[..max_len.saturating_sub(1)])
        }
    }
}

trait VecExt<T> {
    fn second(&self) -> Option<&T>;
    fn chunk_average(&self) -> f64 where T: Into<f64> + Copy;
}

impl<T: Into<f64> + Copy> VecExt<T> for Vec<T> {
    fn second(&self) -> Option<&T> { self.get(1) }
    fn chunk_average(&self) -> f64 {
        if self.is_empty() { return 0.0; }
        self.iter().map(|&x| x.into()).sum::<f64>() / self.len() as f64
    }
}

fn main() {
    let s = "hello world foo bar";
    println!("Word count: {}", s.word_count());
    println!("Capitalize: {}", s.capitalize_words());
    println!("Is 'racecar' palindrome: {}", "racecar".is_palindrome());
    println!("Count 'l': {}", s.count_char('l'));
    println!("Truncate: {}", "Hello, World!".truncate_with_ellipsis(8));

    let v = vec![1.0f64, 2.0, 3.0, 4.0, 5.0];
    println!("Second: {:?}", v.second());
    println!("Average: {}", v.chunk_average());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        assert_eq!("one two three".word_count(), 3);
        assert_eq!("  spaces  ".word_count(), 1);
    }

    #[test]
    fn test_palindrome() {
        assert!("racecar".is_palindrome());
        assert!(!"hello".is_palindrome());
    }

    #[test]
    fn test_capitalize() {
        assert_eq!("hello world".capitalize_words(), "Hello World");
    }
}
