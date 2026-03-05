#[derive(Debug,Clone,PartialEq)]
enum Either<A,B> { Left(A), Right(B) }

impl<A,B> Either<A,B> {
    fn inl(a: A) -> Self { Either::Left(a) }
    fn inr(b: B) -> Self { Either::Right(b) }

    // Universal property: the unique morphism
    fn either<C>(self, f: impl FnOnce(A)->C, g: impl FnOnce(B)->C) -> C {
        match self { Either::Left(a)=>f(a), Either::Right(b)=>g(b) }
    }

    fn bimap<C,D>(self, f: impl FnOnce(A)->C, g: impl FnOnce(B)->D) -> Either<C,D> {
        match self { Either::Left(a)=>Either::Left(f(a)), Either::Right(b)=>Either::Right(g(b)) }
    }

    fn is_left(&self)  -> bool { matches!(self, Either::Left(_))  }
    fn is_right(&self) -> bool { matches!(self, Either::Right(_)) }

    fn left(self)  -> Option<A> { match self { Either::Left(a)=>Some(a),  _=>None } }
    fn right(self) -> Option<B> { match self { Either::Right(b)=>Some(b), _=>None } }
}

// Partition a Vec<Either<A,B>> into (Vec<A>, Vec<B>)
fn partition_either<A,B>(items: Vec<Either<A,B>>) -> (Vec<A>, Vec<B>) {
    let (mut lefts, mut rights) = (vec![], vec![]);
    for item in items {
        match item { Either::Left(a)=>lefts.push(a), Either::Right(b)=>rights.push(b) }
    }
    (lefts, rights)
}

fn main() {
    let xs: Vec<Either<i32,String>> = vec![
        Either::inl(1), Either::inr("hello".into()),
        Either::inl(42), Either::inr("world".into()),
    ];
    for e in &xs {
        let desc = e.clone().either(|n| format!("int:{}", n), |s| format!("str:{}", s));
        println!("{}", desc);
    }
    let (ints, strs) = partition_either(xs);
    println!("ints: {:?}  strs: {:?}", ints, strs);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn inl_left() { assert!(Either::<i32,&str>::inl(5).is_left()); }
    #[test] fn either_map() {
        let e: Either<i32,&str> = Either::inl(5);
        assert_eq!(e.either(|n|n*2,|_|0), 10);
    }
    #[test] fn partition() {
        let v = vec![Either::Left(1),Either::Right("a"),Either::Left(2)];
        let (l,r) = partition_either(v);
        assert_eq!(l, vec![1,2]); assert_eq!(r, vec!["a"]);
    }
}
