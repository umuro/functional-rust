// Paramorphism: algebra receives (current_elem, original_rest, folded_rest)
fn para_list<A: Clone, R>(
    xs: &[A],
    nil: R,
    cons: impl Fn(&A, &[A], R) -> R + Copy,
) -> R {
    match xs {
        []            => nil,
        [head, rest @ ..] => {
            let folded_rest = para_list(rest, nil, cons);
            cons(head, rest, folded_rest)
        }
    }
}

// Insert into sorted list — needs original tail for correctness
fn insert_sorted(x: i32, xs: &[i32]) -> Vec<i32> {
    para_list(
        xs,
        vec![x],
        |&head, tail, folded| {
            if x <= head {
                let mut v = vec![x, head];
                v.extend_from_slice(tail);
                v
            } else {
                let mut v = vec![head];
                v.extend(folded);
                v
            }
        },
    )
}

fn insertion_sort(mut xs: Vec<i32>) -> Vec<i32> {
    let items = xs.clone();
    xs.clear();
    for x in items { xs = insert_sorted(x, &xs); }
    xs
}

// Tails: paramorphism exposing original sublist
fn tails<A: Clone>(xs: &[A]) -> Vec<Vec<A>> {
    para_list(
        xs,
        vec![vec![]],
        |head, _rest, mut folded| {
            let first = folded[0].clone();
            let mut new_first = vec![head.clone()];
            new_first.extend(first);
            folded.insert(0, new_first);
            folded
        },
    )
}

fn main() {
    let sorted = insert_sorted(3, &[1,2,4,5]);
    println!("insert 3 into [1,2,4,5]: {:?}", sorted);
    println!("insertion_sort: {:?}", insertion_sort(vec![3,1,4,1,5,9,2,6]));
    let t = tails(&[1i32,2,3]);
    println!("tails: {:?}", t);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn insert_test() { assert_eq!(insert_sorted(3,&[1,2,4,5]), vec![1,2,3,4,5]); }
    #[test] fn sort_test()   { assert_eq!(insertion_sort(vec![3,1,2]),  vec![1,2,3]); }
    #[test] fn tails_test()  { assert_eq!(tails(&[1i32,2]).len(), 3); /* [1,2],[2],[] */ }
}
