fn fizzbuzz(n: u32) -> String {
    match (n%3==0, n%5==0) {
        (true,true)   => "FizzBuzz".into(),
        (true,false)  => "Fizz".into(),
        (false,true)  => "Buzz".into(),
        (false,false) => n.to_string(),
    }
}

#[derive(Debug,Clone,Copy)]
enum Light { Red, Yellow, Green }

fn next_light(light: Light, emergency: bool) -> Light {
    match (light, emergency) {
        (_,           true)  => Light::Red,
        (Light::Red,  false) => Light::Green,
        (Light::Green,false) => Light::Yellow,
        (Light::Yellow,false)=> Light::Red,
    }
}

fn cmp(a: i32, b: i32) -> &'static str {
    match (a>b, a<b) { (true,false)=>"gt", (false,true)=>"lt", _=>"eq" }
}

fn main() {
    let fb: Vec<_> = (1..=15).map(fizzbuzz).collect();
    println!("{}", fb.join(" "));

    let mut l = Light::Green;
    for i in 0..5 {
        let em = i == 3;
        l = next_light(l, em);
        println!("step {}: {:?} (em={})", i+1, l, em);
    }
    for (a,b) in [(1,2),(2,1),(3,3)] { println!("{} vs {}: {}", a, b, cmp(a,b)); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn fb15() { assert_eq!(fizzbuzz(15), "FizzBuzz"); }
    #[test] fn fb3()  { assert_eq!(fizzbuzz(3),  "Fizz"); }
    #[test] fn em()   { assert!(matches!(next_light(Light::Green, true), Light::Red)); }
}
