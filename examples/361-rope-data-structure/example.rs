#[derive(Debug, Clone)]
enum Rope {
    Leaf(String),
    Node { left: Box<Rope>, right: Box<Rope>, length: usize },
}

impl Rope {
    fn leaf(s: impl Into<String>) -> Self { Self::Leaf(s.into()) }

    fn length(&self) -> usize {
        match self { Self::Leaf(s) => s.len(), Self::Node{length,..} => *length }
    }

    fn concat(left: Rope, right: Rope) -> Rope {
        if left.length() == 0 { return right; }
        if right.length() == 0 { return left; }
        let length = left.length() + right.length();
        Rope::Node { left: Box::new(left), right: Box::new(right), length }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Leaf(s) => s.clone(),
            Self::Node{left,right,..} => left.to_string() + &right.to_string(),
        }
    }

    fn char_at(&self, idx: usize) -> Option<u8> {
        match self {
            Self::Leaf(s) => s.as_bytes().get(idx).copied(),
            Self::Node{left,right,..} => {
                let ll = left.length();
                if idx < ll { left.char_at(idx) } else { right.char_at(idx - ll) }
            }
        }
    }

    fn split_at(self, idx: usize) -> (Rope, Rope) {
        match self {
            Rope::Leaf(s) => {
                let (a,b) = s.split_at(idx.min(s.len()));
                (Rope::leaf(a), Rope::leaf(b))
            }
            Rope::Node{left,right,..} => {
                let ll = left.length();
                if idx <= ll {
                    let (la, lb) = left.split_at(idx);
                    (la, Rope::concat(lb, *right))
                } else {
                    let (ra, rb) = right.split_at(idx - ll);
                    (Rope::concat(*left, ra), rb)
                }
            }
        }
    }
}

fn main() {
    let r = Rope::concat(Rope::leaf("Hello, "), Rope::concat(Rope::leaf("World"), Rope::leaf("!")));
    println!("{}", r.to_string());
    println!("Length: {}", r.length());
    println!("Char at 7: {}", r.char_at(7).unwrap() as char);

    let (left, right) = r.clone().split_at(7);
    println!("Split: '{}' | '{}'", left.to_string(), right.to_string());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn concat_and_string() {
        let r = Rope::concat(Rope::leaf("foo"), Rope::leaf("bar"));
        assert_eq!(r.to_string(), "foobar");
        assert_eq!(r.length(), 6);
    }
    #[test] fn char_at() {
        let r = Rope::concat(Rope::leaf("ab"), Rope::leaf("cd"));
        assert_eq!(r.char_at(2), Some(b'c'));
    }
    #[test] fn split() {
        let r = Rope::concat(Rope::leaf("hello"), Rope::leaf(" world"));
        let (l,rr) = r.split_at(5);
        assert_eq!(l.to_string(), "hello");
        assert_eq!(rr.to_string(), " world");
    }
}
