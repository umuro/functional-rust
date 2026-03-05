// 104: Mutable Borrowing — &mut T
// Exclusive writer: only ONE &mut at a time

fn increment(x: &mut i32) {
    *x += 1;
}

fn push_doubled(v: &mut Vec<i32>, val: i32) {
    v.push(val * 2);
}

fn swap_first_last(v: &mut [i32]) {
    if v.len() >= 2 {
        let last_idx = v.len() - 1;
        v.swap(0, last_idx);
    }
}

// This won't compile — demonstrates the rule:
// fn bad_example() {
//     let mut v = vec![1, 2, 3];
//     let r1 = &mut v;
//     let r2 = &mut v; // ERROR: second mutable borrow
//     r1.push(4);
//     r2.push(5);
// }

// Also can't mix &mut and &:
// fn bad_example2() {
//     let mut v = vec![1, 2, 3];
//     let r1 = &v;     // shared borrow
//     let r2 = &mut v; // ERROR: can't borrow as mutable
//     println!("{:?}", r1);
// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut x = 0;
        increment(&mut x);
        increment(&mut x);
        assert_eq!(x, 2);
    }

    #[test]
    fn test_push_doubled() {
        let mut v = vec![1, 2];
        push_doubled(&mut v, 3);
        assert_eq!(v, vec![1, 2, 6]);
    }

    #[test]
    fn test_swap() {
        let mut v = vec![1, 2, 3, 4, 5];
        swap_first_last(&mut v);
        assert_eq!(v, vec![5, 2, 3, 4, 1]);
    }
}
