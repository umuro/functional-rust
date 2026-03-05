#[derive(Debug,Clone,Copy,PartialEq)]
struct Meters(f64);
#[derive(Debug,Clone,Copy,PartialEq)]
struct Seconds(f64);
#[derive(Debug,Clone,Copy,PartialEq)]
struct Rgb(u8, u8, u8);

fn add(Meters(a): Meters, Meters(b): Meters) -> Meters { Meters(a + b) }
fn speed(Meters(d): Meters, Seconds(t): Seconds) -> f64 { d / t }

fn to_gray(Rgb(r,g,b): Rgb) -> Rgb {
    let avg = ((r as u16 + g as u16 + b as u16) / 3) as u8;
    Rgb(avg, avg, avg)
}

fn show_rgb(Rgb(r,g,b): Rgb) -> String { format!("rgb({},{},{})", r, g, b) }

fn main() {
    let Meters(total) = add(Meters(100.0), Meters(50.0));
    println!("{:.1} m", total);
    println!("{:.1} m/s", speed(Meters(200.0), Seconds(10.0)));
    println!("{}", show_rgb(to_gray(Rgb(255,0,0))));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_add() { assert_eq!(add(Meters(3.0),Meters(4.0)), Meters(7.0)); }
    #[test] fn test_gray() {
        let Rgb(r,g,b) = to_gray(Rgb(60,120,180));
        assert_eq!(r, g); assert_eq!(g, b);
    }
}
