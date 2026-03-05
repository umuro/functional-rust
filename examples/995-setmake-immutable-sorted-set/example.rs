use std::collections::BTreeSet;

pub fn set_of_slice<T: Ord + Clone>(items: &[T]) -> BTreeSet<T> {
    items.iter().cloned().collect()
}

pub fn union<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.union(b).cloned().collect()
}

pub fn inter<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.intersection(b).cloned().collect()
}

pub fn diff<T: Ord + Clone>(a: &BTreeSet<T>, b: &BTreeSet<T>) -> BTreeSet<T> {
    a.difference(b).cloned().collect()
}

pub fn elements<T: Ord + Clone>(s: &BTreeSet<T>) -> Vec<T> {
    s.iter().cloned().collect()
}

pub fn set_from_iter<T: Ord, I: IntoIterator<Item = T>>(iter: I) -> BTreeSet<T> {
    iter.into_iter().collect()
}

pub fn mem<T: Ord>(x: &T, s: &BTreeSet<T>) -> bool {
    s.contains(x)
}

pub fn add<T: Ord + Clone>(x: T, s: &BTreeSet<T>) -> BTreeSet<T> {
    let mut next = s.clone();
    next.insert(x);
    next
}

pub fn remove<T: Ord + Clone>(x: &T, s: &BTreeSet<T>) -> BTreeSet<T> {
    let mut next = s.clone();
    next.remove(x);
    next
}

pub fn filter<T: Ord + Clone, F: Fn(&T) -> bool>(pred: F, s: &BTreeSet<T>) -> BTreeSet<T> {
    s.iter().filter(|x| pred(x)).cloned().collect()
}

pub fn map_set<T: Ord + Clone, U: Ord, F: Fn(&T) -> U>(f: F, s: &BTreeSet<T>) -> BTreeSet<U> {
    s.iter().map(f).collect()
}

pub fn fold_set<T: Ord, A, F: Fn(A, &T) -> A>(f: F, s: &BTreeSet<T>, init: A) -> A {
    s.iter().fold(init, f)
}

fn fmt_set(s: &BTreeSet<i32>) -> String {
    let parts: Vec<String> = s.iter().map(|x| x.to_string()).collect();
    format!("{{{}}}", parts.join(", "))
}

fn main() {
    let s1 = set_of_slice(&[1, 3, 5, 7, 9]);
    let s2 = set_of_slice(&[2, 3, 5, 7, 11]);

    println!("s1 = {}", fmt_set(&s1));
    println!("s2 = {}", fmt_set(&s2));
    println!("union      = {}", fmt_set(&union(&s1, &s2)));
    println!("inter      = {}", fmt_set(&inter(&s1, &s2)));
    println!("diff s1 s2 = {}", fmt_set(&diff(&s1, &s2)));

    let s3 = add(4, &s1);
    println!("add 4 s1   = {}", fmt_set(&s3));
    println!("s1 after   = {} (unchanged)", fmt_set(&s1));

    let s4 = remove(&3, &s1);
    println!("remove 3 s1= {}", fmt_set(&s4));

    let evens = filter(|x| x % 2 == 0, &set_of_slice(&[1, 2, 3, 4, 5, 6]));
    println!("evens      = {}", fmt_set(&evens));

    let sum = fold_set(|acc, x| acc + x, &set_of_slice(&[1, 2, 3, 4, 5]), 0);
    println!("fold sum   = {}", sum);

    let from_range: BTreeSet<i32> = set_from_iter(1..=5);
    println!("from range = {}", fmt_set(&from_range));
}

/* Output:
   s1 = {1, 3, 5, 7, 9}
   s2 = {2, 3, 5, 7, 11}
   union      = {1, 2, 3, 5, 7, 9, 11}
   inter      = {3, 5, 7}
   diff s1 s2 = {1, 9}
   add 4 s1   = {1, 3, 4, 5, 7, 9}
   s1 after   = {1, 3, 5, 7, 9} (unchanged)
   remove 3 s1= {1, 5, 7, 9}
   evens      = {2, 4, 6}
   fold sum   = 15
   from range = {1, 2, 3, 4, 5}
*/
