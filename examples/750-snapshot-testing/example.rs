/// 750: Snapshot Testing — expect files pattern (std-only)

use std::path::Path;
use std::fs;
use std::env;

// ── Functions under test (complex output) ──────────────────────────────────────

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

pub fn render_json_like(keys: &[&str], values: &[i64]) -> String {
    let pairs: Vec<String> = keys.iter().zip(values.iter())
        .map(|(k, v)| format!("  \"{}\": {}", k, v))
        .collect();
    format!("{{\n{}\n}}", pairs.join(",\n"))
}

// ── Snapshot infrastructure ────────────────────────────────────────────────────

/// Directory where snapshots are stored
const SNAPSHOT_DIR: &str = "tests/snapshots";

/// Update snapshots when this env var is set: `UPDATE_SNAPSHOTS=1 cargo test`
fn should_update() -> bool {
    env::var("UPDATE_SNAPSHOTS").map(|v| v == "1").unwrap_or(false)
}

fn snapshot_path(name: &str) -> std::path::PathBuf {
    Path::new(SNAPSHOT_DIR).join(format!("{}.snap", name))
}

/// Assert that `actual` matches the stored snapshot.
/// Creates the snapshot on first run; fails on mismatch unless UPDATE_SNAPSHOTS=1.
pub fn assert_snapshot(name: &str, actual: &str) {
    let path = snapshot_path(name);

    if !path.exists() || should_update() {
        fs::create_dir_all(SNAPSHOT_DIR)
            .expect("could not create snapshot dir");
        fs::write(&path, actual)
            .expect("could not write snapshot");
        eprintln!("[snapshot:{}] {}", name,
            if should_update() { "Updated" } else { "Created" });
        return;
    }

    let expected = fs::read_to_string(&path)
        .expect("could not read snapshot file");

    // Normalize line endings
    let actual_norm   = actual.replace("\r\n", "\n");
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

fn compute_diff(expected: &str, actual: &str) -> String {
    let exp_lines: Vec<&str> = expected.lines().collect();
    let act_lines: Vec<&str> = actual.lines().collect();
    let mut diff = String::new();
    let max = exp_lines.len().max(act_lines.len());
    for i in 0..max {
        match (exp_lines.get(i), act_lines.get(i)) {
            (Some(e), Some(a)) if e == a => diff.push_str(&format!("  {}\n", e)),
            (Some(e), Some(a)) => {
                diff.push_str(&format!("- {}\n", e));
                diff.push_str(&format!("+ {}\n", a));
            }
            (Some(e), None) => diff.push_str(&format!("- {}\n", e)),
            (None, Some(a)) => diff.push_str(&format!("+ {}\n", a)),
            (None, None)    => break,
        }
    }
    diff
}

fn main() {
    let data = &[("Apples", 42u32), ("Bananas", 17), ("Cherries", 99)];
    let report = render_report(data);
    println!("{}", report);

    let json = render_json_like(&["alpha", "beta", "gamma"], &[1, 2, 3]);
    println!("{}", json);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These snapshot tests create files in tests/snapshots/.
    // First run creates the snapshots; subsequent runs verify them.

    #[test]
    fn snapshot_sales_report() {
        let data = &[("Apples", 42u32), ("Bananas", 17), ("Cherries", 99)];
        let report = render_report(data);
        assert_snapshot("sales_report", &report);
    }

    #[test]
    fn snapshot_json_like() {
        let json = render_json_like(&["x", "y"], &[100, 200]);
        assert_snapshot("json_like", &json);
    }

    // Unit tests that don't use snapshots
    #[test]
    fn report_contains_header() {
        let r = render_report(&[("Item", 1)]);
        assert!(r.contains("Sales Report"), "missing header: {}", r);
    }

    #[test]
    fn report_total_items_count() {
        let data = &[("A", 1u32), ("B", 2), ("C", 3)];
        let r = render_report(data);
        assert!(r.contains("Total items: 3"), "wrong count: {}", r);
    }
}
