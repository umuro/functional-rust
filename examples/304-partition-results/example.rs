//! 304. Splitting Ok/Err with partition()
//!
//! `partition(Result::is_ok)` collects ALL successes and ALL failures in one pass.

fn main() {
    // Partition Results into Ok and Err groups
    let results: Vec<Result<i32, &str>> = vec![
        Ok(1), Err("bad1"), Ok(3), Err("bad2"), Ok(5)
    ];
    let (oks, errs): (Vec<_>, Vec<_>) = results.into_iter().partition(Result::is_ok);
    let ok_vals: Vec<i32> = oks.into_iter().flatten().collect();
    let err_msgs: Vec<&str> = errs.into_iter().map(|r| r.unwrap_err()).collect();
    println!("Ok values: {:?}", ok_vals);
    println!("Errors: {:?}", err_msgs);

    // Parse all, keep track of failures
    let inputs = ["1", "two", "3", "four", "5"];
    let (successes, failures): (Vec<_>, Vec<_>) = inputs.iter()
        .map(|s| s.parse::<i32>().map_err(|_| *s))
        .partition(Result::is_ok);
    let nums: Vec<i32> = successes.into_iter().flatten().collect();
    let bad: Vec<&str> = failures.into_iter().map(|r| r.unwrap_err()).collect();
    println!("Parsed: {:?}", nums);
    println!("Unparseable: {:?}", bad);

    // Report: X succeeded, Y failed
    let items = vec![
        ("alice", "25"),
        ("bob", "not_a_number"),
        ("carol", "30"),
    ];
    let (valid, invalid): (Vec<_>, Vec<_>) = items.iter()
        .map(|(name, age)| age.parse::<u32>().map(|a| (*name, a)).map_err(|_| name))
        .partition(Result::is_ok);
    println!("{} valid, {} invalid", valid.len(), invalid.len());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_partition_results() {
        let v: Vec<Result<i32, &str>> = vec![Ok(1), Err("bad"), Ok(3)];
        let (oks, errs): (Vec<_>, Vec<_>) = v.into_iter().partition(Result::is_ok);
        assert_eq!(oks.len(), 2);
        assert_eq!(errs.len(), 1);
    }

    #[test]
    fn test_partition_all_ok() {
        let v: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2)];
        let (oks, errs): (Vec<_>, Vec<_>) = v.into_iter().partition(Result::is_ok);
        assert_eq!(oks.len(), 2);
        assert!(errs.is_empty());
    }

    #[test]
    fn test_partition_extract_values() {
        let v: Vec<Result<i32, &str>> = vec![Ok(10), Err("x"), Ok(30)];
        let (oks, _): (Vec<_>, Vec<_>) = v.into_iter().partition(Result::is_ok);
        let vals: Vec<i32> = oks.into_iter().flatten().collect();
        assert_eq!(vals, vec![10, 30]);
    }
}
