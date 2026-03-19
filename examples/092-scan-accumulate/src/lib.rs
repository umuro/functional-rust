#![allow(clippy::result_unit_err)]
#![allow(clippy::manual_is_multiple_of)]
#![allow(unused_variables)]
#![allow(clippy::match_like_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::explicit_iter_loop)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::char_lit_as_u8)]
#![allow(clippy::while_let_loop)]
#![allow(clippy::manual_strip)]
#![allow(clippy::useless_vec)]
#![allow(clippy::needless_borrow)]
#![allow(clippy::redundant_closure)]
#![allow(unused_imports)]
#![allow(dead_code)]
// 092: Scan with Accumulator

fn running_sum(v: &[i32]) -> Vec<i32> {
    let mut result = vec![0];
    result.extend(v.iter().scan(0, |acc, &x| {
        *acc += x;
        Some(*acc)
    }));
    result
}

fn running_max(v: &[i32]) -> Vec<i32> {
    if v.is_empty() {
        return vec![];
    }
    let mut max_val = v[0];
    let mut result = vec![max_val];
    for &x in &v[1..] {
        max_val = max_val.max(x);
        result.push(max_val);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(&[1, 2, 3, 4]), vec![0, 1, 3, 6, 10]);
    }

    #[test]
    fn test_running_max() {
        assert_eq!(running_max(&[3, 1, 4, 1, 5]), vec![3, 3, 4, 4, 5]);
    }
}
