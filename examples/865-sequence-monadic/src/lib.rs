// Example 066: Sequence Monadic
// Turn a collection of monadic values into a monadic collection

// Approach 1: sequence for Option using collect
fn sequence_option<T>(xs: Vec<Option<T>>) -> Option<Vec<T>> {
    xs.into_iter().collect()
}

// Approach 2: sequence for Result using collect
fn sequence_result<T, E>(xs: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    xs.into_iter().collect()
}

// Approach 3: Manual fold implementation
fn sequence_option_fold<T>(xs: Vec<Option<T>>) -> Option<Vec<T>> {
    xs.into_iter().try_fold(Vec::new(), |mut acc, x| {
        acc.push(x?);
        Some(acc)
    })
}

fn sequence_result_fold<T, E>(xs: Vec<Result<T, E>>) -> Result<Vec<T>, E> {
    xs.into_iter().try_fold(Vec::new(), |mut acc, x| {
        acc.push(x?);
        Ok(acc)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_option_all_some() {
        assert_eq!(
            sequence_option(vec![Some(1), Some(2), Some(3)]),
            Some(vec![1, 2, 3])
        );
    }

    #[test]
    fn test_sequence_option_with_none() {
        assert_eq!(sequence_option(vec![Some(1), None, Some(3)]), None);
    }

    #[test]
    fn test_sequence_option_empty() {
        assert_eq!(sequence_option::<i32>(vec![]), Some(vec![]));
    }

    #[test]
    fn test_sequence_result_all_ok() {
        assert_eq!(
            sequence_result::<i32, String>(vec![Ok(1), Ok(2), Ok(3)]),
            Ok(vec![1, 2, 3])
        );
    }

    #[test]
    fn test_sequence_result_with_err() {
        let rs: Vec<Result<i32, &str>> = vec![Ok(1), Err("e"), Ok(3)];
        assert_eq!(sequence_result(rs), Err("e"));
    }

    #[test]
    fn test_fold_versions() {
        assert_eq!(
            sequence_option_fold(vec![Some(1), Some(2)]),
            Some(vec![1, 2])
        );
        assert_eq!(sequence_option_fold(vec![Some(1), None]), None);
        assert_eq!(
            sequence_result_fold::<i32, String>(vec![Ok(1), Ok(2)]),
            Ok(vec![1, 2])
        );
    }

    #[test]
    fn test_practical_parse() {
        let parsed: Option<Vec<i32>> = vec!["1", "2", "3"].iter().map(|s| s.parse().ok()).collect();
        assert_eq!(parsed, Some(vec![1, 2, 3]));

        let parsed2: Option<Vec<i32>> = vec!["1", "bad", "3"]
            .iter()
            .map(|s| s.parse().ok())
            .collect();
        assert_eq!(parsed2, None);
    }
}
