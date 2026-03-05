// Example 220: Paramorphism — Cata with Access to Original Subtree

#[derive(Debug, Clone)]
enum ListF<A> { NilF, ConsF(i64, A) }

impl<A> ListF<A> {
    fn map<B>(self, f: impl Fn(A) -> B) -> ListF<B> {
        match self { ListF::NilF => ListF::NilF, ListF::ConsF(x, a) => ListF::ConsF(x, f(a)) }
    }
    fn map_ref<B>(&self, f: impl Fn(&A) -> B) -> ListF<B> {
        match self { ListF::NilF => ListF::NilF, ListF::ConsF(x, a) => ListF::ConsF(*x, f(a)) }
    }
}

#[derive(Debug, Clone)]
struct FixList(Box<ListF<FixList>>);

fn nil() -> FixList { FixList(Box::new(ListF::NilF)) }
fn cons(x: i64, xs: FixList) -> FixList { FixList(Box::new(ListF::ConsF(x, xs))) }

fn cata<A>(alg: &dyn Fn(ListF<A>) -> A, FixList(f): &FixList) -> A {
    alg(f.map_ref(|child| cata(alg, child)))
}

fn to_vec(fl: &FixList) -> Vec<i64> {
    cata(&|l| match l {
        ListF::NilF => vec![],
        ListF::ConsF(x, mut acc) => { acc.insert(0, x); acc }
    }, fl)
}

// para: algebra gets (result, original_subtree) for each child
fn para<A: Clone>(alg: &dyn Fn(ListF<(A, FixList)>) -> A, fl: &FixList) -> A {
    let paired = fl.0.map_ref(|child| (para(alg, child), child.clone()));
    alg(paired)
}

// Approach 1: tails — needs original subtree to convert
fn tails(fl: &FixList) -> Vec<Vec<i64>> {
    let mut result = vec![to_vec(fl)];
    result.extend(para(&|l: ListF<(Vec<Vec<i64>>, FixList)>| match l {
        ListF::NilF => vec![vec![]],
        ListF::ConsF(_, (rest_tails, original_tail)) => {
            let mut v = vec![to_vec(&original_tail)];
            v.extend(rest_tails);
            v
        }
    }, fl));
    result
}

// Approach 2: Sliding window
fn sliding_window(n: usize, fl: &FixList) -> Vec<Vec<i64>> {
    para(&|l: ListF<(Vec<Vec<i64>>, FixList)>| match l {
        ListF::NilF => vec![],
        ListF::ConsF(x, (rest_windows, original_tail)) => {
            let mut remainder = vec![x];
            remainder.extend(to_vec(&original_tail));
            let mut result = if remainder.len() >= n {
                vec![remainder[..n].to_vec()]
            } else {
                vec![]
            };
            result.extend(rest_windows);
            result
        }
    }, fl)
}

// Approach 3: drop_while
fn drop_while(pred: impl Fn(i64) -> bool, fl: &FixList) -> Vec<i64> {
    para(&|l: ListF<(Vec<i64>, FixList)>| match l {
        ListF::NilF => vec![],
        ListF::ConsF(x, (rest, original_tail)) => {
            if pred(x) { rest }
            else { let mut v = vec![x]; v.extend(to_vec(&original_tail)); v }
        }
    }, fl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tails() {
        let xs = cons(1, cons(2, nil()));
        assert_eq!(tails(&xs), vec![vec![1,2], vec![2], vec![]]);
    }

    #[test]
    fn test_sliding() {
        let xs = cons(1, cons(2, cons(3, cons(4, nil()))));
        assert_eq!(sliding_window(3, &xs), vec![vec![1,2,3], vec![2,3,4]]);
    }

    #[test]
    fn test_drop_while_all() {
        let xs = cons(1, cons(2, nil()));
        assert_eq!(drop_while(|_| true, &xs), vec![]);
    }
}
