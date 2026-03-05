fn parse_int(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| format!("cannot parse: {s}"))
}

fn parse_all(inputs: &[&str]) -> Result<Vec<i64>, String> {
    inputs.iter().map(|s| parse_int(s)).collect()
}

fn parse_best_effort(inputs: &[&str]) -> Vec<i64> {
    inputs.iter().filter_map(|s| parse_int(s).ok()).collect()
}

fn main() {
    let good = ["1", "2", "3"];
    println!("{:?}", parse_all(&good));
    let bad = ["1", "oops", "3"];
    println!("{:?}", parse_all(&bad));
    println!("Best effort: {:?}", parse_best_effort(&bad));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn all_valid() { assert_eq!(parse_all(&["1","2","3"]), Ok(vec![1,2,3])); }
    #[test] fn short_circuits() { assert!(parse_all(&["1","bad","3"]).is_err()); }
    #[test] fn best_effort() { assert_eq!(parse_best_effort(&["1","bad","3"]), vec![1,3]); }
    #[test] fn empty_ok() { assert_eq!(parse_all(&[]), Ok(vec![])); }
}
