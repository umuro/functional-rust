fn sum(s: &[i32]) -> i32 {
    match s {
        []            => 0,
        [x, rest @ ..] => x + sum(rest),
    }
}

fn product(s: &[i32]) -> i32 {
    match s {
        []            => 1,
        [x, rest @ ..] => x * product(rest),
    }
}

fn first_two(s: &[i32]) -> Option<(i32,i32)> {
    match s { [a,b,..] => Some((*a,*b)), _ => None }
}

fn last(s: &[i32]) -> Option<i32> {
    match s { [] => None, [x] => Some(*x), [_,rest@..] => last(rest) }
}

fn describe(s: &[i32]) -> String {
    match s {
        []              => "empty".into(),
        [x]             => format!("one: {}", x),
        [a,b]           => format!("pair: ({},{})", a, b),
        [first,..,last] => format!("many: {}..{}", first, last),
    }
}

fn main() {
    let xs = [1,2,3,4,5];
    println!("sum={} product={}", sum(&xs), product(&xs));
    println!("first_two: {:?}", first_two(&xs));
    println!("last: {:?}", last(&xs));
    for s in [&[][..],&[1][..],&[1,2][..],&[1,2,3,4][..]] {
        println!("describe {:?}: {}", s, describe(s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_sum()     { assert_eq!(sum(&[1,2,3,4,5]), 15); }
    #[test] fn test_last()    { assert_eq!(last(&[1,2,3]), Some(3)); }
    #[test] fn test_describe(){ assert_eq!(describe(&[]), "empty"); }
}
