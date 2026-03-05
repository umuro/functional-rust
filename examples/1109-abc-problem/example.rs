const BLOCKS: &[(char, char)] = &[
    ('B', 'O'),
    ('X', 'K'),
    ('D', 'Q'),
    ('C', 'P'),
    ('N', 'A'),
    ('G', 'T'),
    ('R', 'E'),
    ('T', 'G'),
    ('Q', 'D'),
    ('F', 'S'),
    ('J', 'W'),
    ('H', 'U'),
    ('V', 'I'),
    ('A', 'N'),
    ('O', 'B'),
    ('E', 'R'),
    ('F', 'S'),
    ('L', 'Y'),
    ('P', 'C'),
    ('Z', 'M'),
];

fn block_has(block: (char, char), c: char) -> bool {
    block.0 == c || block.1 == c
}

// Solution 1: Idiomatic — index-based backtracking with slice patterns
pub fn can_make_word(word: &str) -> bool {
    let letters: Vec<char> = word.to_uppercase().chars().collect();
    let available: Vec<usize> = (0..BLOCKS.len()).collect();
    can_spell(&letters, &available)
}

fn can_spell(letters: &[char], available: &[usize]) -> bool {
    match letters {
        [] => true,
        [c, rest @ ..] => available.iter().enumerate().any(|(pos, &idx)| {
            block_has(BLOCKS[idx], *c) && {
                let remaining: Vec<usize> = available[..pos]
                    .iter()
                    .chain(&available[pos + 1..])
                    .copied()
                    .collect();
                can_spell(rest, &remaining)
            }
        }),
    }
}

// Solution 2: Functional — partition mirrors OCaml List.partition style
pub fn can_make_word_functional(word: &str) -> bool {
    let letters: Vec<char> = word.to_uppercase().chars().collect();
    can_spell_functional(&letters, BLOCKS.to_vec())
}

fn can_spell_functional(letters: &[char], blocks: Vec<(char, char)>) -> bool {
    match letters {
        [] => true,
        [c, rest @ ..] => {
            let (matching, non_matching): (Vec<_>, Vec<_>) =
                blocks.into_iter().partition(|&b| block_has(b, *c));
            matching.iter().enumerate().any(|(i, _)| {
                let remaining: Vec<_> = matching[..i]
                    .iter()
                    .chain(&matching[i + 1..])
                    .chain(&non_matching)
                    .copied()
                    .collect();
                can_spell_functional(rest, remaining)
            })
        }
    }
}

fn main() {
    let words = [
        ("A", true),
        ("BARK", true),
        ("BOOK", false),
        ("TREAT", true),
        ("COMMON", false),
        ("SQUAD", true),
        ("CONFUSE", true),
    ];

    println!("=== Idiomatic (index-based backtracking) ===");
    for (word, expected) in words {
        let result = can_make_word(word);
        println!("can_make_word({word:?}) = {result}  (expected: {expected})");
    }

    println!("\n=== Functional (partition-based) ===");
    for (word, expected) in words {
        let result = can_make_word_functional(word);
        println!("can_make_word_functional({word:?}) = {result}  (expected: {expected})");
    }
}

/* Output:
   === Idiomatic (index-based backtracking) ===
   can_make_word("A") = true  (expected: true)
   can_make_word("BARK") = true  (expected: true)
   can_make_word("BOOK") = false  (expected: false)
   can_make_word("TREAT") = true  (expected: true)
   can_make_word("COMMON") = false  (expected: false)
   can_make_word("SQUAD") = true  (expected: true)
   can_make_word("CONFUSE") = true  (expected: true)

   === Functional (partition-based) ===
   can_make_word_functional("A") = true  (expected: true)
   can_make_word_functional("BARK") = true  (expected: true)
   can_make_word_functional("BOOK") = false  (expected: false)
   can_make_word_functional("TREAT") = true  (expected: true)
   can_make_word_functional("COMMON") = false  (expected: false)
   can_make_word_functional("SQUAD") = true  (expected: true)
   can_make_word_functional("CONFUSE") = true  (expected: true)
*/
