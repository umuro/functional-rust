/// 757: Golden File Testing Pattern — std-only

use std::path::{Path, PathBuf};
use std::fs;
use std::env;

// ── Golden file infrastructure ─────────────────────────────────────────────────

const GOLDEN_DIR: &str = "tests/golden";

/// Compare `actual` against a stored golden file.
/// If `UPDATE_GOLDEN=1`, updates the file instead of comparing.
pub fn assert_golden(name: &str, actual: &str) {
    let path = PathBuf::from(GOLDEN_DIR).join(format!("{}.txt", name));
    let update = env::var("UPDATE_GOLDEN").map(|v| v == "1").unwrap_or(false);

    if !path.exists() || update {
        fs::create_dir_all(GOLDEN_DIR).expect("cannot create golden dir");
        fs::write(&path, actual).expect("cannot write golden file");
        eprintln!("[golden:{}] {}", name,
            if update { "Updated" } else { "Created" });
        return;
    }

    let expected = fs::read_to_string(&path)
        .expect("cannot read golden file");

    let actual_n   = actual.replace("\r\n", "\n");
    let expected_n = expected.replace("\r\n", "\n");

    if actual_n != expected_n {
        eprintln!("[golden:{}] MISMATCH", name);
        eprintln!("--- expected ({}) ---", path.display());
        eprintln!("{}", &expected_n);
        eprintln!("--- actual ---");
        eprintln!("{}", &actual_n);
        panic!("Golden file mismatch for '{}'. Run with UPDATE_GOLDEN=1 to update.", name);
    }
}

// ── Code under test ────────────────────────────────────────────────────────────

/// A simple expression AST
#[derive(Debug)]
pub enum Expr {
    Num(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

/// Render the AST to a human-readable string
pub fn render(expr: &Expr) -> String {
    match expr {
        Expr::Num(n)       => n.to_string(),
        Expr::Var(s)       => s.clone(),
        Expr::Add(a, b)    => format!("({} + {})", render(a), render(b)),
        Expr::Mul(a, b)    => format!("({} * {})", render(a), render(b)),
    }
}

/// Render with indentation (tree format)
pub fn render_tree(expr: &Expr, indent: usize) -> String {
    let pad = "  ".repeat(indent);
    match expr {
        Expr::Num(n)    => format!("{}Num({})\n", pad, n),
        Expr::Var(s)    => format!("{}Var({})\n", pad, s),
        Expr::Add(a, b) => format!("{}Add\n{}{}", pad,
            render_tree(a, indent + 1), render_tree(b, indent + 1)),
        Expr::Mul(a, b) => format!("{}Mul\n{}{}", pad,
            render_tree(a, indent + 1), render_tree(b, indent + 1)),
    }
}

/// Generate a simple markdown report from key-value data
pub fn render_report(title: &str, rows: &[(&str, &str)]) -> String {
    let mut out = format!("# {}\n\n", title);
    out.push_str("| Key | Value |\n");
    out.push_str("|-----|-------|\n");
    for (k, v) in rows {
        out.push_str(&format!("| {} | {} |\n", k, v));
    }
    out.push('\n');
    out
}

fn main() {
    let expr = Expr::Add(
        Box::new(Expr::Mul(Box::new(Expr::Num(2)), Box::new(Expr::Var("x".into())))),
        Box::new(Expr::Num(3)),
    );
    println!("Inline: {}", render(&expr));
    println!("Tree:\n{}", render_tree(&expr, 0));

    let report = render_report("Summary", &[
        ("Status", "OK"),
        ("Items", "42"),
        ("Errors", "0"),
    ]);
    println!("{}", report);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_expr() -> Expr {
        Expr::Add(
            Box::new(Expr::Mul(
                Box::new(Expr::Num(2)),
                Box::new(Expr::Var("x".into()))
            )),
            Box::new(Expr::Num(3)),
        )
    }

    // Golden tests — output captured to tests/golden/*.txt
    #[test]
    fn golden_inline_render() {
        let expr = make_expr();
        assert_golden("expr_inline", &render(&expr));
    }

    #[test]
    fn golden_tree_render() {
        let expr = make_expr();
        assert_golden("expr_tree", &render_tree(&expr, 0));
    }

    #[test]
    fn golden_report() {
        let report = render_report("Test Report", &[
            ("Alpha", "1"),
            ("Beta",  "2"),
            ("Gamma", "3"),
        ]);
        assert_golden("test_report", &report);
    }

    // Non-golden unit tests
    #[test]
    fn render_num() {
        assert_eq!(render(&Expr::Num(42)), "42");
    }

    #[test]
    fn render_var() {
        assert_eq!(render(&Expr::Var("x".into())), "x");
    }

    #[test]
    fn render_add() {
        let e = Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)));
        assert_eq!(render(&e), "(1 + 2)");
    }

    #[test]
    fn render_nested() {
        let e = Expr::Mul(
            Box::new(Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)))),
            Box::new(Expr::Var("y".into())),
        );
        assert_eq!(render(&e), "((1 + 2) * y)");
    }
}
