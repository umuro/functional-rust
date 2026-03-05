// Isomorphism: lossless bidirectional conversion
struct Iso<A, B> {
    to_fn:   Box<dyn Fn(A) -> B>,
    from_fn: Box<dyn Fn(B) -> A>,
}

impl<A: Clone + PartialEq + 'static, B: Clone + PartialEq + 'static> Iso<A, B> {
    fn new(to_fn: impl Fn(A)->B+'static, from_fn: impl Fn(B)->A+'static) -> Self {
        Iso { to_fn: Box::new(to_fn), from_fn: Box::new(from_fn) }
    }
    fn to_(&self, a: A) -> B { (self.to_fn)(a) }
    fn from_(&self, b: B) -> A { (self.from_fn)(b) }

    // Law: from(to(a)) == a
    fn law_roundtrip(&self, a: A) -> bool { self.from_(self.to_(a.clone())) == a }
    // Law: to(from(b)) == b
    fn law_roundtrip_inv(&self, b: B) -> bool { self.to_(self.from_(b.clone())) == b }

    fn inverse(self) -> Iso<B,A> {
        Iso { to_fn: self.from_fn, from_fn: self.to_fn }
    }
}

fn main() {
    // Celsius <-> Fahrenheit
    let c_f: Iso<f64,f64> = Iso::new(
        |c| c * 9.0/5.0 + 32.0,
        |f| (f - 32.0) * 5.0/9.0,
    );
    println!("100°C = {:.1}°F", c_f.to_(100.0));
    let f_c = Iso::new(|f:f64| (f-32.0)*5.0/9.0, |c:f64| c*9.0/5.0+32.0);
    println!("212°F = {:.1}°C", f_c.to_(212.0));
    println!("law C->F->C at 100: {}", c_f.law_roundtrip(100.0));
    println!("law F->C->F at 212: {}", c_f.law_roundtrip_inv(212.0));

    // String <-> Vec<char>
    let str_chars: Iso<String, Vec<char>> = Iso::new(
        |s: String| s.chars().collect(),
        |cs: Vec<char>| cs.into_iter().collect(),
    );
    let s = "hello".to_string();
    let chars = str_chars.to_(s.clone());
    println!("chars: {:?}", chars);
    println!("back: {}", str_chars.from_(chars.clone()));
    println!("law str->chars->str: {}", str_chars.law_roundtrip(s));

    // Tuple swap Iso: (A,B) <-> (B,A)
    let swap: Iso<(i32,&str), (&str,i32)> = Iso::new(|(a,b)|(b,a), |(b,a)|(a,b));
    println!("swap (1,'hi') = {:?}", swap.to_((1,"hi")));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn celsius_roundtrip() {
        let c_f: Iso<f64,f64> = Iso::new(|c|c*9.0/5.0+32.0, |f|(f-32.0)*5.0/9.0);
        assert!(c_f.law_roundtrip(100.0));
        assert!((c_f.to_(0.0) - 32.0).abs() < 1e-10);
    }
}
