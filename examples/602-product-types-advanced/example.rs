#[derive(Debug,Clone,Copy,PartialEq)]
struct Point { x: f64, y: f64 }

#[derive(Debug,Clone,Copy)]
struct Rect { origin: Point, size: Point }

// Projections
fn proj_x(p: Point) -> f64 { p.x }
fn proj_y(p: Point) -> f64 { p.y }

// Record update: categorical pairing
fn translate(dx: f64, dy: f64, p: Point) -> Point { Point { x: p.x+dx, y: p.y+dy } }
fn scale(s: f64, p: Point) -> Point { Point { x: p.x*s, y: p.y*s } }

// Product bifunctor
fn bimap<A,B,C,D>(f: impl Fn(A)->C, g: impl Fn(B)->D, (a,b): (A,B)) -> (C,D) { (f(a), g(b)) }

// Associativity iso: (A×B)×C ≅ A×(B×C)
fn assoc_l<A,B,C>((a,b,c): (A,B,C)) -> (A,(B,C)) { (a,(b,c)) }
fn assoc_r<A,B,C>((a,(b,c)): (A,(B,C))) -> (A,B,C) { (a,b,c) }

// Swap: A×B ≅ B×A
fn swap<A,B>((a,b): (A,B)) -> (B,A) { (b,a) }

// Universal property: diagonal morphism
fn diag<A: Copy>(a: A) -> (A,A) { (a,a) }

fn main() {
    let p = Point { x:1.0, y:2.0 };
    let p2 = translate(3.0, 4.0, p);
    println!("translated: {:?}", p2);
    println!("scaled: {:?}", scale(2.0, p));

    let r = Rect { origin: p, size: Point{x:10.0,y:5.0} };
    // Record update: only change origin
    let moved = Rect { origin: translate(1.0,0.0,r.origin), ..r };
    println!("moved rect origin: {:?}", moved.origin);

    let result = bimap(|x:i32| x*2, |s: &str| s.len(), (5,"hello"));
    println!("bimap: {:?}", result);

    let triple = (1,2,3);
    println!("assoc_l: {:?}", assoc_l(triple));
    println!("assoc_r: {:?}", assoc_r(assoc_l(triple)));
    println!("swap: {:?}", swap((1,"a")));
    println!("diag: {:?}", diag(42));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn translate_test() { let p=translate(1.0,2.0,Point{x:0.0,y:0.0}); assert_eq!((p.x,p.y),(1.0,2.0)); }
    #[test] fn assoc_roundtrip() { let t=(1,2,3); assert_eq!(assoc_r(assoc_l(t)),t); }
    #[test] fn swap_swap() { let p=(1,"a"); assert_eq!(swap(swap(p)),p); }
}
