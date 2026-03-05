use std::num::ParseIntError;

#[derive(Debug)]
enum Err { Parse(ParseIntError), Range(i32), DivZero }
impl std::fmt::Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Err::Parse(e)  => write!(f, "parse: {}", e),
            Err::Range(n)  => write!(f, "{} out of range", n),
            Err::DivZero   => write!(f, "div by zero"),
        }
    }
}

fn parse(s: &str) -> Result<i32, Err> { s.parse().map_err(Err::Parse) }
fn validate(n: i32) -> Result<i32, Err> {
    if n>=1 && n<=100 { Ok(n) } else { Err(Err::Range(n)) }
}

// ? operator chains
fn process(s: &str) -> Result<i32, Err> {
    let n = parse(s)?;
    let v = validate(n)?;
    Ok(v * v)
}

fn main() {
    for s in ["42","abc","0","100","101"] {
        match process(s) {
            Ok(v)    => println!("{} -> {}", s, v),
            Err(e) => println!("{} -> Err: {}", s, e),
        }
    }
    // combinators
    let r: Result<i32,Err> = Ok(5);
    println!("map: {:?}", r.map(|x|x*2).ok());

    // collect: Vec<Result> -> Result<Vec>
    let ok_strs = vec!["1","2","3"];
    let ok_nums: Result<Vec<i32>,_> = ok_strs.iter().map(|s|parse(s)).collect();
    println!("collect ok: {:?}", ok_nums.ok());

    let bad_strs = vec!["1","x","3"];
    let bad: Result<Vec<i32>,_> = bad_strs.iter().map(|s|parse(s)).collect();
    println!("collect bad err: {}", bad.is_err());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn ok()   { assert!(process("42").is_ok()); }
    #[test] fn abc()  { assert!(process("abc").is_err()); }
    #[test] fn zero() { assert!(process("0").is_err()); }
}
