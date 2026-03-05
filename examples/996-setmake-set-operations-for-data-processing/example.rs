use std::collections::HashSet;

pub fn unique_words<'a>(words: &[&'a str]) -> HashSet<&'a str> {
    words.iter().copied().collect()
}

pub fn remove_stopwords<'a>(
    words: &HashSet<&'a str>,
    stopwords: &HashSet<&'a str>,
) -> HashSet<&'a str> {
    words.difference(stopwords).copied().collect()
}

pub fn is_member(set: &HashSet<&str>, word: &str) -> bool {
    set.contains(word)
}

pub fn union<'a>(a: &HashSet<&'a str>, b: &HashSet<&'a str>) -> HashSet<&'a str> {
    a.union(b).copied().collect()
}

pub fn intersect<'a>(a: &HashSet<&'a str>, b: &HashSet<&'a str>) -> HashSet<&'a str> {
    a.intersection(b).copied().collect()
}

fn main() {
    let words = ["the", "cat", "sat", "on", "the", "mat", "the", "cat"];
    let unique = unique_words(&words);
    println!("Unique words: {}", unique.len());

    let stopwords: HashSet<&str> = ["the", "on", "a", "an"].iter().copied().collect();
    let content = remove_stopwords(&unique, &stopwords);
    let mut sorted: Vec<&&str> = content.iter().collect();
    sorted.sort();
    println!("Content words: {:?}", sorted);

    println!("Is 'cat' a member? {}", is_member(&unique, "cat"));
    println!("Is 'dog' a member? {}", is_member(&unique, "dog"));

    let other: HashSet<&str> = ["mat", "rat", "bat"].iter().copied().collect();
    let shared = intersect(&unique, &other);
    println!("Shared with {{mat, rat, bat}}: {:?}", shared);
}

/* Output:
   Unique words: 5
   Content words: ["cat", "mat", "sat"]
   Is 'cat' a member? true
   Is 'dog' a member? false
   Shared with {mat, rat, bat}: {"mat"}
*/
