// Histomorphism: algebra receives a "history" comonad
// We represent history as a slice of previously computed results

// Histo over natural numbers 0..=n
fn histo<R: Clone + Default>(n: usize, alg: impl Fn(&[R], usize) -> R) -> Vec<R> {
    let mut memo = vec![R::default(); n+1];
    for i in 0..=n {
        memo[i] = alg(&memo[..i], i);
    }
    memo
}

fn fibonacci(n: usize) -> u64 {
    let memo = histo(n, |hist, i| match i {
        0 => 0,
        1 => 1,
        _ => hist[i-1] + hist[i-2],
    });
    memo[n]
}

fn tribonacci(n: usize) -> u64 {
    let memo = histo(n, |hist, i| match i {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => hist[i-1] + hist[i-2] + hist[i-3],
    });
    memo[n]
}

// Histomorphism over trees (fold with all subtree results available)
#[derive(Debug,Clone)]
enum Tree<A> { Leaf(A), Node(Box<Tree<A>>, Box<Tree<A>>) }

fn histo_tree<A: Clone, R: Clone>(
    tree: &Tree<A>,
    leaf_alg:   impl Fn(A) -> R + Copy,
    branch_alg: impl Fn(R, R, &Tree<A>, &Tree<A>) -> R + Copy,
) -> R {
    match tree {
        Tree::Leaf(a) => leaf_alg(a.clone()),
        Tree::Node(l,r) => {
            let lr = histo_tree(l, leaf_alg, branch_alg);
            let rr = histo_tree(r, leaf_alg, branch_alg);
            branch_alg(lr, rr, l, r)
        }
    }
}

fn main() {
    println!("fib(0..10): {:?}", histo(10, |hist,i| match i { 0=>0u64, 1=>1, _=>hist[i-1]+hist[i-2] }));
    println!("fib(20) = {}", fibonacci(20));
    println!("trib(10) = {}", tribonacci(10));

    // Dynamic programming via histomorphism
    // Coin change: fewest coins to make amount
    let coins = vec![1u64,5,10,25];
    let n = 41usize;
    let dp = histo(n, |hist, amount| {
        if amount == 0 { return 0u64; }
        coins.iter()
            .filter(|&&c| c as usize <= amount)
            .map(|&c| hist[amount - c as usize] + 1)
            .min()
            .unwrap_or(u64::MAX/2)
    });
    println!("min coins for {}: {}", n, dp[n]);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fib_test() {
        assert_eq!(fibonacci(0), 0); assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(10), 55);
    }
    #[test] fn trib_test() { assert_eq!(tribonacci(6), 13); }
}
