/// Interval Tree for Stabbing Queries.
///
/// Each node stores its median and intervals spanning it in two sorted orders.
/// Query answers "which intervals contain x?" in O(log n + k).

#[derive(Clone, Debug)]
struct Interval {
    lo: f64,
    hi: f64,
    id: usize,
}

struct IntervalNode {
    median: f64,
    by_lo: Vec<Interval>,  // sorted ascending by lo
    by_hi: Vec<Interval>,  // sorted descending by hi
    left: Option<Box<IntervalNode>>,
    right: Option<Box<IntervalNode>>,
}

/// Build an interval tree from a list of intervals.
fn build(ivs: Vec<Interval>) -> Option<Box<IntervalNode>> {
    if ivs.is_empty() { return None; }

    // Compute median of all endpoints
    let mut endpoints: Vec<f64> = ivs.iter().flat_map(|iv| [iv.lo, iv.hi]).collect();
    endpoints.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = endpoints[endpoints.len() / 2];

    let mut spanning = Vec::new();
    let mut left_ivs = Vec::new();
    let mut right_ivs = Vec::new();

    for iv in ivs {
        if iv.hi < median {
            left_ivs.push(iv);
        } else if iv.lo > median {
            right_ivs.push(iv);
        } else {
            spanning.push(iv);
        }
    }

    let mut by_lo = spanning.clone();
    let mut by_hi = spanning;
    by_lo.sort_by(|a, b| a.lo.partial_cmp(&b.lo).unwrap());
    by_hi.sort_by(|a, b| b.hi.partial_cmp(&a.hi).unwrap());

    Some(Box::new(IntervalNode {
        median,
        by_lo,
        by_hi,
        left: build(left_ivs),
        right: build(right_ivs),
    }))
}

/// Stabbing query: return all intervals containing x.
fn query(x: f64, node: &Option<Box<IntervalNode>>, results: &mut Vec<usize>) {
    let Some(n) = node else { return; };

    if x <= n.median {
        // Scan by_lo until lo > x (early termination)
        for iv in &n.by_lo {
            if iv.lo > x { break; }
            results.push(iv.id);
        }
        query(x, &n.left, results);
    } else {
        // Scan by_hi (desc) until hi < x
        for iv in &n.by_hi {
            if iv.hi < x { break; }
            results.push(iv.id);
        }
        query(x, &n.right, results);
    }
}

struct IntervalTree {
    root: Option<Box<IntervalNode>>,
}

impl IntervalTree {
    fn new(intervals: Vec<Interval>) -> Self {
        Self { root: build(intervals) }
    }

    fn stab(&self, x: f64) -> Vec<usize> {
        let mut results = Vec::new();
        query(x, &self.root, &mut results);
        results.sort();
        results
    }
}

fn main() {
    let ivs = vec![
        Interval { lo: 1.0, hi: 5.0,  id: 1 },
        Interval { lo: 2.0, hi: 8.0,  id: 2 },
        Interval { lo: 6.0, hi: 10.0, id: 3 },
        Interval { lo: 3.0, hi: 7.0,  id: 4 },
        Interval { lo: 9.0, hi: 12.0, id: 5 },
    ];
    let tree = IntervalTree::new(ivs);

    for x in [0.0f64, 3.0, 6.5, 9.5, 15.0] {
        println!("stab({x}): {:?}", tree.stab(x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_tree() -> IntervalTree {
        IntervalTree::new(vec![
            Interval { lo: 1.0, hi: 5.0,  id: 1 },
            Interval { lo: 2.0, hi: 8.0,  id: 2 },
            Interval { lo: 6.0, hi: 10.0, id: 3 },
            Interval { lo: 3.0, hi: 7.0,  id: 4 },
            Interval { lo: 9.0, hi: 12.0, id: 5 },
        ])
    }

    // Brute-force stab for verification
    fn brute_stab(x: f64, ivs: &[(f64, f64, usize)]) -> Vec<usize> {
        let mut r: Vec<usize> = ivs.iter()
            .filter(|&&(lo, hi, _)| lo <= x && x <= hi)
            .map(|&(_, _, id)| id)
            .collect();
        r.sort();
        r
    }

    #[test]
    fn test_stab_miss() {
        assert_eq!(make_tree().stab(0.0), vec![]);
        assert_eq!(make_tree().stab(15.0), vec![]);
    }

    #[test]
    fn test_stab_hit() {
        let t = make_tree();
        assert_eq!(t.stab(3.0), vec![1, 2, 4]);
        assert_eq!(t.stab(6.5), vec![2, 3, 4]);
        assert_eq!(t.stab(9.5), vec![3, 5]);
    }

    #[test]
    fn test_boundary() {
        let t = make_tree();
        assert!(t.stab(1.0).contains(&1));
        assert!(t.stab(5.0).contains(&1));
        assert!(!t.stab(5.1).contains(&1));
    }

    #[test]
    fn test_matches_brute() {
        let raw = vec![(1.0f64, 5.0, 1), (2.0, 8.0, 2), (6.0, 10.0, 3), (3.0, 7.0, 4), (9.0, 12.0, 5)];
        let t = make_tree();
        for x_int in 0..=15 {
            let x = x_int as f64;
            let mut tree_res = t.stab(x);
            let mut brute = brute_stab(x, &raw);
            tree_res.sort();
            brute.sort();
            assert_eq!(tree_res, brute, "mismatch at x={x}");
        }
    }
}
