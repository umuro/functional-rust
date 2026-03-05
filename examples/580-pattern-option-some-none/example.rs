fn safe_div(a: i32, b: i32) -> Option<i32> { if b==0 {None} else {Some(a/b)} }
fn safe_sqrt(x: f64) -> Option<f64>         { if x<0.0{None} else {Some(x.sqrt())} }

fn compute(a: i32, b: i32) -> Option<f64> {
    safe_div(a, b).map(|q| q as f64).and_then(safe_sqrt).map(|r| r*2.0)
}

fn main() {
    for (a,b) in [(10,2),(10,0),(-4,2)] {
        match compute(a,b) {
            Some(v) => println!("{}/{} -> {:.2}", a, b, v),
            None    => println!("{}/{} -> None", a, b),
        }
    }

    let names: Vec<Option<&str>> = vec![Some("alice"),None,Some("bob")];
    let upper: Vec<_> = names.iter().filter_map(|o| o.map(str::to_uppercase)).collect();
    println!("{:?}", upper);

    // Combinators
    let x: Option<i32> = None;
    println!("unwrap_or: {}", x.unwrap_or(0));
    println!("unwrap_or_else: {}", x.unwrap_or_else(|| 42));
    println!("unwrap_or_default: {}", x.unwrap_or_default());

    let s: Option<&str> = Some("42");
    let parsed = s.and_then(|s| s.parse::<i32>().ok());
    println!("parsed: {:?}", parsed);
    println!("filtered: {:?}", parsed.filter(|&n| n > 0));

    // flatten, zip
    let nested: Option<Option<i32>> = Some(Some(42));
    println!("flatten: {:?}", nested.flatten());
    println!("zip: {:?}", Some(1).zip(Some("hello")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn div_ok()  { assert_eq!(safe_div(10,2), Some(5)); }
    #[test] fn div_zero(){ assert_eq!(safe_div(10,0), None); }
    #[test] fn combinator(){
        let opt: Option<i32> = Some(4);
        assert_eq!(opt.map(|x|x*2), Some(8));
        assert_eq!(opt.filter(|&x|x>10), None);
    }
}
