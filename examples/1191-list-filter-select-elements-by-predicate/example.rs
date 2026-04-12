/// List.filter — Select Elements by Predicate
/// Idiomatic and recursive implementations.

pub fn filter<T: Clone, F>(items: &[T], pred: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| pred(x)).cloned().collect()
}

pub fn filter_recursive<T: Clone, F>(items: &[T], pred: &F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    match items {
        [] => vec![],
        [head, rest @ ..] => {
            let mut tail = filter_recursive(rest, pred);
            if pred(head) {
                tail.insert(0, head.clone());
            }
            tail
        }
    }
}

pub fn filter_evens(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

pub fn filter_long<'a>(words: &[&'a str], min_len: usize) -> Vec<&'a str> {
    words.iter().filter(|s| s.len() > min_len).copied().collect()
}

fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    let evens = filter(&numbers, |&x| x % 2 == 0);
    let odds = filter(&numbers, |&x| x % 2 != 0);
    println!("numbers = {:?}", numbers);
    println!("evens   = {:?}", evens);
    println!("odds    = {:?}", odds);

    let words = ["hi", "hello", "hey", "salutation", "greetings"];
    let long_words = filter_long(&words, 4);
    println!("words longer than 4 chars: {:?}", long_words);

    let pred = |x: &i32| *x > 4;
    let recursive_result = filter_recursive(&numbers, &pred);
    println!("recursive filter (> 4): {:?}", recursive_result);
}

/* Output:
   numbers = [1, 2, 3, 4, 5, 6, 7, 8]
   evens   = [2, 4, 6, 8]
   odds    = [1, 3, 5, 7]
   words longer than 4 chars: ["hello", "salutation", "greetings"]
   recursive filter (> 4): [5, 6, 7, 8]
*/
