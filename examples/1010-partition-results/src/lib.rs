#![allow(dead_code)]
#![allow(clippy::all)]
// 1010: Partition Results
// Separate Ok and Err values using Iterator::partition

fn parse_int(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| format!("bad: {}", s))
}

// Approach 1: partition into two Vecs of Results, then unwrap
fn partition_results(inputs: &[&str]) -> (Vec<i64>, Vec<String>) {
    let (oks, errs): (Vec<Result<i64, String>>, Vec<Result<i64, String>>) =
        inputs.iter().map(|s| parse_int(s)).partition(Result::is_ok);

    (
        oks.into_iter().map(Result::unwrap).collect(),
        errs.into_iter().map(Result::unwrap_err).collect(),
    )
}

// Approach 2: Single fold into two accumulators
fn partition_fold(inputs: &[&str]) -> (Vec<i64>, Vec<String>) {
    inputs.iter().map(|s| parse_int(s)).fold(
        (Vec::new(), Vec::new()),
        |(mut oks, mut errs), result| {
            match result {
                Ok(v) => oks.push(v),
                Err(e) => errs.push(e),
            }
            (oks, errs)
        },
    )
}

// Approach 3: Using filter_map for just one side
fn only_successes(inputs: &[&str]) -> Vec<i64> {
    inputs.iter().filter_map(|s| parse_int(s).ok()).collect()
}

fn only_errors(inputs: &[&str]) -> Vec<String> {
    inputs.iter().filter_map(|s| parse_int(s).err()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_mixed() {
        let (oks, errs) = partition_results(&["1", "abc", "3", "def", "5"]);
        assert_eq!(oks, vec![1, 3, 5]);
        assert_eq!(errs, vec!["bad: abc", "bad: def"]);
    }

    #[test]
    fn test_partition_all_ok() {
        let (oks, errs) = partition_results(&["1", "2", "3"]);
        assert_eq!(oks, vec![1, 2, 3]);
        assert!(errs.is_empty());
    }

    #[test]
    fn test_partition_all_err() {
        let (oks, errs) = partition_results(&["a", "b", "c"]);
        assert!(oks.is_empty());
        assert_eq!(errs.len(), 3);
    }

    #[test]
    fn test_fold_matches_partition() {
        let inputs = &["1", "abc", "3"];
        assert_eq!(partition_results(inputs), partition_fold(inputs));
    }

    #[test]
    fn test_filter_map_successes() {
        assert_eq!(only_successes(&["1", "x", "3"]), vec![1, 3]);
    }

    #[test]
    fn test_filter_map_errors() {
        assert_eq!(only_errors(&["1", "x", "3"]), vec!["bad: x"]);
    }

    #[test]
    fn test_empty_input() {
        let (oks, errs) = partition_results(&[]);
        assert!(oks.is_empty());
        assert!(errs.is_empty());
    }
}
