fn demo_deref_pattern() {
    let values = vec![1,2,3,4,5];
    // &x strips one layer of reference — x is i32
    let sum: i32 = values.iter().map(|&x| x).sum();
    println!("sum = {}", sum);
}

fn first_two_borrowed(v: &[String]) -> Option<(&str, &str)> {
    match v {
        [ref a, ref b, ..] => Some((a, b)),
        _                  => None,
    }
}

fn increment_first(v: &mut [i32]) {
    if let [ref mut first, ..] = v { *first += 1; }
}

fn demo_ref_in_let() {
    let s = String::from("hello");
    let ref r = s;   // r: &String, s still owned
    println!("r={} s={}", r, s);
}

fn main() {
    demo_deref_pattern();
    demo_ref_in_let();

    let words: Vec<String> = ["a","b","c"].iter().map(|s|s.to_string()).collect();
    if let Some((a,b)) = first_two_borrowed(&words) {
        println!("first two: {}, {}", a, b);
    }

    let mut nums = vec![10,20,30];
    increment_first(&mut nums);
    println!("{:?}", nums);

    // Match ergonomics: auto-ref on &Option
    let opt = Some(String::from("hello"));
    if let Some(s) = &opt {   // s: &String, opt still usable
        println!("borrowed: {}", s);
    }
    println!("opt: {:?}", opt);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_inc() { let mut v=vec![5,6]; increment_first(&mut v); assert_eq!(v[0],6); }
    #[test] fn test_deref() {
        let v = vec![1i32,2,3];
        let s: i32 = v.iter().map(|&x| x).sum();
        assert_eq!(s, 6);
    }
}
