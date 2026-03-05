// Example 094: Peekable Iterator
// Lookahead parsing with .peekable() — inspect the next element without consuming it.

// === Approach 1: Idiomatic Rust — consume while condition holds ===
// Uses peek() to decide whether to advance; no "push-back" needed.
pub fn sum_while_positive(data: &[i32]) -> i32 {
    let mut iter = data.iter().peekable();
    let mut sum = 0;
    while iter.peek().is_some_and(|&&v| v > 0) {
        sum += iter.next().unwrap();
    }
    sum
}

// Group consecutive equal elements using peek() to detect group boundaries.
pub fn group_consecutive<T: PartialEq + Clone>(data: &[T]) -> Vec<Vec<T>> {
    let mut iter = data.iter().peekable();
    let mut groups: Vec<Vec<T>> = Vec::new();

    while let Some(item) = iter.next() {
        let mut group = vec![item.clone()];
        // Peek ahead: keep consuming while next equals current
        while iter.peek().is_some_and(|next| *next == item) {
            group.push(iter.next().unwrap().clone());
        }
        groups.push(group);
    }
    groups
}

// === Approach 2: Tokenizer using peekable — multi-char number scanning ===
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Num(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '0'..='9' => {
                // Consume digit run with next_if — the power move
                let mut num_str = String::new();
                while let Some(d) = chars.next_if(|c| c.is_ascii_digit()) {
                    num_str.push(d);
                }
                tokens.push(Token::Num(num_str.parse().unwrap()));
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Minus);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            '/' => {
                chars.next();
                tokens.push(Token::Slash);
            }
            '(' => {
                chars.next();
                tokens.push(Token::LParen);
            }
            ')' => {
                chars.next();
                tokens.push(Token::RParen);
            }
            other => panic!("unexpected character: {other:?}"),
        }
    }
    tokens
}

// === Approach 3: next_if combinator — functional lookahead ===
// Advance only if predicate holds; return None if it doesn't.
pub fn take_while_lt<'a>(
    iter: &mut std::iter::Peekable<impl Iterator<Item = &'a i32>>,
    limit: i32,
) -> Vec<i32> {
    let mut out = Vec::new();
    while let Some(&&v) = iter.peek() {
        if v >= limit {
            break;
        }
        out.push(v);
        iter.next();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- sum_while_positive ---

    #[test]
    fn test_sum_while_positive_all_positive() {
        assert_eq!(sum_while_positive(&[1, 2, 3, 4]), 10);
    }

    #[test]
    fn test_sum_while_positive_stops_at_nonpositive() {
        // stops at 0, ignores trailing values
        assert_eq!(sum_while_positive(&[3, 2, 0, 5]), 5);
    }

    #[test]
    fn test_sum_while_positive_negative_first() {
        assert_eq!(sum_while_positive(&[-1, 2, 3]), 0);
    }

    #[test]
    fn test_sum_while_positive_empty() {
        assert_eq!(sum_while_positive(&[]), 0);
    }

    // --- group_consecutive ---

    #[test]
    fn test_group_consecutive_mixed() {
        let result = group_consecutive(&[1, 1, 2, 3, 3, 3]);
        assert_eq!(result, vec![vec![1, 1], vec![2], vec![3, 3, 3]]);
    }

    #[test]
    fn test_group_consecutive_all_same() {
        let result = group_consecutive(&[7, 7, 7]);
        assert_eq!(result, vec![vec![7, 7, 7]]);
    }

    #[test]
    fn test_group_consecutive_all_distinct() {
        let result = group_consecutive(&[1, 2, 3]);
        assert_eq!(result, vec![vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn test_group_consecutive_empty() {
        let result: Vec<Vec<i32>> = group_consecutive(&[]);
        assert_eq!(result, Vec::<Vec<i32>>::new());
    }

    // --- tokenize ---

    #[test]
    fn test_tokenize_number_and_ops() {
        use Token::*;
        let toks = tokenize("12 + 3 * (4 - 1)");
        assert_eq!(
            toks,
            vec![
                Num(12),
                Plus,
                Num(3),
                Star,
                LParen,
                Num(4),
                Minus,
                Num(1),
                RParen
            ]
        );
    }

    #[test]
    fn test_tokenize_multi_digit_numbers() {
        use Token::*;
        let toks = tokenize("100 / 25");
        assert_eq!(toks, vec![Num(100), Slash, Num(25)]);
    }

    #[test]
    fn test_tokenize_single_number() {
        assert_eq!(tokenize("42"), vec![Token::Num(42)]);
    }

    #[test]
    fn test_tokenize_empty() {
        assert_eq!(tokenize(""), vec![]);
    }

    // --- take_while_lt ---

    #[test]
    fn test_take_while_lt_basic() {
        let data = [1, 2, 3, 10, 11];
        let mut iter = data.iter().peekable();
        let taken = take_while_lt(&mut iter, 5);
        assert_eq!(taken, vec![1, 2, 3]);
        // iterator still has 10, 11
        assert_eq!(iter.next(), Some(&10));
    }

    #[test]
    fn test_take_while_lt_none_qualify() {
        let data = [5, 6, 7];
        let mut iter = data.iter().peekable();
        let taken = take_while_lt(&mut iter, 5);
        assert_eq!(taken, vec![]);
        assert_eq!(iter.next(), Some(&5));
    }
}
