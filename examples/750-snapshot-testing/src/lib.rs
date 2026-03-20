#![allow(clippy::all)]
//! # Snapshot Testing
//!
//! Expect files pattern for testing complex output (std-only).

use std::env;
use std::fs;
use std::path::Path;

/// Render a sales report
pub fn render_report(data: &[(&str, u32)]) -> String {
    let mut out = String::new();
    out.push_str("=== Sales Report ===\n");
    for (i, (name, qty)) in data.iter().enumerate() {
        out.push_str(&format!("{:3}. {:<20} {}\n", i + 1, name, qty));
    }
    out.push_str("====================\n");
    let total: u32 = data.iter().map(|(_, q)| q).sum();
    out.push_str(&format!("Total items: {}\n", data.len()));
    out.push_str(&format!("Total qty:   {}\n", total));
    out
}

/// Render a JSON-like structure
pub fn render_json_like(keys: &[&str], values: &[i64]) -> String {
    let pairs: Vec<String> = keys
        .iter()
        .zip(values.iter())
        .map(|(k, v)| format!("  \"{}\": {}", k, v))
        .collect();
    format!("{{\n{}\n}}", pairs.join(",\n"))
}

/// Snapshot directory
const SNAPSHOT_DIR: &str = "tests/snapshots";

/// Check if we should update snapshots
pub fn should_update() -> bool {
    env::var("UPDATE_SNAPSHOTS")
        .map(|v| v == "1")
        .unwrap_or(false)
}

/// Get the path to a snapshot file
pub fn snapshot_path(name: &str) -> std::path::PathBuf {
    Path::new(SNAPSHOT_DIR).join(format!("{}.snap", name))
}

/// Assert that `actual` matches the stored snapshot.
///
/// Creates the snapshot on first run; fails on mismatch unless UPDATE_SNAPSHOTS=1.
pub fn assert_snapshot(name: &str, actual: &str) {
    let path = snapshot_path(name);

    if !path.exists() || should_update() {
        fs::create_dir_all(SNAPSHOT_DIR).expect("could not create snapshot dir");
        fs::write(&path, actual).expect("could not write snapshot");
        return;
    }

    let expected = fs::read_to_string(&path).expect("could not read snapshot file");

    let actual_norm = actual.replace("\r\n", "\n");
    let expected_norm = expected.replace("\r\n", "\n");

    if actual_norm != expected_norm {
        let diff = compute_diff(&expected_norm, &actual_norm);
        panic!(
            "Snapshot '{}' mismatch!\n\
             To update: UPDATE_SNAPSHOTS=1 cargo test\n\n\
             Diff:\n{}",
            name, diff
        );
    }
}

/// Compute a simple line-by-line diff
pub fn compute_diff(expected: &str, actual: &str) -> String {
    let exp_lines: Vec<&str> = expected.lines().collect();
    let act_lines: Vec<&str> = actual.lines().collect();
    let mut diff = String::new();
    let max = exp_lines.len().max(act_lines.len());
    for i in 0..max {
        let e = exp_lines.get(i).unwrap_or(&"");
        let a = act_lines.get(i).unwrap_or(&"");
        if e != a {
            diff.push_str(&format!("- {}\n+ {}\n", e, a));
        }
    }
    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_report_empty() {
        let report = render_report(&[]);
        assert!(report.contains("=== Sales Report ==="));
        assert!(report.contains("Total items: 0"));
    }

    #[test]
    fn test_render_report_with_data() {
        let data = [("Widget", 10), ("Gadget", 20)];
        let report = render_report(&data);
        assert!(report.contains("Widget"));
        assert!(report.contains("Gadget"));
        assert!(report.contains("Total qty:   30"));
    }

    #[test]
    fn test_render_json_like() {
        let output = render_json_like(&["a", "b"], &[1, 2]);
        assert!(output.contains("\"a\": 1"));
        assert!(output.contains("\"b\": 2"));
    }

    #[test]
    fn test_compute_diff_identical() {
        let diff = compute_diff("hello\nworld", "hello\nworld");
        assert!(diff.is_empty());
    }

    #[test]
    fn test_compute_diff_different() {
        let diff = compute_diff("hello", "world");
        assert!(diff.contains("- hello"));
        assert!(diff.contains("+ world"));
    }

    #[test]
    fn test_snapshot_path() {
        let path = snapshot_path("my_test");
        assert!(path.ends_with("my_test.snap"));
    }
}
