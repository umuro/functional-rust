use example_278_iterator_last::last;

fn main() {
    let nums = vec![1i32, 2, 3, 4, 5];
    println!(
        "Last: {}",
        last(&nums).map_or("None".to_string(), |n| n.to_string())
    );
    println!(
        "Last of []: {}",
        last(&[] as &[i32]).map_or("None".to_string(), |n| n.to_string())
    );

    let words = vec!["apple", "banana", "cherry"];
    println!("Last word: {}", last(&words).map_or("None", |w| w));

    let last_even = nums.iter().copied().filter(|x| x % 2 == 0).last();
    println!(
        "Last even: {}",
        last_even.map_or("None".to_string(), |n| n.to_string())
    );
}
