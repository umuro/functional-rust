//! 307. Error propagation in closures
//!
//! `?` in closures requires the closure to return `Result`/`Option`.

fn parse_number(s: &str) -> Result<i32, String> {
    s.trim().parse::<i32>().map_err(|_| format!("not a number: '{}'", s))
}

fn main() {
    // Pattern 1: collect into Result<Vec> (short-circuits on first error)
    let strs = ["1", "2", "3", "4"];
    let numbers: Result<Vec<i32>, String> = strs.iter()
        .map(|s| parse_number(s))
        .collect();
    println!("Pattern 1 (collect): {:?}", numbers);

    // Pattern 2: filter_map for Option (silently drops failures)
    let mixed = ["1", "bad", "3", "also_bad", "5"];
    let valid: Vec<i32> = mixed.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Pattern 2 (filter_map): {:?}", valid);

    // Pattern 3: closure returning Result (can use ?)
    let results: Vec<Result<i32, _>> = mixed.iter()
        .map(|s| -> Result<i32, _> {
            let n = s.trim().parse::<i32>()?;
            if n < 0 { return Err("negative".into()); }
            Ok(n * 2)
        })
        .collect();
    println!("Pattern 3 (collect results): {:?}", results);

    // Pattern 4: try_fold for short-circuit accumulation
    let sum = strs.iter().try_fold(0i32, |acc, s| -> Result<i32, String> {
        Ok(acc + parse_number(s)?)
    });
    println!("Pattern 4 (try_fold sum): {:?}", sum);

    // Pattern 5: extract to named function to use ?
    fn process_all(inputs: &[&str]) -> Result<Vec<i32>, String> {
        inputs.iter().map(|s| parse_number(s)).collect()
    }
    println!("Pattern 5 (named fn): {:?}", process_all(&["10", "20", "30"]));
    println!("Pattern 5 (with err): {:?}", process_all(&["10", "bad"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_all_ok() {
        let result: Result<Vec<i32>, _> = ["1","2","3"].iter()
            .map(|s| parse_number(s)).collect();
        assert_eq!(result.unwrap(), vec![1,2,3]);
    }

    #[test]
    fn test_filter_map_drops_errors() {
        let result: Vec<i32> = ["1","bad","3"].iter()
            .filter_map(|s| s.parse::<i32>().ok()).collect();
        assert_eq!(result, vec![1,3]);
    }

    #[test]
    fn test_try_fold() {
        let sum = ["1","2","3"].iter()
            .try_fold(0i32, |acc, s| -> Result<i32, String> {
                Ok(acc + parse_number(s)?)
            });
        assert_eq!(sum, Ok(6));
    }
}
