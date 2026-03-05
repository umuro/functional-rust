#[derive(Debug, PartialEq)]
enum MathError { DivisionByZero, NegativeInput(i64) }

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DivisionByZero => write!(f, "division by zero"),
            Self::NegativeInput(n) => write!(f, "negative input: {n}"),
        }
    }
}

fn safe_div(a: i64, b: i64) -> Result<i64, MathError> {
    if b == 0 { Err(MathError::DivisionByZero) } else { Ok(a / b) }
}

fn safe_sqrt(x: i64) -> Result<u64, MathError> {
    if x < 0 { Err(MathError::NegativeInput(x)) } else { Ok((x as f64).sqrt() as u64) }
}

fn main() {
    println!("{:?}", safe_div(10, 2));
    println!("{:?}", safe_div(10, 0));
    println!("{:?}", safe_sqrt(16));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn div_ok() -> Result<(), MathError> { assert_eq!(safe_div(10,2)?, 5); Ok(()) }
    #[test] fn div_zero() { assert_eq!(safe_div(5,0), Err(MathError::DivisionByZero)); }
    #[test] fn sqrt_ok() -> Result<(), MathError> { assert_eq!(safe_sqrt(16)?, 4); Ok(()) }
    #[test] fn sqrt_neg() { assert_eq!(safe_sqrt(-9).unwrap_err(), MathError::NegativeInput(-9)); }
    #[test] #[should_panic] fn panics_on_unwrap() { safe_div(1,0).unwrap(); }
}
