#[derive(Debug)]
enum Status { Active, Inactive, Pending, Banned }

fn main() {
    let users = [Status::Active, Status::Inactive, Status::Pending, Status::Banned, Status::Active];

    let active = users.iter().filter(|u| matches!(u, Status::Active)).count();
    let usable = users.iter().filter(|u| matches!(u, Status::Active | Status::Pending)).count();
    println!("active={} usable={}", active, usable);

    // matches! with guard
    let nums = vec![1,2,3,4,5,6,7,8,9,10];
    let even_small: Vec<_> = nums.iter().copied()
        .filter(|&n| matches!(n, x if x%2==0 && x<=6))
        .collect();
    println!("even ≤ 6: {:?}", even_small);

    // in assert!
    let r: Result<i32,&str> = Ok(42);
    assert!(matches!(r, Ok(n) if n > 0));

    // with enum data
    #[derive(Debug)]
    enum Shape { Circle(f64), Square(f64), Other }
    let shapes = vec![Shape::Circle(1.0), Shape::Square(2.0), Shape::Other, Shape::Circle(0.5)];
    let circles = shapes.iter().filter(|s| matches!(s, Shape::Circle(_))).count();
    let large   = shapes.iter().filter(|s| matches!(s, Shape::Circle(r)|Shape::Square(r) if *r>1.0)).count();
    println!("circles={} large={}", circles, large);

    let words = ["fn","let","hello","match","world"];
    let kw: Vec<_> = words.iter().filter(|&&w| matches!(w,"fn"|"let"|"match"|"if")).collect();
    println!("keywords: {:?}", kw);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn is_active() { assert!( matches!(Status::Active,  Status::Active)); }
    #[test] fn not_active(){ assert!(!matches!(Status::Banned, Status::Active)); }
    #[test] fn guard()     { assert!( matches!(4, x if x%2==0)); }
}
