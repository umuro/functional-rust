// 004: Option and Result
// Safe handling of missing values and errors

// Approach 1: Option basics
fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn safe_head(v: &[i32]) -> Option<i32> {
    v.first().copied()
}

fn find_even(v: &[i32]) -> Option<i32> {
    v.iter().find(|&&x| x % 2 == 0).copied()
}

// Approach 2: Chaining with map and and_then
fn double_head(v: &[i32]) -> Option<i32> {
    safe_head(v).map(|x| x * 2)
}

fn safe_div_then_add(a: i32, b: i32, c: i32) -> Option<i32> {
    safe_div(a, b).map(|q| q + c)
}

fn chain_lookups(v1: &[i32], v2: &[i32]) -> Option<i32> {
    safe_head(v1).and_then(|idx| v2.get(idx as usize).copied())
}

// Approach 3: Result for richer errors
#[derive(Debug, PartialEq)]
enum MyError {
    DivByZero,
    NegativeInput,
    EmptyList,
}

fn safe_div_r(a: i32, b: i32) -> Result<i32, MyError> {
    if b == 0 { Err(MyError::DivByZero) } else { Ok(a / b) }
}

fn safe_sqrt(x: f64) -> Result<f64, MyError> {
    if x < 0.0 { Err(MyError::NegativeInput) } else { Ok(x.sqrt()) }
}

fn safe_head_r(v: &[i32]) -> Result<i32, MyError> {
    v.first().copied().ok_or(MyError::EmptyList)
}

fn compute(v: &[i32]) -> Result<i32, MyError> {
    let x = safe_head_r(v)?;
    safe_div_r(x * 10, 3)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_div() {
        assert_eq!(safe_div(10, 3), Some(3));
        assert_eq!(safe_div(10, 0), None);
    }

    #[test]
    fn test_safe_head() {
        assert_eq!(safe_head(&[1, 2, 3]), Some(1));
        assert_eq!(safe_head(&[]), None);
    }

    #[test]
    fn test_find_even() {
        assert_eq!(find_even(&[1, 3, 4, 5]), Some(4));
        assert_eq!(find_even(&[1, 3, 5]), None);
    }

    #[test]
    fn test_double_head() {
        assert_eq!(double_head(&[5, 10]), Some(10));
        assert_eq!(double_head(&[]), None);
    }

    #[test]
    fn test_chain_lookups() {
        assert_eq!(chain_lookups(&[1], &[10, 20, 30]), Some(20));
        assert_eq!(chain_lookups(&[], &[10, 20]), None);
    }

    #[test]
    fn test_result() {
        assert_eq!(safe_div_r(10, 2), Ok(5));
        assert_eq!(safe_div_r(10, 0), Err(MyError::DivByZero));
    }

    #[test]
    fn test_compute() {
        assert_eq!(compute(&[5, 10]), Ok(16));
        assert_eq!(compute(&[]), Err(MyError::EmptyList));
    }
}
