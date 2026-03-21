#![allow(dead_code)]
#![allow(clippy::all)]
// 1009: Collecting Results
// Iterator<Item=Result<T,E>> -> Result<Vec<T>, E> via collect()

fn parse_int(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| format!("bad: {}", s))
}

// Approach 1: collect() — the magic of FromIterator for Result
fn parse_all(inputs: &[&str]) -> Result<Vec<i64>, String> {
    inputs.iter().map(|s| parse_int(s)).collect()
}

// Approach 2: Manual fold for clarity
fn parse_all_manual(inputs: &[&str]) -> Result<Vec<i64>, String> {
    let mut results = Vec::new();
    for s in inputs {
        results.push(parse_int(s)?);
    }
    Ok(results)
}

// Approach 3: Using try_fold
fn parse_all_fold(inputs: &[&str]) -> Result<Vec<i64>, String> {
    inputs.iter().try_fold(Vec::new(), |mut acc, s| {
        acc.push(parse_int(s)?);
        Ok(acc)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_all_ok() {
        assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_collect_first_error() {
        let result = parse_all(&["1", "abc", "3"]);
        assert_eq!(result, Err("bad: abc".to_string()));
    }

    #[test]
    fn test_collect_empty() {
        assert_eq!(parse_all(&[]), Ok(vec![]));
    }

    #[test]
    fn test_manual_matches_collect() {
        let inputs = &["10", "20", "30"];
        assert_eq!(parse_all(inputs), parse_all_manual(inputs));

        let bad = &["10", "x"];
        assert!(parse_all_manual(bad).is_err());
    }

    #[test]
    fn test_fold_matches_collect() {
        let inputs = &["5", "10", "15"];
        assert_eq!(parse_all(inputs), parse_all_fold(inputs));
    }

    #[test]
    fn test_short_circuit_behavior() {
        // collect() on Result short-circuits at first Err
        let mut count = 0;
        let result: Result<Vec<i64>, String> = ["1", "bad", "3"]
            .iter()
            .map(|s| {
                count += 1;
                parse_int(s)
            })
            .collect();
        assert!(result.is_err());
        // Iterator is lazy — may stop at error
        assert!(count <= 3);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(parse_all(&["42"]), Ok(vec![42]));
        assert!(parse_all(&["xyz"]).is_err());
    }
}
