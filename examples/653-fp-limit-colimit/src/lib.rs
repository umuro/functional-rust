//! # Limits and Colimits
//!
//! Limits and colimits are universal constructions in category theory.
//! - Limit: Product, Equalizer, Pullback
//! - Colimit: Coproduct (Sum), Coequalizer, Pushout

use std::marker::PhantomData;

// === LIMITS ===

/// Product (binary limit) - AND of types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Product<A, B>(pub A, pub B);

impl<A, B> Product<A, B> {
    pub fn new(a: A, b: B) -> Self { Product(a, b) }
    pub fn fst(&self) -> &A { &self.0 }
    pub fn snd(&self) -> &B { &self.1 }
}

/// Terminal object (limit of empty diagram)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Terminal;

/// Equalizer - limit that finds where two functions agree
pub fn equalizer<A: Clone + PartialEq, B, F, G>(items: impl Iterator<Item = A>, f: F, g: G) -> Vec<A>
where
    F: Fn(&A) -> B,
    G: Fn(&A) -> B,
    B: PartialEq,
{
    items.filter(|a| f(a) == g(a)).collect()
}

/// Pullback - limit of a cospan A -> C <- B
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pullback<A, B> {
    pub left: A,
    pub right: B,
}

pub fn pullback<A: Clone, B: Clone, C: PartialEq, F, G>(
    as_: impl Iterator<Item = A>,
    bs: &[B],
    f: F,
    g: G,
) -> Vec<Pullback<A, B>>
where
    F: Fn(&A) -> C,
    G: Fn(&B) -> C,
{
    as_.flat_map(|a| {
        let fa = f(&a);
        bs.iter()
            .filter(|b| g(b) == fa)
            .map(|b| Pullback { left: a.clone(), right: b.clone() })
            .collect::<Vec<_>>()
    }).collect()
}

// === COLIMITS ===

/// Coproduct (binary colimit) - OR of types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Coproduct<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Coproduct<A, B> {
    pub fn left(a: A) -> Self { Coproduct::Left(a) }
    pub fn right(b: B) -> Self { Coproduct::Right(b) }
    
    pub fn fold<C, F, G>(self, f: F, g: G) -> C
    where
        F: FnOnce(A) -> C,
        G: FnOnce(B) -> C,
    {
        match self {
            Coproduct::Left(a) => f(a),
            Coproduct::Right(b) => g(b),
        }
    }
}

/// Initial object (colimit of empty diagram)
#[derive(Debug, Clone, Copy)]
pub enum Initial {}

impl Initial {
    pub fn absurd<T>(self) -> T {
        match self {}
    }
}

/// Coequalizer - colimit that identifies elements
pub fn coequalizer<A: Clone + Eq + std::hash::Hash>(
    items: Vec<A>,
    equivalences: Vec<(A, A)>,
) -> Vec<A> {
    use std::collections::HashSet;
    let mut result: Vec<A> = Vec::new();
    let mut seen: HashSet<A> = HashSet::new();
    
    for item in items {
        let canonical = equivalences.iter()
            .find(|(a, _)| a == &item)
            .map(|(_, b)| b.clone())
            .unwrap_or(item.clone());
        
        if !seen.contains(&canonical) {
            seen.insert(canonical.clone());
            result.push(canonical);
        }
    }
    result
}

/// Pushout - colimit of a span A <- C -> B
pub fn pushout<A, B, C, F, G>(c: C, f: F, g: G) -> Coproduct<A, B>
where
    F: FnOnce(C) -> A,
    C: Clone,
    G: FnOnce(C) -> B,
{
    // Simplified: returns left by default
    // Real pushout would identify f(c) and g(c)
    Coproduct::Left(f(c))
}

// Practical applications

/// JOIN as pullback (database-style)
pub fn join_on<A: Clone, B: Clone, K: PartialEq, FA, FB>(
    table_a: &[A],
    table_b: &[B],
    key_a: FA,
    key_b: FB,
) -> Vec<(A, B)>
where
    FA: Fn(&A) -> K,
    FB: Fn(&B) -> K,
{
    table_a.iter()
        .flat_map(|a| {
            let ka = key_a(a);
            table_b.iter()
                .filter(|b| key_b(b) == ka)
                .map(|b| (a.clone(), b.clone()))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product() {
        let p = Product::new(1, "hello");
        assert_eq!(*p.fst(), 1);
        assert_eq!(*p.snd(), "hello");
    }

    #[test]
    fn test_coproduct_left() {
        let c: Coproduct<i32, &str> = Coproduct::left(42);
        let result = c.fold(|x| x * 2, |s| s.len() as i32);
        assert_eq!(result, 84);
    }

    #[test]
    fn test_coproduct_right() {
        let c: Coproduct<i32, &str> = Coproduct::right("hello");
        let result = c.fold(|x| x, |s| s.len() as i32);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_equalizer() {
        let f = |x: &i32| x % 2;
        let g = |x: &i32| x % 4;
        let result = equalizer(vec![0, 1, 2, 3, 4, 5, 6, 7, 8].into_iter(), f, g);
        // Elements where x%2 == x%4: 0, 1, 4, 5, 8
        assert_eq!(result, vec![0, 1, 4, 5, 8]);
    }

    #[test]
    fn test_pullback() {
        let as_ = vec![1, 2, 3];
        let bs = vec!["a", "bb", "ccc"];
        let f = |x: &i32| *x as usize;
        let g = |s: &&str| s.len();
        
        let pb: Vec<Pullback<i32, &str>> = pullback(as_.into_iter(), &bs, f, g);
        assert_eq!(pb.len(), 3);
    }

    #[test]
    fn test_join_on() {
        #[derive(Clone, Debug)]
        struct User { id: i32, name: String }
        #[derive(Clone, Debug)]
        struct Order { user_id: i32, product: String }
        
        let users = vec![
            User { id: 1, name: "Alice".into() },
            User { id: 2, name: "Bob".into() },
        ];
        let orders = vec![
            Order { user_id: 1, product: "Widget".into() },
            Order { user_id: 1, product: "Gadget".into() },
        ];
        
        let joined = join_on(&users, &orders, |u| u.id, |o| o.user_id);
        assert_eq!(joined.len(), 2);
        assert_eq!(joined[0].0.name, "Alice");
    }

    #[test]
    fn test_coequalizer() {
        let items = vec![1, 2, 3, 4, 5];
        let equiv = vec![(2, 1), (4, 3)]; // 2~1, 4~3
        let result = coequalizer(items, equiv);
        assert!(result.contains(&1));
        assert!(result.contains(&3));
        assert!(result.contains(&5));
    }

    #[test]
    fn test_terminal() {
        let _t = Terminal;
        assert_eq!(Terminal, Terminal); // unique
    }
}
