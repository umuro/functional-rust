#![allow(clippy::all)]
//! Rental Pattern
//!
//! Owning data and borrowing from it simultaneously.

/// Simple rental: owns data and provides view.
pub struct Rental {
    data: String,
}

impl Rental {
    pub fn new(data: &str) -> Self {
        Rental {
            data: data.to_string(),
        }
    }

    pub fn rent(&self) -> &str {
        &self.data
    }

    pub fn rent_slice(&self, start: usize, end: usize) -> &str {
        &self.data[start..end.min(self.data.len())]
    }
}

/// Rental with lazy parsing.
pub struct ParsedRental {
    raw: String,
    parsed: Vec<usize>, // indices into raw
}

impl ParsedRental {
    pub fn new(raw: &str) -> Self {
        let parsed = raw
            .char_indices()
            .filter(|(_, c)| c.is_whitespace())
            .map(|(i, _)| i)
            .collect();
        ParsedRental {
            raw: raw.to_string(),
            parsed,
        }
    }

    pub fn words(&self) -> Vec<&str> {
        let mut words = Vec::new();
        let mut start = 0;
        for &end in &self.parsed {
            if start < end {
                words.push(&self.raw[start..end]);
            }
            start = end + 1;
        }
        if start < self.raw.len() {
            words.push(&self.raw[start..]);
        }
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rental() {
        let r = Rental::new("hello world");
        assert_eq!(r.rent(), "hello world");
        assert_eq!(r.rent_slice(0, 5), "hello");
    }

    #[test]
    fn test_parsed_rental() {
        let r = ParsedRental::new("hello world rust");
        let words = r.words();
        assert_eq!(words, vec!["hello", "world", "rust"]);
    }
}
