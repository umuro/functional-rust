// Example 203: Lens Laws — GetSet, SetGet, SetSet

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

struct Lens<S, A> {
    get: Box<dyn Fn(&S) -> A>,
    set: Box<dyn Fn(A, &S) -> S>,
}

impl<S, A> Lens<S, A> {
    fn new(
        get: impl Fn(&S) -> A + 'static,
        set: impl Fn(A, &S) -> S + 'static,
    ) -> Self {
        Lens { get: Box::new(get), set: Box::new(set) }
    }
}

// Approach 1: Lawful lenses
fn x_lens() -> Lens<Point, f64> {
    Lens::new(|p: &Point| p.x, |x: f64, p: &Point| Point { x, ..p.clone() })
}

fn y_lens() -> Lens<Point, f64> {
    Lens::new(|p: &Point| p.y, |y: f64, p: &Point| Point { y, ..p.clone() })
}

// Approach 2: An UNLAWFUL lens — set has a side effect
fn bad_lens() -> Lens<Point, f64> {
    Lens::new(
        |p: &Point| p.x,
        |x: f64, p: &Point| Point { x, y: p.y + 1.0 }, // mutates y!
    )
}

// Approach 3: Law verification
fn check_get_set<S: PartialEq + Clone, A: Clone>(lens: &Lens<S, A>, s: &S) -> bool {
    let a = (lens.get)(s);
    let result = (lens.set)(a, s);
    result == *s
}

fn check_set_get<S: Clone, A: PartialEq + Clone>(lens: &Lens<S, A>, a: A, s: &S) -> bool {
    let result = (lens.get)(&(lens.set)(a.clone(), s));
    result == a
}

fn check_set_set<S: PartialEq + Clone, A: Clone>(
    lens: &Lens<S, A>, a: A, b: A, s: &S,
) -> bool {
    let r1 = (lens.set)(b.clone(), &(lens.set)(a, s));
    let r2 = (lens.set)(b, s);
    r1 == r2
}

fn verify_laws<S: PartialEq + Clone, A: PartialEq + Clone>(
    name: &str, lens: &Lens<S, A>, s: &S, a: A, b: A,
) -> (bool, bool, bool) {
    let gs = check_get_set(lens, s);
    let sg = check_set_get(lens, a.clone(), s);
    let ss = check_set_set(lens, a, b, s);
    println!("{}: GetSet={} SetGet={} SetSet={}", name, gs, sg, ss);
    (gs, sg, ss)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_lens_lawful() {
        let p = Point { x: 1.0, y: 2.0 };
        let l = x_lens();
        assert!(check_get_set(&l, &p));
        assert!(check_set_get(&l, 99.0, &p));
        assert!(check_set_set(&l, 5.0, 10.0, &p));
    }

    #[test]
    fn test_bad_lens_unlawful() {
        let p = Point { x: 1.0, y: 2.0 };
        let l = bad_lens();
        assert!(!check_get_set(&l, &p));
    }

    #[test]
    fn test_set_get_law() {
        let p = Point { x: 0.0, y: 0.0 };
        let l = y_lens();
        let result = (l.get)(&(l.set)(42.0, &p));
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_set_set_law() {
        let p = Point { x: 1.0, y: 2.0 };
        let l = x_lens();
        let r1 = (l.set)(99.0, &(l.set)(50.0, &p));
        let r2 = (l.set)(99.0, &p);
        assert_eq!(r1, r2);
    }
}
