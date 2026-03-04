#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Yacht,
    Choice,
}

fn count(dice: &[u8], n: u8) -> u8 {
    dice.iter().filter(|&&d| d == n).count() as u8
}

pub fn score(dice: &[u8; 5], category: Category) -> u32 {
    match category {
        Category::Ones => u32::from(count(dice, 1)),
        Category::Twos => 2 * u32::from(count(dice, 2)),
        Category::Threes => 3 * u32::from(count(dice, 3)),
        Category::Fours => 4 * u32::from(count(dice, 4)),
        Category::Fives => 5 * u32::from(count(dice, 5)),
        Category::Sixes => 6 * u32::from(count(dice, 6)),
        Category::Choice => dice.iter().map(|&d| u32::from(d)).sum(),
        Category::Yacht => {
            if dice.iter().all(|&d| d == dice[0]) {
                50
            } else {
                0
            }
        }
        Category::FullHouse => {
            let mut counts = [0u8; 7];
            for &d in dice {
                counts[d as usize] += 1;
            }
            let freqs: Vec<u8> = counts.iter().copied().filter(|&c| c > 0).collect();
            let mut sorted_freqs = freqs.clone();
            sorted_freqs.sort_unstable();
            if sorted_freqs == [2, 3] {
                dice.iter().map(|&d| u32::from(d)).sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => (1u8..=6)
            .find(|&n| count(dice, n) >= 4)
            .map(|n| 4 * u32::from(n))
            .unwrap_or(0),
        Category::LittleStraight => {
            let mut sorted = *dice;
            sorted.sort_unstable();
            if sorted == [1, 2, 3, 4, 5] { 30 } else { 0 }
        }
        Category::BigStraight => {
            let mut sorted = *dice;
            sorted.sort_unstable();
            if sorted == [2, 3, 4, 5, 6] { 30 } else { 0 }
        }
    }
}

pub fn score_four_of_a_kind_recursive(dice: &[u8; 5], face: u8) -> u32 {
    if face > 6 {
        return 0;
    }
    if count(dice, face) >= 4 {
        4 * u32::from(face)
    } else {
        score_four_of_a_kind_recursive(dice, face + 1)
    }
}

fn main() {
    println!("Yacht [5,5,5,5,5] = {}", score(&[5, 5, 5, 5, 5], Category::Yacht));
    println!("FullHouse [2,2,3,3,3] = {}", score(&[2, 2, 3, 3, 3], Category::FullHouse));
    println!("Choice [1,2,3,4,5] = {}", score(&[1, 2, 3, 4, 5], Category::Choice));
    println!("LittleStraight [1,2,3,4,5] = {}", score(&[1, 2, 3, 4, 5], Category::LittleStraight));
    println!("BigStraight [2,3,4,5,6] = {}", score(&[2, 3, 4, 5, 6], Category::BigStraight));
    println!("FourOfAKind [3,3,3,3,1] = {}", score(&[3, 3, 3, 3, 1], Category::FourOfAKind));
    println!(
        "FourOfAKind recursive [2,2,2,2,5] = {}",
        score_four_of_a_kind_recursive(&[2, 2, 2, 2, 5], 1)
    );
}

/* Output:
   Yacht [5,5,5,5,5] = 50
   FullHouse [2,2,3,3,3] = 13
   Choice [1,2,3,4,5] = 15
   LittleStraight [1,2,3,4,5] = 30
   BigStraight [2,3,4,5,6] = 30
   FourOfAKind [3,3,3,3,1] = 12
   FourOfAKind recursive [2,2,2,2,5] = 8
*/
