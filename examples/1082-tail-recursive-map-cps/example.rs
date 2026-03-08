// === Solution 1: Naive recursive map ===
// Not tail-recursive — stack overflows on large inputs.
pub fn map_naive<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U> {
    match list {
        [] => vec![],
        [head, tail @ ..] => {
            let mut result = vec![f(head)];
            result.extend(map_naive(f, tail));
            result
        }
    }
}

// === Solution 2: Accumulator-based map (loop) ===
// Rust translation of OCaml's tail-recursive `go acc = function | [] -> List.rev acc | h::t -> go (f h :: acc) t`
// Vec::push appends in order (unlike OCaml cons which prepends), so no reverse needed.
pub fn map_acc<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U> {
    let mut acc = Vec::with_capacity(list.len());
    for item in list {
        acc.push(f(item));
    }
    acc
}

// === Solution 3: CPS (Continuation-Passing Style) ===
// Builds a chain of boxed closures, each wrapping the previous one.
pub fn map_cps<T, U, F: Fn(&T) -> U>(f: &F, list: &[T]) -> Vec<U> {
    let mut cont: Box<dyn FnOnce(Vec<U>) -> Vec<U>> = Box::new(|v| v);

    for item in list.iter().rev() {
        let mapped = f(item);
        let prev = cont;
        cont = Box::new(move |mut rest: Vec<U>| {
            rest.push(mapped);
            prev(rest)
        });
    }

    cont(Vec::new())
}

// === Solution 4: Idiomatic Rust ===
pub fn map_idiomatic<T, U, F: Fn(&T) -> U>(f: F, list: &[T]) -> Vec<U> {
    list.iter().map(f).collect()
}

fn main() {
    let input = [1, 2, 3, 4, 5];
    let double = |x: &i32| x * 2;

    println!("Input: {:?}", input);
    println!();
    println!("map_naive:    {:?}", map_naive(&double, &input));
    println!("map_acc:      {:?}", map_acc(&double, &input));
    println!("map_cps:      {:?}", map_cps(&double, &input));
    println!("map_idiomatic:{:?}", map_idiomatic(&double, &input));

    // Demonstrate that acc and idiomatic handle large inputs
    let big: Vec<i32> = (0..1_000_000).collect();
    let result = map_acc(&|x: &i32| x * 2, &big);
    println!();
    println!(
        "map_acc on 1M elements: len={}, first={}, last={}",
        result.len(),
        result[0],
        result[999_999]
    );

    let result = map_idiomatic(|x: &i32| x * 2, &big);
    println!(
        "map_idiomatic on 1M elements: len={}, first={}, last={}",
        result.len(),
        result[0],
        result[999_999]
    );
}

/* Output:
   Input: [1, 2, 3, 4, 5]

   map_naive:    [2, 4, 6, 8, 10]
   map_acc:      [2, 4, 6, 8, 10]
   map_cps:      [2, 4, 6, 8, 10]
   map_idiomatic:[2, 4, 6, 8, 10]

   map_acc on 1M elements: len=1000000, first=0, last=1999998
   map_idiomatic on 1M elements: len=1000000, first=0, last=1999998
*/
