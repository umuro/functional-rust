// Prism: fallible optic for sum types
struct Prism<S, A> {
    preview_fn: Box<dyn Fn(&S) -> Option<A>>,
    review_fn:  Box<dyn Fn(A) -> S>,
}

impl<S: Clone + PartialEq, A: Clone + PartialEq> Prism<S, A> {
    fn new(preview_fn: impl Fn(&S)->Option<A>+'static, review_fn: impl Fn(A)->S+'static) -> Self {
        Prism { preview_fn: Box::new(preview_fn), review_fn: Box::new(review_fn) }
    }
    fn preview(&self, s: &S) -> Option<A> { (self.preview_fn)(s) }
    fn review(&self, a: A) -> S { (self.review_fn)(a) }
    fn over(&self, f: impl Fn(A)->A, s: S) -> S {
        match self.preview(&s) {
            Some(a) => self.review(f(a)),
            None    => s,
        }
    }

    // Laws
    fn law_preview_review(&self, a: A) -> bool where A: std::fmt::Debug {
        self.preview(&self.review(a.clone())) == Some(a)
    }
    fn law_review_preview(&self, s: &S) -> bool {
        match self.preview(s) {
            None    => true,
            Some(a) => &self.review(a) == s,
        }
    }
}

// JSON-like type for demonstration
#[derive(Debug,Clone,PartialEq)]
enum Json { Null, Bool(bool), Num(f64), Str(String), Arr(Vec<Json>) }

fn main() {
    let bool_prism: Prism<Json,bool> = Prism::new(
        |j| match j { Json::Bool(b) => Some(*b), _ => None },
        |b| Json::Bool(b),
    );
    let num_prism: Prism<Json,f64> = Prism::new(
        |j| match j { Json::Num(n) => Some(*n), _ => None },
        |n| Json::Num(n),
    );

    // Laws
    println!("bool law preview∘review(true):  {}", bool_prism.law_preview_review(true));
    println!("bool law review∘preview(Num):   {}", bool_prism.law_review_preview(&Json::Num(1.0)));
    println!("num  law preview∘review(42.0):  {}", num_prism.law_preview_review(42.0));

    // over: modify if matches
    let j1 = Json::Bool(false);
    let j2 = bool_prism.over(|b| !b, j1);
    println!("!false = {:?}", j2);

    let j3 = Json::Null;
    let j4 = bool_prism.over(|b| !b, j3.clone());
    println!("over Null = {:?} (unchanged)", j4);

    // Compose prisms manually (via preview/review)
    let jsons = vec![Json::Bool(true), Json::Num(3.14), Json::Str("hi".into()), Json::Null];
    let bools: Vec<bool> = jsons.iter().filter_map(|j| bool_prism.preview(j)).collect();
    println!("bools from json array: {:?}", bools);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn prism_law_bool() {
        let p: Prism<Json,bool> = Prism::new(
            |j| match j { Json::Bool(b)=>Some(*b), _=>None }, Json::Bool);
        assert!(p.law_preview_review(true));
        assert!(p.law_review_preview(&Json::Num(1.0)));
    }
}
