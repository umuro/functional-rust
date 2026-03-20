#![allow(clippy::all)]
use std::collections::HashSet;

pub fn sum_of_multiples(factors: &[u64], limit: u64) -> u64 {
    factors
        .iter()
        .filter(|&&f| f != 0)
        .flat_map(|&f| (f..limit).step_by(f as usize))
        .collect::<HashSet<u64>>()
        .into_iter()
        .sum()
}

pub fn sum_of_multiples_fold(factors: &[u64], limit: u64) -> u64 {
    let set: HashSet<u64> =
        factors
            .iter()
            .filter(|&&f| f != 0)
            .fold(HashSet::new(), |mut acc, &f| {
                (f..limit).step_by(f as usize).for_each(|m| {
                    acc.insert(m);
                });
                acc
            });
    set.into_iter().sum()
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm_pair(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

pub fn sum_of_multiples_math(factors: &[u64], limit: u64) -> u64 {
    fn sum_divisible(k: u64, limit: u64) -> u64 {
        if k == 0 {
            return 0;
        }
        let n = (limit - 1) / k;
        k * n * (n + 1) / 2
    }

    let unique: Vec<u64> = {
        let mut seen = HashSet::new();
        factors
            .iter()
            .filter(|&&f| f != 0 && seen.insert(f))
            .copied()
            .collect()
    };

    let n = unique.len();
    (1u64..=(1 << n) - 1)
        .map(|mask| {
            let mut lcm = 1u64;
            let mut bits = 0u32;
            for (i, &val) in unique.iter().enumerate() {
                if mask & (1 << i) != 0 {
                    bits += 1;
                    lcm = lcm_pair(lcm, val);
                    if lcm >= limit {
                        return 0i64;
                    }
                }
            }
            let s = sum_divisible(lcm, limit) as i64;
            if bits % 2 == 1 {
                s
            } else {
                -s
            }
        })
        .sum::<i64>() as u64
}

/* Output:
   sum_of_multiples([3, 5], 1000)          = 233168
   sum_of_multiples([2,3,5,7,11], 10000)   = 39614537
   sum_of_multiples([], 1000)               = 0
   sum_of_multiples([0, 3], 10)             = 18

   -- fold variant --
   sum_of_multiples_fold([3, 5], 1000)      = 233168

   -- math (inclusion-exclusion) variant --
   sum_of_multiples_math([3, 5], 1000)      = 233168
   sum_of_multiples_math([2,3,5,7,11],10000)= 39614537
*/
