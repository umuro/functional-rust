//! Extension Trait Pattern

pub trait StrExt {
    fn word_count(&self) -> usize;
    fn capitalize_words(&self) -> String;
    fn is_palindrome(&self) -> bool;
}

impl StrExt for str {
    fn word_count(&self) -> usize {
        self.split_whitespace().count()
    }
    fn capitalize_words(&self) -> String {
        self.split_whitespace()
            .map(|w| {
                let mut c = w.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().to_string() + c.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    fn is_palindrome(&self) -> bool {
        let chars: Vec<char> = self.chars().collect();
        chars.iter().eq(chars.iter().rev())
    }
}

pub trait VecExt<T> {
    fn second(&self) -> Option<&T>;
}
impl<T> VecExt<T> for Vec<T> {
    fn second(&self) -> Option<&T> {
        self.get(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        assert_eq!("hello world".word_count(), 2);
    }
    #[test]
    fn test_capitalize() {
        assert_eq!("hello world".capitalize_words(), "Hello World");
    }
    #[test]
    fn test_palindrome() {
        assert!("racecar".is_palindrome());
        assert!(!"hello".is_palindrome());
    }
    #[test]
    fn test_second() {
        assert_eq!(vec![1, 2, 3].second(), Some(&2));
    }
}
