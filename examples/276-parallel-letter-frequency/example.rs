use std::collections::HashMap;

/// Count letter frequencies in a single string.
fn letter_freq(s: &str) -> HashMap<char, usize> {
    s.chars().fold(HashMap::new(), |mut map, c| {
        let c = c.to_ascii_lowercase();
        if c.is_ascii_lowercase() {
            *map.entry(c).or_insert(0) += 1;
        }
        map
    })
}

/// Merge two frequency maps by summing counts.
fn merge_maps(mut a: HashMap<char, usize>, b: &HashMap<char, usize>) -> HashMap<char, usize> {
    for (&ch, &count) in b {
        *a.entry(ch).or_insert(0) += count;
    }
    a
}

/// Map-reduce: compute letter frequencies across multiple texts.
fn parallel_frequency(texts: &[&str]) -> HashMap<char, usize> {
    texts
        .iter()
        .map(|text| letter_freq(text))
        .fold(HashMap::new(), |acc, freq| merge_maps(acc, &freq))
}

/// Recursive version — processes texts one at a time.
fn parallel_frequency_recursive(texts: &[&str]) -> HashMap<char, usize> {
    match texts {
        [] => HashMap::new(),
        [single] => letter_freq(single),
        [head, rest @ ..] => {
            let head_freq = letter_freq(head);
            let rest_freq = parallel_frequency_recursive(rest);
            merge_maps(head_freq, &rest_freq)
        }
    }
}

fn main() {
    let texts = &["Hello World", "Functional Programming", "OCaml is Great"];
    let freq = parallel_frequency(texts);

    let mut entries: Vec<_> = freq.iter().collect();
    entries.sort_by_key(|&(ch, _)| *ch);
    for (ch, count) in &entries {
        print!("{}:{} ", ch, count);
    }
    println!();

    println!("'o' count = {}", freq[&'o']);

    // Verify recursive version matches
    let freq2 = parallel_frequency_recursive(texts);
    assert_eq!(freq, freq2);
    println!("Recursive matches iterative: ✓");
}

/* Output:
   a:4 c:2 d:1 e:2 f:1 g:3 h:1 i:3 l:5 m:3 n:3 o:5 p:1 r:4 s:1 t:2 u:1 w:1
   'o' count = 5
   Recursive matches iterative: ✓
*/
