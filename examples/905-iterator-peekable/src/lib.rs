//! 261. Lookahead with Peekable
//!
//! `Peekable` wraps any iterator and adds `peek()` — inspect the next element
//! without consuming it. Essential for parsers, run-length encoding, and
//! merge algorithms.

/// Group consecutive equal elements using `Peekable`.
pub fn group_consecutive<T: PartialEq + Copy>(data: &[T]) -> Vec<Vec<T>> {
    let mut iter = data.iter().peekable();
    let mut groups: Vec<Vec<T>> = Vec::new();

    while let Some(&val) = iter.peek() {
        let mut group = Vec::new();
        while iter.peek() == Some(&val) {
            // Safe: we just peeked a Some
            group.push(*iter.next().unwrap());
        }
        groups.push(group);
    }

    groups
}

/// Simple token type for a minimal tokenizer demo.
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),
    Plus,
    Minus,
    Unknown(char),
}

/// Tokenize a string of digits, `+`, and `-` using `Peekable` for lookahead.
pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '0'..='9' => {
                let mut num_str = String::new();
                while chars.peek().is_some_and(|c| c.is_ascii_digit()) {
                    num_str.push(chars.next().unwrap());
                }
                tokens.push(Token::Number(num_str.parse().unwrap_or(0)));
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            ' ' => {
                chars.next();
            }
            other => {
                chars.next();
                tokens.push(Token::Unknown(other));
            }
        }
    }

    tokens
}

/// Merge two sorted slices using `Peekable` for head comparison.
pub fn merge_sorted(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut ia = a.iter().peekable();
    let mut ib = b.iter().peekable();
    let mut result = Vec::with_capacity(a.len() + b.len());

    loop {
        match (ia.peek(), ib.peek()) {
            (Some(&&x), Some(&&y)) => {
                if x <= y {
                    result.push(x);
                    ia.next();
                } else {
                    result.push(y);
                    ib.next();
                }
            }
            (Some(&&x), None) => {
                result.push(x);
                ia.next();
            }
            (None, Some(&&y)) => {
                result.push(y);
                ib.next();
            }
            (None, None) => break,
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_consecutive_basic() {
        let data = [1i32, 1, 2, 2, 2, 3, 1, 1];
        assert_eq!(
            group_consecutive(&data),
            vec![vec![1, 1], vec![2, 2, 2], vec![3], vec![1, 1]]
        );
    }

    #[test]
    fn test_group_consecutive_all_same() {
        assert_eq!(group_consecutive(&[5i32, 5, 5]), vec![vec![5, 5, 5]]);
    }

    #[test]
    fn test_group_consecutive_all_unique() {
        assert_eq!(
            group_consecutive(&[1i32, 2, 3]),
            vec![vec![1], vec![2], vec![3]]
        );
    }

    #[test]
    fn test_group_consecutive_empty() {
        assert_eq!(group_consecutive::<i32>(&[]), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_tokenize_multi_digit_numbers() {
        assert_eq!(
            tokenize("123 + 45 - 6"),
            vec![
                Token::Number(123),
                Token::Plus,
                Token::Number(45),
                Token::Minus,
                Token::Number(6),
            ]
        );
    }

    #[test]
    fn test_tokenize_empty() {
        assert_eq!(tokenize(""), vec![]);
    }

    #[test]
    fn test_merge_sorted_basic() {
        assert_eq!(merge_sorted(&[1, 3, 5], &[2, 4, 6]), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_merge_sorted_one_empty() {
        assert_eq!(merge_sorted(&[1, 2, 3], &[]), vec![1, 2, 3]);
        assert_eq!(merge_sorted(&[], &[4, 5]), vec![4, 5]);
    }

    #[test]
    fn test_peek_does_not_consume() {
        let mut iter = [10i32, 20, 30].iter().peekable();
        let first_peek = iter.peek().copied();
        let second_peek = iter.peek().copied();
        let consumed = iter.next().copied();
        assert_eq!(first_peek, Some(&10i32));
        assert_eq!(second_peek, Some(&10i32));
        assert_eq!(consumed, Some(10i32));
        assert_eq!(iter.next().copied(), Some(20i32));
    }
}
