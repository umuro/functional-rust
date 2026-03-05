#[derive(Debug)]
struct Point { x: f64, y: f64 }

#[derive(Debug)]
struct Person { name: String, age: u32, email: String }

fn distance(Point { x, y }: &Point) -> f64 {
    (x * x + y * y).sqrt()
}

fn greet(Person { name, age, .. }: &Person) -> String {
    format!("Hello {}, age {}", name, age)
}

fn classify(person: &Person) -> &'static str {
    match person {
        Person { age, .. } if *age < 18 => "minor",
        Person { age, .. } if *age < 65 => "adult",
        _                               => "senior",
    }
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("dist = {:.1}", distance(&p));
    let alice = Person { name: "Alice".into(), age: 30, email: "a@b.com".into() };
    println!("{}", greet(&alice));
    println!("{}", classify(&alice));

    // Nested destructuring
    struct Rect { tl: Point, br: Point }
    let r = Rect { tl: Point { x: 0.0, y: 4.0 }, br: Point { x: 3.0, y: 0.0 } };
    let Rect { tl: Point { x: x1, y: y1 }, br: Point { x: x2, y: y2 } } = r;
    println!("area = {:.1}", (x2-x1).abs() * (y2-y1).abs());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_dist() { assert_eq!(distance(&Point{x:3.0,y:4.0}), 5.0); }
    #[test] fn test_minor() {
        let p = Person{name:"".into(),age:10,email:"".into()};
        assert_eq!(classify(&p), "minor");
    }
}
