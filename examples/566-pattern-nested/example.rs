#[derive(Debug,Clone,PartialEq)]
enum Suit { C, D, H, S }
#[derive(Debug,Clone,PartialEq)]
enum Rank { N(u8), J, Q, K, A }
#[derive(Debug,Clone)]
struct Card { r: Rank, s: Suit }
#[derive(Debug)]
enum Hand { Empty, One(Card), Two(Card,Card) }

fn rank_val(r: &Rank) -> u32 {
    match r { Rank::N(n)=>*n as u32, Rank::J|Rank::Q|Rank::K=>10, Rank::A=>11 }
}

fn hand_pts(h: &Hand) -> u32 {
    match h {
        Hand::Empty       => 0,
        Hand::One(c)      => rank_val(&c.r),
        Hand::Two(a,b)    => rank_val(&a.r) + rank_val(&b.r),
    }
}

fn describe(h: &Hand) -> &'static str {
    match h {
        Hand::One(Card{r:Rank::A, s:Suit::S}) => "ace of spades!",
        Hand::One(Card{r:Rank::A, ..})        => "an ace",
        Hand::Two(Card{r:r1,..},Card{r:r2,..}) if r1==r2 => "a pair",
        Hand::Two(_,_)                         => "two cards",
        Hand::Empty                            => "nothing",
    }
}

fn main() {
    let h1 = Hand::One(Card{r:Rank::A, s:Suit::S});
    let h2 = Hand::Two(Card{r:Rank::K,s:Suit::H}, Card{r:Rank::K,s:Suit::C});
    println!("{} ({} pts)", describe(&h1), hand_pts(&h1));
    println!("{} ({} pts)", describe(&h2), hand_pts(&h2));

    // Nested array patterns
    if let [[a,_],[_,d]] = [[1,2],[3,4]] { println!("Diagonal: {} {}", a, d); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn ace_spades() {
        assert_eq!(describe(&Hand::One(Card{r:Rank::A,s:Suit::S})), "ace of spades!");
    }
    #[test] fn pair_pts() {
        let h = Hand::Two(Card{r:Rank::N(7),s:Suit::H}, Card{r:Rank::N(3),s:Suit::C});
        assert_eq!(hand_pts(&h), 10);
    }
}
