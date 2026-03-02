// List Operations and Recursion in Rust

// Basic recursive list operations (using slices)
fn length<T>(lst: &[T]) -> usize {
    match lst {
        [] => 0,
        [_, tail @ ..] => 1 + length(tail),
    }
}

fn sum(lst: &[i32]) -> i32 {
    match lst {
        [] => 0,
        [head, tail @ ..] => head + sum(tail),
    }
}

// Append creates a new Vec (Rust doesn't have built-in cons lists)
fn append<T: Clone>(lst1: &[T], lst2: &[T]) -> Vec<T> {
    match lst1 {
        [] => lst2.to_vec(),
        [head, tail @ ..] => {
            let mut result = vec![head.clone()];
            result.extend_from_slice(&append(tail, lst2));
            result
        }
    }
}

// Tail-recursive versions for efficiency
fn length_tr<T>(lst: &[T]) -> usize {
    fn aux<T>(acc: usize, lst: &[T]) -> usize {
        match lst {
            [] => acc,
            [_, tail @ ..] => aux(acc + 1, tail),
        }
    }
    aux(0, lst)
}

fn sum_tr(lst: &[i32]) -> i32 {
    fn aux(acc: i32, lst: &[i32]) -> i32 {
        match lst {
            [] => acc,
            [head, tail @ ..] => aux(acc + head, tail),
        }
    }
    aux(0, lst)
}

// Map and filter using recursion
fn map<T, U, F>(f: F, lst: &[T]) -> Vec<U>
where
    F: Fn(&T) -> U + Copy,
{
    match lst {
        [] => vec![],
        [head, tail @ ..] => {
            let mut result = vec![f(head)];
            result.extend(map(f, tail));
            result
        }
    }
}

fn filter<T: Clone, F>(pred: F, lst: &[T]) -> Vec<T>
where
    F: Fn(&T) -> bool + Copy,
{
    match lst {
        [] => vec![],
        [head, tail @ ..] => {
            if pred(head) {
                let mut result = vec![head.clone()];
                result.extend(filter(pred, tail));
                result
            } else {
                filter(pred, tail)
            }
        }
    }
}

// Take and drop
fn take<T: Clone>(n: usize, lst: &[T]) -> Vec<T> {
    match (n, lst) {
        (0, _) | (_, []) => vec![],
        (n, [head, tail @ ..]) => {
            let mut result = vec![head.clone()];
            result.extend(take(n - 1, tail));
            result
        }
    }
}

fn drop<T>(n: usize, lst: &[T]) -> &[T] {
    match (n, lst) {
        (0, _) => lst,
        (_, []) => &[],
        (n, [_, tail @ ..]) => drop(n - 1, tail),
    }
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    println!("List: {:?}", numbers);
    println!("Length: {}", length(&numbers));
    println!("Sum: {}", sum(&numbers));
    
    let doubled = map(|x| x * 2, &numbers);
    println!("Doubled: {:?}", doubled);
    
    let evens = filter(|x| x % 2 == 0, &numbers);
    println!("Evens: {:?}", evens);
    
    println!("Take 3: {:?}", take(3, &numbers));
    println!("Drop 2: {:?}", drop(2, &numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length() {
        assert_eq!(length(&[1, 2, 3]), 3);
        assert_eq!(length_tr(&[1, 2, 3]), 3);
        assert_eq!(length::<i32>(&[]), 0);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4]), 10);
        assert_eq!(sum_tr(&[1, 2, 3, 4]), 10);
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn test_map() {
        assert_eq!(map(|x| x * 2, &[1, 2, 3]), vec![2, 4, 6]);
    }

    #[test]
    fn test_filter() {
        assert_eq!(filter(|x| x % 2 == 0, &[1, 2, 3, 4]), vec![2, 4]);
    }

    #[test]
    fn test_take_drop() {
        assert_eq!(take(2, &[1, 2, 3, 4]), vec![1, 2]);
        assert_eq!(drop(2, &[1, 2, 3, 4]), &[3, 4]);
    }
}
