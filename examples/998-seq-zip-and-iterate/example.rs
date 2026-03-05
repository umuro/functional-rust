/// Zip two slices into a vector of pairs.
///
/// Mirrors `Seq.zip letters numbers |> List.of_seq`.
pub fn zip_slices<A: Copy, B: Copy>(a: &[A], b: &[B]) -> Vec<(A, B)> {
    a.iter().copied().zip(b.iter().copied()).collect()
}

/// The Collatz step function: n/2 if even, 3n+1 if odd.
pub fn collatz(n: u64) -> u64 {
    if n.is_multiple_of(2) {
        n / 2
    } else {
        3 * n + 1
    }
}

/// Collect the first `n` terms of the sequence produced by repeated application of `f`.
///
/// Mirrors `Seq.iterate f start |> Seq.take n |> List.of_seq`.
pub fn iterate<T, F>(f: F, start: T, n: usize) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> T,
{
    std::iter::successors(Some(start), |prev| Some(f(prev)))
        .take(n)
        .collect()
}

/// Recursive style — accumulator mirrors OCaml's tail-recursive helper.
pub fn iterate_recursive<T, F>(f: &F, start: T, n: usize) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> T,
{
    fn go<T: Clone, F: Fn(&T) -> T>(f: &F, current: T, remaining: usize, acc: &mut Vec<T>) {
        if remaining == 0 {
            return;
        }
        let next = f(&current);
        acc.push(current);
        go(f, next, remaining - 1, acc);
    }

    let mut acc = Vec::with_capacity(n);
    go(f, start, n, &mut acc);
    acc
}

fn main() {
    // Seq.zip example
    let letters = ['a', 'b', 'c', 'd'];
    let numbers = [1i32, 2, 3, 4];
    let pairs = zip_slices(&letters, &numbers);
    print!("zip: ");
    for (c, n) in &pairs {
        print!("({c}, {n}) ");
    }
    println!();

    // Seq.iterate collatz example
    let seq = iterate(|&n| collatz(n), 27u64, 20);
    print!("collatz from 27 (20 steps): ");
    for x in &seq {
        print!("{x} ");
    }
    println!();

    // Doubling sequence
    let doubles = iterate(|&n: &u64| n * 2, 1u64, 8);
    println!("doubling: {doubles:?}");
}

/* Output:
   zip: (a, 1) (b, 2) (c, 3) (d, 4)
   collatz from 27 (20 steps): 27 82 41 124 62 31 94 47 142 71 214 107 322 161 484 242 121 364 182 91
   doubling: [1, 2, 4, 8, 16, 32, 64, 128]
*/
