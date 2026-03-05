use std::f64::consts::PI;

const DAY: f64 = 86_400.0;

pub fn rad_of_time(t: f64) -> f64 {
    t * 2.0 * PI / DAY
}

pub fn time_of_rad(r: f64) -> f64 {
    r * DAY / (2.0 * PI)
}

pub fn mean_angle(angles: &[f64]) -> f64 {
    let sum_sin: f64 = angles.iter().map(|a| a.sin()).sum();
    let sum_cos: f64 = angles.iter().map(|a| a.cos()).sum();
    sum_sin.atan2(sum_cos)
}

pub fn mean_time(times: &[f64]) -> f64 {
    let angles: Vec<f64> = times.iter().map(|&t| rad_of_time(t)).collect();
    let t = time_of_rad(mean_angle(&angles));
    if t < 0.0 { t + DAY } else { t }
}

pub fn parse_time(s: &str) -> Option<f64> {
    let mut parts = s.split(':');
    let h: i32 = parts.next()?.parse().ok()?;
    let m: i32 = parts.next()?.parse().ok()?;
    let sec: i32 = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((h * 3600 + m * 60 + sec) as f64)
}

pub fn format_time(t: f64) -> String {
    let t = (t + 0.5).floor() as u64;
    let h = t / 3600;
    let rem = t % 3600;
    let m = rem / 60;
    let s = rem % 60;
    format!("{h}:{m}:{s}")
}

pub fn mean_time_of_strings(times: &[&str]) -> Option<String> {
    let parsed: Vec<f64> = times
        .iter()
        .map(|t| parse_time(t))
        .collect::<Option<Vec<_>>>()?;
    Some(format_time(mean_time(&parsed)))
}

fn main() {
    let times = ["23:00:17", "23:40:20", "00:12:45", "00:17:19"];
    println!(
        "The mean time of [{}] is: {}",
        times.join("; "),
        mean_time_of_strings(&times).unwrap_or_else(|| "error".to_string())
    );

    // Symmetry demo: 23:00:00 and 01:00:00 average to midnight
    let t1 = parse_time("23:00:00").unwrap();
    let t2 = parse_time("01:00:00").unwrap();
    println!(
        "Mean of 23:00:00 and 01:00:00 = {}",
        format_time(mean_time(&[t1, t2]))
    );
}

/* Output:
   The mean time of [23:00:17; 23:40:20; 00:12:45; 00:17:19] is: 23:47:43
   Mean of 23:00:00 and 01:00:00 = 0:0:0
*/
