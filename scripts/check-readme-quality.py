#!/usr/bin/env python3
"""
README quality checker for functional-rust examples.

A README is "rich enough" when it gives a reader who knows neither the
algorithm nor OCaml/Rust enough context to understand:
  - WHY the concept/algorithm exists and what problem it solves
  - WHERE it might be used in real systems
  - HOW it maps between OCaml and Rust

Exit code 0 = all pass, 1 = failures found.
"""
import sys
import re
from pathlib import Path

EXAMPLES_DIR = Path(__file__).parent.parent / "examples"

# Accept either name variant for each required section
REQUIRED_SECTIONS = [
    # (canonical name, accepted regex patterns)
    ("Problem Statement",  r"##\s+Problem\s+Statement"),
    ("Learning Outcomes",  r"##\s+Learning\s+Outcomes?"),
    ("Rust section",       r"##\s+Rust\s+(?:Application|Approach|Implementation)"),
    ("OCaml section",      r"##\s+OCaml\s+(?:Approach|Implementation|Source|Mapping)"),
    ("Key Differences",    r"##\s+Key\s+Diff(?:erences?)?"),
    ("Exercises",          r"##\s+Exercises?"),
]

# Minimum body word count (enough prose to explain context)
MIN_WORDS = 150

# Lines of actual content (non-blank, not the boilerplate header, not H1)
MIN_BODY_LINES = 20

_BOILERPLATE_RE = re.compile(
    r"^\s*$|"               # blank
    r"^📖|"                 # hightechmind link
    r"^---$|"               # horizontal rule
    r"^#\s+",               # H1
    re.MULTILINE
)


def body_lines(text: str) -> list[str]:
    return [l for l in text.splitlines()
            if l.strip()
            and not l.startswith("📖")
            and not l.startswith("---")
            and not re.match(r"^#\s+", l)]


def check_readme(path: Path) -> list[str]:
    """Return list of failure reasons; empty list = pass."""
    text = path.read_text(encoding="utf-8")
    failures = []

    bl = body_lines(text)
    if len(bl) < MIN_BODY_LINES:
        failures.append(
            f"too short: {len(bl)} content lines (need {MIN_BODY_LINES})"
        )

    words = len(re.findall(r"\w+", text))
    if words < MIN_WORDS:
        failures.append(f"too few words: {words} (need {MIN_WORDS})")

    for name, pattern in REQUIRED_SECTIONS:
        if not re.search(pattern, text, re.IGNORECASE | re.MULTILINE):
            failures.append(f"missing section: {name}")

    return failures


def main():
    args = sys.argv[1:]
    if args:
        # Check specific directories passed as arguments
        examples = [EXAMPLES_DIR / a for a in args if (EXAMPLES_DIR / a).is_dir()]
    else:
        examples = sorted(EXAMPLES_DIR.iterdir())

    failures: dict[str, list[str]] = {}

    for ex in examples:
        readme = ex / "README.md"
        if not readme.exists():
            continue
        issues = check_readme(readme)
        if issues:
            failures[ex.name] = issues

    if failures:
        print(f"README quality check FAILED: {len(failures)} examples need improvement\n")
        for name, issues in sorted(failures.items()):
            print(f"  {name}:")
            for issue in issues:
                print(f"    - {issue}")
        sys.exit(1)
    else:
        total = sum(1 for ex in examples if (ex / "README.md").exists())
        print(f"README quality check PASSED: all {total} READMEs meet the standard")
        sys.exit(0)


if __name__ == "__main__":
    main()
