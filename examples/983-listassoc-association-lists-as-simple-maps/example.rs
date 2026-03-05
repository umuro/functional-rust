pub fn assoc<'a, K, V>(key: &K, pairs: &'a [(K, V)]) -> Option<&'a V>
where
    K: PartialEq,
{
    pairs.iter().find(|(k, _)| k == key).map(|(_, v)| v)
}

pub fn mem_assoc<K, V>(key: &K, pairs: &[(K, V)]) -> bool
where
    K: PartialEq,
{
    pairs.iter().any(|(k, _)| k == key)
}

pub fn remove_assoc<'a, K, V>(key: &K, pairs: &'a [(K, V)]) -> Vec<&'a (K, V)>
where
    K: PartialEq,
{
    let mut removed = false;
    pairs
        .iter()
        .filter(|(k, _)| {
            if !removed && k == key {
                removed = true;
                false
            } else {
                true
            }
        })
        .collect()
}

pub fn assoc_recursive<'a, K, V>(key: &K, pairs: &'a [(K, V)]) -> Option<&'a V>
where
    K: PartialEq,
{
    match pairs {
        [] => None,
        [(k, v), ..] if k == key => Some(v),
        [_, rest @ ..] => assoc_recursive(key, rest),
    }
}

fn main() {
    let phonebook: &[(&str, &str)] = &[
        ("Alice", "555-1234"),
        ("Bob", "555-5678"),
        ("Carol", "555-9012"),
    ];

    let bobs_number = assoc(&"Bob", phonebook);
    println!("Bob's number: {:?}", bobs_number);

    let has_dave = mem_assoc(&"Dave", phonebook);
    println!("Dave in phonebook: {}", has_dave);

    let without_bob = remove_assoc(&"Bob", phonebook);
    let remaining: Vec<&str> = without_bob.iter().map(|(k, _)| *k).collect();
    println!("After removing Bob: {:?}", remaining);

    let carol = assoc_recursive(&"Carol", phonebook);
    println!("Carol (recursive): {:?}", carol);
}

/* Output:
   Bob's number: Some("555-5678")
   Dave in phonebook: false
   After removing Bob: ["Alice", "Carol"]
   Carol (recursive): Some("555-9012")
*/
