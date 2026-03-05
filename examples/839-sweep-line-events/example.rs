/// Sweep Line Algorithm with Event Queue.
///
/// Processes interval START/END events in sorted order.
/// Applications: max overlap depth, union length, scheduling.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EventKind {
    End = 0,   // Process END before START at same x (avoid double-counting touches)
    Start = 1,
}

#[derive(Debug, Clone, Copy)]
struct Event {
    x: f64,
    kind: EventKind,
}

fn make_events(intervals: &[(f64, f64)]) -> Vec<Event> {
    let mut events: Vec<Event> = intervals
        .iter()
        .flat_map(|&(lo, hi)| [
            Event { x: lo, kind: EventKind::Start },
            Event { x: hi, kind: EventKind::End },
        ])
        .collect();
    // Sort by x; at same x, END before START
    events.sort_by(|a, b| {
        a.x.partial_cmp(&b.x).unwrap()
            .then(a.kind.cmp(&b.kind))
    });
    events
}

/// Maximum number of intervals active simultaneously.
pub fn max_overlap(intervals: &[(f64, f64)]) -> usize {
    let events = make_events(intervals);
    let mut active = 0i64;
    let mut best = 0usize;
    for ev in &events {
        match ev.kind {
            EventKind::Start => {
                active += 1;
                best = best.max(active as usize);
            }
            EventKind::End => active -= 1,
        }
    }
    best
}

/// Total length covered by the union of intervals.
pub fn union_length(intervals: &[(f64, f64)]) -> f64 {
    let events = make_events(intervals);
    let mut active = 0i64;
    let mut total = 0.0f64;
    let mut prev_x = 0.0f64;

    for ev in &events {
        if active > 0 {
            total += ev.x - prev_x;
        }
        prev_x = ev.x;
        match ev.kind {
            EventKind::Start => active += 1,
            EventKind::End => active -= 1,
        }
    }
    total
}

/// Find all intervals that contain point x (sweep-line approach).
pub fn intervals_at(x: f64, intervals: &[(f64, f64)]) -> Vec<usize> {
    intervals.iter()
        .enumerate()
        .filter(|(_, &(lo, hi))| lo <= x && x <= hi)
        .map(|(i, _)| i)
        .collect()
}

fn main() {
    let ivs: &[(f64, f64)] = &[(1.0, 4.0), (2.0, 6.0), (3.0, 5.0), (7.0, 9.0)];
    println!("Intervals: [1,4] [2,6] [3,5] [7,9]");
    println!("Max overlap:  {} (expected 3, at [3,4])", max_overlap(ivs));
    println!("Union length: {:.1} (expected 7.0: [1,6]∪[7,9])", union_length(ivs));

    let touching: &[(f64, f64)] = &[(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)];
    println!("\nTouching intervals [0,1] [1,2] [2,3]:");
    println!("Max overlap:  {} (expected 1)", max_overlap(touching));
    println!("Union length: {:.1} (expected 3.0)", union_length(touching));

    // Meeting-room scheduling: how many rooms do we need?
    let meetings: &[(f64, f64)] = &[(0.0, 30.0), (5.0, 10.0), (15.0, 20.0)];
    println!("\nMeetings [0,30] [5,10] [15,20]: need {} rooms", max_overlap(meetings));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_overlap() {
        assert_eq!(max_overlap(&[(1.0, 4.0), (2.0, 6.0), (3.0, 5.0)]), 3);
    }

    #[test]
    fn test_no_overlap() {
        assert_eq!(max_overlap(&[(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]), 1);
    }

    #[test]
    fn test_touching_not_overlapping() {
        // Touching intervals [0,1] [1,2] [2,3] — at each point only 1 active
        assert_eq!(max_overlap(&[(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)]), 1);
    }

    #[test]
    fn test_union_length_basic() {
        let ivs: &[(f64, f64)] = &[(1.0, 4.0), (2.0, 6.0), (3.0, 5.0), (7.0, 9.0)];
        let u = union_length(ivs);
        assert!((u - 7.0).abs() < 1e-9, "union={u}");
    }

    #[test]
    fn test_union_length_disjoint() {
        let ivs: &[(f64, f64)] = &[(0.0, 1.0), (2.0, 3.0), (4.0, 5.0)];
        assert!((union_length(ivs) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_union_length_nested() {
        // [0,10] contains [2,5] — union is 10
        assert!((union_length(&[(0.0, 10.0), (2.0, 5.0)]) - 10.0).abs() < 1e-9);
    }

    #[test]
    fn test_touching_union() {
        // [0,1] [1,2] [2,3] — union is 3
        assert!((union_length(&[(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)]) - 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_meeting_rooms() {
        // [0,30] [5,10] [15,20]: max 2 overlapping at any point
        assert_eq!(max_overlap(&[(0.0, 30.0), (5.0, 10.0), (15.0, 20.0)]), 2);
    }
}
