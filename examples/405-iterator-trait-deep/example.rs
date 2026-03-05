// Iterator adapters and combinators in Rust

fn main() {
    // Basic pipeline
    let sum_of_squares_of_evens: i32 = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .sum();
    println!("Sum of squares of evens 1..10: {}", sum_of_squares_of_evens);

    // flat_map
    let pairs: Vec<(i32, i32)> = (1..=4)
        .flat_map(|x| (1..=4).map(move |y| (x, y)))
        .filter(|(x, y)| x < y)
        .collect();
    println!("Pairs (x<y) from 1..4: {} pairs", pairs.len());

    // zip and enumerate
    let names = ["Alice", "Bob", "Carol"];
    let scores = [95, 87, 91];
    names.iter().zip(scores.iter()).enumerate().for_each(|(i, (name, score))| {
        println!("  {}. {}: {}", i + 1, name, score);
    });

    // scan (stateful)
    let running_total: Vec<i32> = (1..=5)
        .scan(0, |state, x| { *state += x; Some(*state) })
        .collect();
    println!("Running total: {:?}", running_total);

    // take_while / skip_while
    let data = vec![1, 3, 5, 2, 4, 6, 1, 2];
    let odds_prefix: Vec<_> = data.iter().take_while(|&&x| x % 2 != 0).collect();
    println!("Odd prefix: {:?}", odds_prefix);

    // chain
    let a = 1..=3;
    let b = 7..=9;
    let chained: Vec<i32> = a.chain(b).collect();
    println!("Chained: {:?}", chained);

    // partition
    let (evens, odds): (Vec<i32>, Vec<i32>) = (1..=10).partition(|x| x % 2 == 0);
    println!("Evens: {:?}", evens);
    println!("Odds: {:?}", odds);

    // unzip
    let (names2, scores2): (Vec<_>, Vec<_>) = vec![("Alice", 95), ("Bob", 87)].into_iter().unzip();
    println!("Names: {:?}, Scores: {:?}", names2, scores2);

    // Custom iterator — Fibonacci
    struct Fib { a: u64, b: u64 }
    impl Iterator for Fib {
        type Item = u64;
        fn next(&mut self) -> Option<u64> {
            let next = self.a + self.b;
            self.a = self.b;
            self.b = next;
            Some(self.a)
        }
    }
    let fibs: Vec<u64> = Fib { a: 0, b: 1 }.take(10).collect();
    println!("Fibonacci: {:?}", fibs);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_map() {
        let v: Vec<i32> = (1..=10).filter(|x| x % 2 == 0).map(|x| x * x).collect();
        assert_eq!(v, vec![4, 16, 36, 64, 100]);
    }

    #[test]
    fn test_flat_map() {
        let v: Vec<i32> = vec![1, 2, 3].into_iter().flat_map(|x| vec![x, x * 10]).collect();
        assert_eq!(v, vec![1, 10, 2, 20, 3, 30]);
    }

    #[test]
    fn test_scan() {
        let r: Vec<i32> = (1..=4).scan(0, |s, x| { *s += x; Some(*s) }).collect();
        assert_eq!(r, vec![1, 3, 6, 10]);
    }
}
