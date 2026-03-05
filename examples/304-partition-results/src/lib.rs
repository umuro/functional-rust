//! # Splitting Ok/Err with partition()
//!
//! `partition(Result::is_ok)` collects ALL successes and ALL failures in one pass.

/// Partition results into successes and failures
pub fn partition_results<T: std::fmt::Debug, E: std::fmt::Debug>(results: Vec<Result<T, E>>) -> (Vec<T>, Vec<E>) {
    let (oks, errs): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
    let ok_vals: Vec<T> = oks.into_iter().map(|r| r.unwrap()).collect();
    let err_vals: Vec<E> = errs.into_iter().map(|r| r.unwrap_err()).collect();
    (ok_vals, err_vals)
}

/// Parse all strings, collecting both successes and failures
pub fn parse_all_report(inputs: &[&str]) -> (Vec<i32>, Vec<String>) {
    let results: Vec<Result<i32, String>> = inputs
        .iter()
        .map(|s| s.parse::<i32>().map_err(|_| s.to_string()))
        .collect();
    partition_results(results)
}

/// Alternative using fold for more control
pub fn partition_fold<T, E>(results: Vec<Result<T, E>>) -> (Vec<T>, Vec<E>) {
    results.into_iter().fold((vec![], vec![]), |(mut oks, mut errs), r| {
        match r {
            Ok(v) => oks.push(v),
            Err(e) => errs.push(e),
        }
        (oks, errs)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_results() {
        let v: Vec<Result<i32, &str>> = vec![Ok(1), Err("bad"), Ok(3)];
        let (oks, errs) = partition_results(v);
        assert_eq!(oks, vec![1, 3]);
        assert_eq!(errs, vec!["bad"]);
    }

    #[test]
    fn test_partition_all_ok() {
        let v: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2)];
        let (oks, errs) = partition_results(v);
        assert_eq!(oks, vec![1, 2]);
        assert!(errs.is_empty());
    }

    #[test]
    fn test_parse_all_report() {
        let (nums, bad) = parse_all_report(&["1", "two", "3", "four"]);
        assert_eq!(nums, vec![1, 3]);
        assert_eq!(bad, vec!["two", "four"]);
    }

    #[test]
    fn test_partition_fold() {
        let v: Vec<Result<i32, &str>> = vec![Ok(1), Err("x"), Ok(2)];
        let (oks, errs) = partition_fold(v);
        assert_eq!(oks, vec![1, 2]);
        assert_eq!(errs, vec!["x"]);
    }

    #[test]
    fn test_empty_input() {
        let v: Vec<Result<i32, &str>> = vec![];
        let (oks, errs) = partition_results(v);
        assert!(oks.is_empty());
        assert!(errs.is_empty());
    }
}
