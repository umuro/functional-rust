//! 303. Collecting Iterator<Result<T>> into Result<Vec<T>>
//!
//! `collect::<Result<Vec<T>,E>>()` short-circuits on first Err.

fn main() {
    // All Ok -> Ok(Vec)
    let good: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
    let result: Result<Vec<i32>, &str> = good.into_iter().collect();
    println!("All ok: {:?}", result);

    // One Err -> Err (short-circuits)
    let bad: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Err("oops"), Ok(4)];
    let result: Result<Vec<i32>, &str> = bad.into_iter().collect();
    println!("With error: {:?}", result);

    // Practical: parse all strings
    let inputs = ["1", "2", "3", "4"];
    let parsed: Result<Vec<i32>, _> = inputs.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Parse all: {:?}", parsed);

    // Fail case
    let bad_inputs = ["1", "two", "3"];
    let parsed_bad: Result<Vec<i32>, _> = bad_inputs.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("Parse with bad: {:?}", parsed_bad);

    // Collect and process only if all succeed
    if let Ok(numbers) = (["10", "20", "30"]).iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()
    {
        let sum: i32 = numbers.iter().sum();
        println!("Sum of all: {}", sum);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_collect_all_ok() {
        let result: Result<Vec<i32>, &str> = vec![Ok(1), Ok(2), Ok(3)].into_iter().collect();
        assert_eq!(result, Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_collect_with_err() {
        let result: Result<Vec<i32>, &str> = vec![Ok(1), Err("bad"), Ok(3)].into_iter().collect();
        assert_eq!(result, Err("bad"));
    }

    #[test]
    fn test_collect_parse_ints() {
        let ok: Result<Vec<i32>, _> = ["1","2","3"].iter().map(|s| s.parse::<i32>()).collect();
        assert_eq!(ok.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_collect_parse_fails() {
        let err: Result<Vec<i32>, _> = ["1","x"].iter().map(|s| s.parse::<i32>()).collect();
        assert!(err.is_err());
    }
}
