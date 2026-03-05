#[derive(Debug)]
enum Shape { Circle(f64), Rect(f64, f64) }

fn describe(s: &Shape) -> &'static str {
    match s {
        Shape::Circle(r) if *r <= 0.0   => "invalid",
        Shape::Circle(r) if *r < 1.0    => "tiny circle",
        Shape::Circle(_)                 => "circle",
        Shape::Rect(w, h) if w == h      => "square",
        Shape::Rect(w, h) if w > h       => "wide",
        Shape::Rect(_, _)                => "tall",
    }
}

fn grade(score: u32) -> char {
    match score {
        n if n >= 90 => 'A',
        n if n >= 80 => 'B',
        n if n >= 70 => 'C',
        n if n >= 60 => 'D',
        _            => 'F',
    }
}

fn main() {
    for s in [Shape::Circle(-1.0), Shape::Circle(0.5), Shape::Circle(2.0),
              Shape::Rect(3.0,3.0), Shape::Rect(5.0,2.0), Shape::Rect(2.0,5.0)] {
        println!("{:?} -> {}", s, describe(&s));
    }
    for n in [95u32, 82, 74, 61, 45] {
        println!("{} -> {}", n, grade(n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn square() { assert_eq!(describe(&Shape::Rect(4.0,4.0)), "square"); }
    #[test] fn grade_a() { assert_eq!(grade(95), 'A'); }
    #[test] fn grade_f() { assert_eq!(grade(55), 'F'); }
}
