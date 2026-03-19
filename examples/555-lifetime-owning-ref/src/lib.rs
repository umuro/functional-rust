//! Owning References Pattern
//!
//! Combining ownership with interior borrowing.

/// Owner with cached view.
pub struct OwnedSlice {
    data: Vec<u8>,
    start: usize,
    end: usize,
}

impl OwnedSlice {
    pub fn new(data: Vec<u8>) -> Self {
        let end = data.len();
        OwnedSlice {
            data,
            start: 0,
            end,
        }
    }

    pub fn slice(&self) -> &[u8] {
        &self.data[self.start..self.end]
    }

    pub fn narrow(&mut self, start: usize, end: usize) {
        self.start = start.min(self.data.len());
        self.end = end.min(self.data.len());
    }
}

/// String with owned and view.
pub struct OwnedStr {
    data: String,
}

impl OwnedStr {
    pub fn new(s: &str) -> Self {
        OwnedStr {
            data: s.to_string(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.data
    }

    pub fn into_string(self) -> String {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owned_slice() {
        let mut slice = OwnedSlice::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(slice.slice(), &[1, 2, 3, 4, 5]);
        slice.narrow(1, 4);
        assert_eq!(slice.slice(), &[2, 3, 4]);
    }

    #[test]
    fn test_owned_str() {
        let s = OwnedStr::new("hello");
        assert_eq!(s.as_str(), "hello");
        assert_eq!(s.into_string(), "hello");
    }
}
