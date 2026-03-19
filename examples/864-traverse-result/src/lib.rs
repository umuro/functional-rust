#![allow(clippy::all)]
// Example 065: Traverse with Result
// Turn Vec<Result<T,E>> into Result<Vec<T>,E>

// Approach 1: Using collect (Rust's built-in traverse for Result!)
fn traverse_result<T, U, E, F: Fn(&T) -> Result<U, E>>(xs: &[T], f: F) -> Result<Vec<U>, E> {
    xs.iter().map(f).collect()
}

// Approach 2: Using try_fold
fn traverse_result_fold<T, U, E, F: Fn(&T) -> Result<U, E>>(xs: &[T], f: F) -> Result<Vec<U>, E> {
    xs.iter().try_fold(Vec::new(), |mut acc, x| {
        acc.push(f(x)?);
        Ok(acc)
    })
}

// Approach 3: Sequence
fn sequence_result<T, E>(xs: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    xs.into_iter().collect()
}

fn parse_positive(s: &&str) -> Result<i32, String> {
    let n: i32 = s.parse().map_err(|_| format!("Not a number: {}", s))?;
    if n <= 0 {
        Err(format!("Not positive: {}", n))
    } else {
        Ok(n)
    }
}

fn validate_username(s: &&str) -> Result<String, String> {
    if s.len() < 3 {
        Err("Too short".into())
    } else if s.len() > 20 {
        Err("Too long".into())
    } else {
        Ok(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse_all_ok() {
        assert_eq!(
            traverse_result(&["1", "2", "3"], parse_positive),
            Ok(vec![1, 2, 3])
        );
    }

    #[test]
    fn test_traverse_parse_error() {
        assert_eq!(
            traverse_result(&["1", "bad", "3"], parse_positive),
            Err("Not a number: bad".into())
        );
    }

    #[test]
    fn test_traverse_validation_error() {
        assert_eq!(
            traverse_result(&["1", "-2", "3"], parse_positive),
            Err("Not positive: -2".into())
        );
    }

    #[test]
    fn test_traverse_empty() {
        let empty: &[&str] = &[];
        assert_eq!(traverse_result(empty, parse_positive), Ok(vec![]));
    }

    #[test]
    fn test_fold_version() {
        assert_eq!(
            traverse_result_fold(&["1", "2"], parse_positive),
            Ok(vec![1, 2])
        );
        assert_eq!(
            traverse_result_fold(&["1", "bad"], parse_positive),
            Err("Not a number: bad".into())
        );
    }

    #[test]
    fn test_sequence_ok() {
        assert_eq!(
            sequence_result::<i32, String>(vec![Ok(1), Ok(2), Ok(3)]),
            Ok(vec![1, 2, 3])
        );
    }

    #[test]
    fn test_sequence_err() {
        let rs: Vec<Result<i32, &str>> = vec![Ok(1), Err("e"), Ok(3)];
        assert_eq!(sequence_result(rs), Err("e"));
    }

    #[test]
    fn test_validate_usernames() {
        assert_eq!(
            traverse_result(&["alice", "bob"], validate_username),
            Ok(vec!["alice".into(), "bob".into()])
        );
        assert_eq!(
            traverse_result(&["alice", "ab"], validate_username),
            Err("Too short".into())
        );
    }
}
