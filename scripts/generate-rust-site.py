#!/usr/bin/env python3
"""
Functional Rust site generator — fully featured version.
Reads example dirs, generates HTML pages with theme toggle, nav pages, full SEO.

## Deployment layout (DO NOT CHANGE PATH LOGIC)

All output goes to /tmp/rust-site/rust/ (both index + examples).
They are deployed together to the server at:
  public_html/rust/index.html
  public_html/rust/001-last-element.html
  ... etc

Because index.html and all example pages live in the SAME directory (rust/),
ALL internal hrefs must be relative with NO "rust/" prefix:
  OK  href="index.html"
  OK  href="001-last-element.html"
  BAD href="rust/index.html"

Deploy command:
  scp -P 65002 -i /home/node/.ssh/id_ed25519 -o StrictHostKeyChecking=no \\
    /tmp/rust-site/rust/*.html \\
    u508071997@185.224.137.204:~/domains/hightechmind.io/public_html/rust/

## Preview locally:
  python3 -m http.server 8080 --directory /tmp/rust-site/rust/
  open http://localhost:8080/index.html
"""
import os, re, sys, shutil
from pathlib import Path
from datetime import datetime

# ---- Paths ----
# Canonical source: git-tracked repo (github.com/umuro/functional-rust)
EXAMPLES_DIR = Path("/home/umur/.openclaw/workspace/examples")
if not EXAMPLES_DIR.exists():
    for candidate in [
        Path("/home/node/.openclaw/workspace/examples"),
        Path("/home/umur/workspaces/hightechmind2024/functional-rust/examples"),
        Path("/work/hightechmind2024/functional-rust/examples"),
    ]:
        if candidate.exists():
            EXAMPLES_DIR = candidate
            break

OUTPUT_DIR = Path("/tmp/rust-site")
RUST_DIR = OUTPUT_DIR / "rust"
SITE_BASE_URL = "https://hightechmind.io/rust"
SITE_OG_IMAGE = "https://hightechmind.io/functional-rust/og-image.png"

OUTPUT_DIR.mkdir(exist_ok=True)
RUST_DIR.mkdir(exist_ok=True)

# ---- Level config: (bg_classes, text_classes, label) ----
LEVELS = {
    "fundamental":  ("bg-green-100 dark:bg-green-900/30",   "text-green-800 dark:text-green-300",   "Fundamental"),
    "intermediate": ("bg-blue-100 dark:bg-blue-900/30",     "text-blue-800 dark:text-blue-300",     "Intermediate"),
    "advanced":     ("bg-purple-100 dark:bg-purple-900/30", "text-purple-800 dark:text-purple-300", "Advanced"),
    "expert":       ("bg-red-100 dark:bg-red-900/30",       "text-red-800 dark:text-red-300",       "Expert"),
}

# ---- Topic taxonomy (slug-based classification for Browse by Topic) ----
# Ordered: earlier entries take priority on ambiguous slugs.
# Each entry: (id, display_label, [slug substrings that map to this topic])
TOPICS = [
    ("iterators",        "Iterators",
     ["iterator", "scan-", "unfold", "zip-", "flatten-iter", "lazy-seq", "lazy-fib",
      "double-ended", "exact-size", "try-fold", "fold-left", "fold-right", "fold-optic",
      "step-by", "group-by-iter", "collect-", "peekable", "take-while", "skip-while"]),
    ("pattern-matching", "Pattern Matching",
     ["pattern-", "destructur", "variant-", "variants-"]),
    ("closures-hof",     "Closures & Higher-Order Functions",
     ["closure-", "higher-order", "currying", "partial-app", "function-comp",
      "function-pointer", "pipeline-op", "applying-a-function", "twice",
      "higher-order-fn", "function-composition"]),
    ("error-handling",   "Error Handling",
     ["error-", "result-", "option-", "validated-", "try-operator", "try-trait",
      "parse-error", "parse-dont", "parse-int-safe", "railway", "custom-error",
      "panic-", "multiple-error", "collecting-result", "recover-from",
      "validation-error", "partition-result", "error-handling"]),
    ("strings-parsing",  "Strings & Parsing",
     ["string-", "-string", "str-", "anagram", "palindrome", "isogram", "pangram",
      "run-length", "json-", "csv-", "parser-", "recursive-descent", "map-parser",
      "word-count", "word-break", "frequency-anal", "phone-number", "nucleotide",
      "caesar", "hamming-dist", "reverse-string", "-rle", "rle-",
      "encode-", "decode-", "modified-rle", "decode-rle", "direct-rle",
      "edit-distance", "balanced-parentheses", "balanced-paren",
      "arithmetic-parser", "sequence-parser", "choice-parser",
      "binary-format", "binary-decimal"]),
    ("trees",            "Trees",
     ["-tree", "tree-", "red-black", "binary-heap", "avl-", "trie",
      "map-fold-tree", "balanced-insert", "leaves", "internal-node",
      "complete-tree", "expression-tree", "zipper", "at-level"]),
    ("graphs-algorithms","Graphs & Algorithms",
     ["graph-", "dijkstra", "topological-sort", "shortest-path", "huffman",
      "greedy-", "adjacency-", "merge-sort", "quicksort", "sort-by",
      "insertion-sort", "bubble-sort", "hamiltonian", "minimum-path",
      "minimum-vertex", "modular-arith", "modular-exp"]),
    ("data-structures",  "Data Structures",
     ["list-", "reverse-list", "difference-list", "flatten-nested",
      "stack-", "queue-", "priority-queue", "circular-buffer",
      "hashmap-", "btreemap", "btreeset", "persistent-", "small-vec", "vec-",
      "matrix-", "array-", "deque-", "rope-", "skip-list", "arena-",
      "multimap", "bitset", "lru-cache", "bloom-filter",
      "last-element", "last-two", "kth-element", "eliminate-dup", "pack-consec",
      "replicate-", "drop-every", "split-list", "slice-list", "rotate-left",
      "remove-kth", "insert-at", "lotto-", "random-select", "random-perm",
      "combinations", "group-by-size", "accumulate", "association-list",
      "duplicate-elem", "windows-", "collect-leaves", "count-leaves",
      "internal-nodes", "heterogeneous-"]),
    ("recursion-dp",     "Recursion & Dynamic Programming",
     ["tail-recursive", "fibonacci-", "memoiz", "knapsack",
      "longest-common", "longest-incr", "partition-equal",
      "church-numer", "church-encod", "catamorphism",
      "-dp", "dp-", "dynamic-prog", "collatz", "sieve-",
      "gcd-", "gcd-lcm", "difference-of-sq", "recursive-type",
      "trampoline"]),
    ("traits-types",     "Traits & Type System",
     ["trait-", "type-level", "type-alias", "phantom-", "associated-type",
      "newtype-", "typestate-", "impl-trait", "from-into", "display-",
      "clone-copy", "gadt-", "type-safe", "type-witness", "where-clause",
      "lambda-calc", "never-type", "zero-cost",
      "deref-", "send-sync", "existential-", "type-equality", "type-erasure",
      "gat-", "coherence-", "default-trait", "default-method",
      "product-type", "records", "state-machine"]),
    ("generics-macros",  "Generics & Macros",
     ["macro-", "proc-macro", "const-generic", "const-fn", "const-",
      "generic-", "higher-kinded-sim", "higher-kinded-type", "serde-"]),
    ("lifetimes-memory", "Lifetimes & Memory Safety",
     ["lifetime-", "borrowing-", "borrow-", "cow-", "move-",
      "unsafe-", "ffi-", "zero-copy", "memory-", "arc-", "mutex-", "rwlock-",
      "drop-", "arena-alloc", "small-vec-pat", "raw-pointer", "transmute",
      "once-cell", "lock-free-"]),
    ("async-concurrency","Async & Concurrency",
     ["async-", "thread-", "channel-", "work-steal", "work-queue",
      "pipeline-concurr", "pipeline-stage", "scoped-thread", "pool-pattern",
      "future-", "select-", "timeout-", "retry-", "semaphore-", "buffered-stream",
      "mpsc-", "cancellation-", "producer-consumer", "rayon-", "crossbeam-",
      "barrier-", "actor-", "concurrent-", "parallel-", "reactive-",
      "event-loop", "backpressure", "lock-free-", "once-init"]),
    ("fp-abstractions",  "FP Abstractions",
     ["monad-", "writer-monad", "state-monad", "functor-", "applicative-",
      "free-monad", "monoid-", "effect-", "lens", "prism-", "profunctor-",
      "optic", "continuation-", "comonad", "bifunctor", "kleisli",
      "adjunction", "yoneda", "coyoneda", "tambara", "higher-order-fn",
      "traversal", "affine-", "iso-", "morphism", "natural-transf",
      "scott-encod", "sequence-monadic", "traverse-", "choice-profunctor"]),
    ("testing",          "Testing & Code Quality",
     ["test-fixture", "test-double", "test-isolation", "test-prop",
      "property-test", "fuzzing", "doctest"]),
    ("other",            "Other",
     []),  # catch-all — remaining examples
]

def classify_topic(slug):
    """Classify an example by its directory slug into a topic group."""
    s = slug.lower()
    for tid, _label, keywords in TOPICS:
        if tid == "other":
            return "other"
        for kw in keywords:
            if kw in s:
                return tid
    return "other"

# ---- Learning paths ----
LEARNING_PATHS = [
    {
        "id": "foundations",
        "title": "Functional Foundations",
        "icon": "🏗️",
        "description": "Core FP concepts: higher-order functions, closures, function composition, and currying.",
        "keywords": ["higher-order", "function", "composition", "currying", "closure", "lambda", "apply", "pipeline", "twice", "applying"],
    },
    {
        "id": "pattern-matching",
        "title": "Pattern Matching & Types",
        "icon": "🔍",
        "description": "Algebraic data types, sum types, and pattern-based destructuring in Rust.",
        "keywords": ["pattern", "matching", "enum", "algebraic", "variant", "destructure", "option", "sum", "type"],
    },
    {
        "id": "collections",
        "title": "Data Structures & Collections",
        "icon": "📦",
        "description": "Lists, trees, maps, and other functional data structures.",
        "keywords": ["list", "tree", "map", "stack", "queue", "deque", "linked", "binary", "array", "vector", "kth", "last", "length", "element", "operations"],
    },
    {
        "id": "iterators",
        "title": "Iterators & Transformation",
        "icon": "🔄",
        "description": "Map, filter, fold, and lazy iteration patterns from OCaml to Rust.",
        "keywords": ["iterator", "fold", "reduce", "filter", "zip", "flatten", "scan", "sequence", "map"],
    },
    {
        "id": "error-handling",
        "title": "Error Handling FP Style",
        "icon": "🛡️",
        "description": "Result, Option, and monadic error composition.",
        "keywords": ["error", "result", "option", "monad", "combinator", "either", "exception", "failure", "safe"],
    },
    {
        "id": "recursion",
        "title": "Recursion & Induction",
        "icon": "♾️",
        "description": "Recursive patterns, tail-call optimization, and structural induction.",
        "keywords": ["recursion", "recursive", "tail", "fibonacci", "factorial", "accumulator", "divide", "merge", "sort", "palindrome"],
    },
]

# ---- Theme & JS snippets (plain strings, NOT f-strings) ----

THEME_JS = """<script>
(function() {
  var t = localStorage.getItem('fr-theme') || 'auto';
  if (t === 'dark' || (t === 'auto' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
    document.documentElement.classList.add('dark');
  }
})();
</script>"""

THEME_TOGGLE_HTML = """<div class="flex items-center gap-0.5 bg-gray-100 dark:bg-gray-700 rounded-full px-1 py-1" id="theme-widget">
  <button class="theme-btn px-2 py-1 rounded-full text-xs transition-all hidden sm:inline-flex" data-theme="auto" title="Auto">🖥</button>
  <button class="theme-btn px-2 py-1 rounded-full text-xs transition-all hidden sm:inline-flex" data-theme="light" title="Light">☀️</button>
  <button class="theme-btn px-2 py-1 rounded-full text-xs transition-all hidden sm:inline-flex" data-theme="dark" title="Dark">🌙</button>
  <button id="theme-cycle" class="px-2 py-1 rounded-full text-sm transition-all sm:hidden" title="Toggle theme" aria-label="Toggle colour theme">🖥</button>
</div>"""

THEME_INIT_JS = """<script>
(function() {
  function applyTheme(t) {
    if (t === 'dark' || (t === 'auto' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
    document.querySelectorAll('.theme-btn').forEach(function(btn) {
      var active = btn.dataset.theme === t;
      btn.classList.toggle('bg-white', active);
      btn.classList.toggle('dark:bg-gray-600', active);
      btn.classList.toggle('shadow-sm', active);
      btn.classList.toggle('font-semibold', active);
    });
    localStorage.setItem('fr-theme', t);
  }
  var ICONS = {auto: '🖥', light: '☀️', dark: '🌙'};
  var ORDER = ['auto', 'light', 'dark'];
  var saved = localStorage.getItem('fr-theme') || 'auto';
  applyTheme(saved);
  document.addEventListener('DOMContentLoaded', function() {
    applyTheme(saved);
    document.querySelectorAll('.theme-btn').forEach(function(btn) {
      btn.addEventListener('click', function() { applyTheme(this.dataset.theme); });
    });
    // Mobile cycle button: shows current icon, cycles on tap
    var cycleBtn = document.getElementById('theme-cycle');
    if (cycleBtn) {
      function updateCycleIcon(t) { cycleBtn.textContent = ICONS[t] || '🖥'; }
      updateCycleIcon(saved);
      cycleBtn.addEventListener('click', function() {
        var cur = localStorage.getItem('fr-theme') || 'auto';
        var next = ORDER[(ORDER.indexOf(cur) + 1) % ORDER.length];
        applyTheme(next);
        updateCycleIcon(next);
      });
    }
  });
})();
</script>"""

SEARCH_JS = """<script>
document.addEventListener('DOMContentLoaded', function() {
  var searchEl = document.getElementById('search');
  if (!searchEl) return;
  var activeLevel = 'all';

  function filterCards() {
    var q = searchEl.value.toLowerCase().trim();
    var cards = document.querySelectorAll('.example-card');
    var count = 0;
    cards.forEach(function(card) {
      var textMatch = !q || card.textContent.toLowerCase().includes(q);
      var levelMatch = activeLevel === 'all' || card.dataset.level === activeLevel;
      var show = textMatch && levelMatch;
      card.style.display = show ? '' : 'none';
      if (show) count++;
    });
    var counter = document.getElementById('result-count');
    if (counter) counter.textContent = (q || activeLevel !== 'all') ? count + ' result' + (count !== 1 ? 's' : '') : '';
  }

  searchEl.addEventListener('input', filterCards);

  document.querySelectorAll('.level-filter').forEach(function(btn) {
    btn.addEventListener('click', function() {
      activeLevel = this.dataset.level;
      document.querySelectorAll('.level-filter').forEach(function(b) {
        b.classList.remove('bg-orange-500', 'text-white');
        b.classList.add('bg-gray-100', 'dark:bg-gray-700', 'text-gray-700', 'dark:text-gray-300');
      });
      this.classList.add('bg-orange-500', 'text-white');
      this.classList.remove('bg-gray-100', 'dark:bg-gray-700', 'text-gray-700', 'dark:text-gray-300');
      filterCards();
    });
  });

  var allBtn = document.querySelector('.level-filter[data-level="all"]');
  if (allBtn) {
    allBtn.classList.add('bg-orange-500', 'text-white');
    allBtn.classList.remove('bg-gray-100', 'dark:bg-gray-700', 'text-gray-700', 'dark:text-gray-300');
  }
});
</script>"""

FILTER_JS = """<script>
(function() {
  function initFilter(wrap) {
    var btns = Array.from(wrap.querySelectorAll('[data-fv]'));
    var secs = Array.from(wrap.querySelectorAll('[data-fs]'));
    function activate(val) {
      btns.forEach(function(b) {
        var on = b.dataset.fv === val;
        b.classList.toggle('!bg-orange-500', on);
        b.classList.toggle('!text-white', on);
        b.classList.toggle('!border-orange-500', on);
        b.setAttribute('aria-pressed', on ? 'true' : 'false');
      });
      secs.forEach(function(s) {
        s.style.display = (val === 'all' || s.dataset.fs === val) ? '' : 'none';
      });
    }
    btns.forEach(function(btn) {
      btn.addEventListener('click', function() { activate(this.dataset.fv); });
    });
    activate('all');
  }
  document.querySelectorAll('[data-filter-wrap]').forEach(initFilter);
})();
</script>"""

TAB_JS = """<script>
(function() {
  document.querySelectorAll('[data-tab-wrap]').forEach(function(wrap) {
    var btns = wrap.querySelectorAll('[data-tab-btn]');
    var panels = wrap.querySelectorAll('[data-tab-panel]');
    btns.forEach(function(btn) {
      btn.addEventListener('click', function() {
        var t = this.dataset.tabBtn;
        btns.forEach(function(b) {
          var on = b.dataset.tabBtn === t;
          b.classList.toggle('bg-gray-800', on);
          b.classList.toggle('text-white', on);
          b.classList.toggle('text-gray-400', !on);
          b.classList.toggle('hover:text-white', !on);
        });
        panels.forEach(function(p) {
          p.style.display = p.dataset.tabPanel === t ? '' : 'none';
        });
      });
    });
  });
})();
</script>"""


# ---- Utilities ----

def read_file(path):
    try:
        return path.read_text(encoding="utf-8")
    except Exception:
        return ""

def read_rust_source(example_dir):
    for p in [example_dir / "src" / "lib.rs", example_dir / "src" / "main.rs", example_dir / "example.rs"]:
        if p.exists():
            return p.read_text(encoding="utf-8")
    return ""

def escape_html(s):
    return s.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")

def escape_json(s):
    return s.replace("\\", "\\\\").replace('"', '\\"').replace("\n", " ").replace("\r", " ").replace("\t", " ")

def get_example_number(dirname):
    m = re.match(r"^(\d+)", dirname)
    return m.group(1).zfill(3) if m else "000"


# ---- Metadata extraction ----

def extract_title(readme):
    m = re.search(r"^#\s+Example\s+\d+[^:—\-]*[:\-—]\s*(.+)", readme, re.MULTILINE)
    if m:
        return m.group(1).strip()
    m = re.search(r"^#\s+(.+)", readme, re.MULTILINE)
    if m:
        return m.group(1).strip()
    return ""

def extract_difficulty(readme):
    """Count star emoji symbols to determine level."""
    m = re.search(r"\*\*Difficulty:\*\*\s*([⭐🌟]+)", readme)
    if m:
        count = m.group(1).count("⭐") + m.group(1).count("🌟")
        if count <= 1:   return "fundamental"
        elif count == 2: return "intermediate"
        elif count == 3: return "advanced"
        else:            return "expert"
    # Text fallback
    m = re.search(r"\*\*Difficulty:\*\*\s*(.+)", readme)
    if m:
        d = m.group(1).strip().lower()
        for level in LEVELS:
            if level in d:
                return level
        if "beginner" in d:
            return "fundamental"
    return "fundamental"

def extract_concepts(readme):
    """Return list of concepts from **Category:** field."""
    m = re.search(r"\*\*Category:\*\*\s*(.+)", readme)
    if m:
        raw = m.group(1).strip().strip("[]")
        parts = [c.strip() for c in re.split(r"[|&]", raw)]
        return [p for p in parts if p]
    return ["Functional Programming"]

def extract_video_url(readme):
    m = re.search(r"\*\*Video:\*\*\s*(https?://\S+)", readme)
    return m.group(1).strip() if m else None

def extract_section(md, *headings):
    """Return content under the first matching ## heading."""
    for heading in headings:
        pattern = rf"(?:^|\n)##\s+{re.escape(heading)}\s*\n(.*?)(?=\n##\s|\Z)"
        m = re.search(pattern, md, re.DOTALL | re.IGNORECASE)
        if m:
            return m.group(1).strip()
    return ""

def classify_learning_path(concepts, title, slug):
    text = " ".join(concepts).lower() + " " + title.lower() + " " + slug.replace("-", " ")
    scores = {p["id"]: sum(1 for kw in p["keywords"] if kw in text) for p in LEARNING_PATHS}
    best = max(scores, key=scores.get)
    return best if scores[best] > 0 else "foundations"

def parse_comparison(comparison_md):
    blocks = re.findall(r"```(\w+)\n(.*?)```", comparison_md, re.DOTALL)
    return (
        [(lang, code) for lang, code in blocks if lang == "ocaml"],
        [(lang, code) for lang, code in blocks if lang == "rust"],
    )

def extract_tests(rs_code):
    m = re.search(r"(#\[cfg\(test\)\][\s\S]*)", rs_code)
    return m.group(1).strip() if m else ""


# ---- Markdown → HTML ----

def md_to_html(md):
    if not md:
        return ""
    html = md
    html = re.sub(r"```(\w+)\n(.*?)```",
        lambda m: f'<pre><code class="language-{m.group(1)}">{escape_html(m.group(2))}</code></pre>',
        html, flags=re.DOTALL)
    html = re.sub(r"```\n?(.*?)```",
        lambda m: f'<pre><code class="language-text">{escape_html(m.group(1))}</code></pre>',
        html, flags=re.DOTALL)
    html = re.sub(r"`([^`]+)`", r'<code class="bg-gray-100 dark:bg-gray-700 px-1 rounded text-sm font-mono">\1</code>', html)
    html = re.sub(r"\*\*(.+?)\*\*", r"<strong>\1</strong>", html)
    html = re.sub(r"\*(.+?)\*",     r"<em>\1</em>",         html)
    html = re.sub(r"^####\s+(.+)$", r'<h5 class="text-base font-semibold mt-4 mb-1">\1</h5>', html, flags=re.MULTILINE)
    html = re.sub(r"^###\s+(.+)$",  r'<h4 class="text-lg font-semibold mt-5 mb-2">\1</h4>',  html, flags=re.MULTILINE)
    html = re.sub(r"^##\s+(.+)$",   r'<h3 class="text-xl font-bold mt-6 mb-3">\1</h3>',      html, flags=re.MULTILINE)
    html = re.sub(r"^#\s+(.+)$",    r'<h2 class="text-2xl font-bold mt-8 mb-4">\1</h2>',     html, flags=re.MULTILINE)
    html = re.sub(r"^---+$", '<hr class="my-6 border-gray-200 dark:border-gray-700">', html, flags=re.MULTILINE)
    # Tables
    lines, result, in_table = html.split("\n"), [], False
    for line in lines:
        if re.match(r"\|\s*[-:]+\s*\|", line):
            in_table = True
            continue
        if line.startswith("|") and not in_table:
            cells = [c.strip() for c in line.strip("|").split("|")]
            row = "".join(f'<th class="px-3 py-2 border border-gray-200 dark:border-gray-700 bg-gray-50 dark:bg-gray-800 text-sm font-medium">{c}</th>' for c in cells)
            result.append(f"<thead><tr>{row}</tr></thead><tbody>")
            in_table = True
        elif line.startswith("|") and in_table:
            cells = [c.strip() for c in line.strip("|").split("|")]
            row = "".join(f'<td class="px-3 py-2 border border-gray-200 dark:border-gray-700 text-sm">{c}</td>' for c in cells)
            result.append(f"<tr>{row}</tr>")
        else:
            if in_table:
                result.append("</tbody></table>")
                in_table = False
            result.append(line)
    if in_table:
        result.append("</tbody></table>")
    html = "\n".join(result)
    html = re.sub(r"<thead>", '<table class="w-full border-collapse my-4"><thead>', html)
    # Lists
    html = re.sub(r"^[-*]\s+(.+)$",  r'<li class="ml-4 mb-1">• \1</li>',              html, flags=re.MULTILINE)
    html = re.sub(r"^\d+\.\s+(.+)$", r'<li class="ml-4 mb-1 list-decimal">\1</li>',   html, flags=re.MULTILINE)
    # Paragraphs
    html = re.sub(r"\n\n+", '</p><p class="mb-4">', html)
    html = f'<p class="mb-4">{html}</p>'
    for tag in ["<pre>", "</pre>", "<h2", "<h3", "<h4", "<h5", "<table", "</table>", "<hr", "<thead", "<tbody", "</tbody>"]:
        html = html.replace(f'<p class="mb-4">{tag}', tag)
        html = html.replace(f"{tag}</p>", tag)
    return html


# ---- Component builders ----

def level_badge(level):
    cfg = LEVELS.get(level, LEVELS["fundamental"])
    return f'<span class="px-3 py-1 rounded-full text-xs font-semibold {cfg[0]} {cfg[1]}">{cfg[2]}</span>'

def concepts_html(concepts, max_tags=8):
    return "".join(
        f'<span class="px-2 py-1 bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 rounded-full text-xs font-medium">{escape_html(c)}</span>'
        for c in concepts[:max_tags]
    )

def example_card(ex, show_path=False):
    href     = ex["href"]
    num      = ex["num"]
    title    = ex["title"] or "(untitled)"
    level    = ex["level"]
    concepts = ex["concepts"]
    path_id  = ex.get("path_id", "")

    badge = level_badge(level)
    tags  = concepts_html(concepts, max_tags=4)

    path_label = ""
    if show_path and path_id:
        info = next((p for p in LEARNING_PATHS if p["id"] == path_id), None)
        if info:
            path_label = f'<span class="text-xs text-gray-400 dark:text-gray-500">{info["icon"]} {escape_html(info["title"])}</span>'

    return (
        f'\n      <a href="{href}" data-level="{level}"'
        f' class="example-card bg-white dark:bg-gray-800 rounded-xl border border-gray-200 dark:border-gray-700'
        f' hover:shadow-md hover:border-orange-400 dark:hover:border-orange-500 transition-all p-5 block">'
        f'\n        <div class="flex items-center justify-between mb-3">'
        f'\n          <div class="flex items-center gap-2">'
        f'<span class="font-mono text-xs text-gray-400 dark:text-gray-500">{num}</span>'
        f'{badge}</div>{path_label}</div>'
        f'\n        <h3 class="text-sm font-semibold text-gray-900 dark:text-white mb-2 leading-snug">{escape_html(title)}</h3>'
        f'\n        <div class="flex flex-wrap gap-1">{tags}</div>'
        f'\n      </a>'
    )

def repo_cards_html(dirname=None):
    fr_url = (
        f"https://github.com/umurozkul/functional-rust/tree/main/examples/{dirname}"
        if dirname else "https://github.com/umurozkul/functional-rust"
    )
    fr_desc = (
        f"View the source for this example on GitHub — OCaml and Rust side by side in the repo."
        if dirname else "1000+ OCaml → Rust functional programming examples with side-by-side comparisons."
    )
    repos = [
        {"name": "functional-rust", "url": fr_url,
         "desc": fr_desc,
         "lang": "Rust", "color": "#dea584"},
        {"name": "openclaw", "url": "https://github.com/openclaw/openclaw",
         "desc": "AI agent gateway — connect Claude, GPT-4, and others to any messaging platform.",
         "lang": "TypeScript", "color": "#3178c6"},
    ]
    gh = ('<svg class="w-4 h-4 text-gray-500" fill="currentColor" viewBox="0 0 24 24">'
          '<path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57'
          ' 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695'
          '-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99'
          '.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225'
          '-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405'
          'c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225'
          ' 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3'
          ' 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/></svg>')
    ext = ('<svg class="w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">'
           '<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"'
           ' d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"/></svg>')
    cards = "\n".join(
        f'<a href="{r["url"]}" target="_blank" rel="noopener"'
        f' class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-5 hover:shadow-md transition-shadow block">'
        f'<div class="flex items-start justify-between mb-3"><div class="flex items-center gap-2">{gh}'
        f'<span class="font-semibold text-gray-900 dark:text-white text-sm">{r["name"]}</span></div>{ext}</div>'
        f'<p class="text-sm text-gray-600 dark:text-gray-300 mb-3">{r["desc"]}</p>'
        f'<div class="flex items-center gap-1.5"><span class="w-3 h-3 rounded-full inline-block" style="background-color:{r["color"]}"></span>'
        f'<span class="text-xs text-gray-500 dark:text-gray-400">{r["lang"]}</span></div></a>'
        for r in repos
    )
    return (
        '\n  <section class="mb-12">'
        '\n    <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">Open Source Repos</h2>'
        f'\n    <div class="grid md:grid-cols-2 gap-4">{cards}</div>'
        '\n  </section>'
    )


# ---- SEO ----

def meta_tags(title, description, url, keywords=None, og_type="website"):
    kw = keywords or "rust, ocaml, functional programming, pattern matching, algebraic data types, FP, systems programming"
    return (
        f'  <meta name="description" content="{escape_html(description)}">\n'
        f'  <meta name="keywords" content="{escape_html(kw)}">\n'
        f'  <meta name="author" content="Umur Ozkul, High Tech Mind">\n'
        f'  <meta name="robots" content="index, follow">\n'
        f'  <link rel="canonical" href="{url}">\n'
        f'  <meta property="og:type" content="{og_type}">\n'
        f'  <meta property="og:site_name" content="Functional Rust">\n'
        f'  <meta property="og:url" content="{url}">\n'
        f'  <meta property="og:title" content="{escape_html(title)} | Functional Rust">\n'
        f'  <meta property="og:description" content="{escape_html(description)}">\n'
        f'  <meta property="og:image" content="{SITE_OG_IMAGE}">\n'
        f'  <meta property="og:image:width" content="1200">\n'
        f'  <meta property="og:image:height" content="630">\n'
        f'  <meta name="twitter:card" content="summary_large_image">\n'
        f'  <meta name="twitter:site" content="@hightechmindio">\n'
        f'  <meta name="twitter:url" content="{url}">\n'
        f'  <meta name="twitter:title" content="{escape_html(title)} | Functional Rust">\n'
        f'  <meta name="twitter:description" content="{escape_html(description)}">\n'
        f'  <meta name="twitter:image" content="{SITE_OG_IMAGE}">'
    )

def jsonld_website(name, description, url):
    return (
        '  <script type="application/ld+json">\n'
        '  {\n'
        f'    "@context": "https://schema.org",\n'
        f'    "@type": "WebSite",\n'
        f'    "name": "{escape_json(name)}",\n'
        f'    "description": "{escape_json(description)}",\n'
        f'    "url": "{url}",\n'
        f'    "potentialAction": {{"@type": "SearchAction", "target": "{url}?q={{search_term_string}}", "query-input": "required name=search_term_string"}},\n'
        f'    "author": {{"@type": "Person", "name": "Umur Ozkul", "url": "https://hightechmind.io"}}\n'
        '  }\n'
        '  </script>'
    )

def jsonld_article(title, description, url, keywords=None, num=None):
    kw_json = escape_json(keywords) if keywords else "rust, functional programming, ocaml, tutorial"
    teaches = f', "teaches": "{escape_json(title)}"' if title else ""
    num_str = f', "position": {num}' if num else ""
    return (
        '  <script type="application/ld+json">\n'
        '  {\n'
        f'    "@context": "https://schema.org",\n'
        f'    "@type": ["Article", "LearningResource"],\n'
        f'    "headline": "{escape_json(title)}",\n'
        f'    "description": "{escape_json(description)}",\n'
        f'    "url": "{url}",\n'
        f'    "keywords": "{kw_json}",\n'
        f'    "educationalUse": "instruction",\n'
        f'    "learningResourceType": "tutorial"{teaches}{num_str},\n'
        f'    "inLanguage": "en",\n'
        f'    "isPartOf": {{"@type": "Course", "name": "Functional Rust", "url": "{SITE_BASE_URL}/index.html"}},\n'
        f'    "author": {{"@type": "Person", "name": "Umur Ozkul", "url": "https://hightechmind.io"}},\n'
        f'    "publisher": {{"@type": "Organization", "name": "High Tech Mind B.V.", "url": "https://hightechmind.io"}}\n'
        '  }\n'
        '  </script>'
    )

def jsonld_collection(name, description, url, item_urls):
    items = ",\n".join(
        f'    {{"@type": "ListItem", "position": {i+1}, "url": "{u}"}}'
        for i, u in enumerate(item_urls[:50])  # cap at 50 for inline JSON size
    )
    return (
        '  <script type="application/ld+json">\n'
        '  {\n'
        f'    "@context": "https://schema.org",\n'
        f'    "@type": "ItemList",\n'
        f'    "name": "{escape_json(name)}",\n'
        f'    "description": "{escape_json(description)}",\n'
        f'    "url": "{url}",\n'
        f'    "numberOfItems": {len(item_urls)},\n'
        f'    "itemListElement": [\n{items}\n    ]\n'
        '  }\n'
        '  </script>'
    )

def jsonld_breadcrumb(crumbs):
    """crumbs: list of (name, url) tuples"""
    items = ",\n".join(
        f'    {{"@type": "ListItem", "position": {i+1}, "name": "{escape_json(n)}", "item": "{u}"}}'
        for i, (n, u) in enumerate(crumbs)
    )
    return (
        '  <script type="application/ld+json">\n'
        '  {\n'
        '    "@context": "https://schema.org",\n'
        '    "@type": "BreadcrumbList",\n'
        f'    "itemListElement": [\n{items}\n    ]\n'
        '  }\n'
        '  </script>'
    )


# ---- Layout ----

def nav_html(active=""):
    items = [
        ("Examples",       "index.html",            "examples"),
        ("By Level",       "by-level.html",         "by-level"),
        ("By Topic",       "by-topic.html",         "by-topic"),
        ("Learning Paths", "by-learning-path.html", "learning-paths"),
    ]
    links = "\n".join(
        '<a href="{}" class="{}">{}</a>'.format(
            href,
            "text-orange-500 font-semibold" if key == active
            else "text-gray-700 dark:text-gray-300 hover:text-orange-500 dark:hover:text-orange-400 transition-colors font-medium",
            label,
        )
        for label, href, key in items
    )
    logo = (
        '<a href="index.html" class="font-extrabold text-xl" '
        'style="background:linear-gradient(135deg,#CE422B,#f97316);'
        '-webkit-background-clip:text;-webkit-text-fill-color:transparent;background-clip:text">'
        '🦀 Functional Rust</a>'
    )
    return (
        '\n  <nav class="bg-white dark:bg-gray-900 shadow-sm sticky top-0 z-50 border-b border-gray-200 dark:border-gray-700">'
        '\n    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">'
        '\n      <div class="flex justify-between items-center h-16">'
        '\n        <div class="flex items-center gap-6">'
        '\n          <a href="https://hightechmind.io" class="text-gray-400 dark:text-gray-500 hover:text-orange-500 text-sm font-medium transition-colors">High Tech Mind</a>'
        f'\n          {logo}'
        '\n        </div>'
        '\n        <div class="flex items-center gap-3">'
        '\n          <div class="hidden md:flex items-center gap-5">'
        f'\n            {links}'
        '\n            <a href="https://linkedin.com/in/umurozkul" target="_blank" rel="noopener"'
        '               class="text-gray-700 dark:text-gray-300 hover:text-orange-500 transition-colors font-medium text-sm">LinkedIn</a>'
        '\n          </div>'
        f'\n          {THEME_TOGGLE_HTML}'
        '\n        </div>'
        '\n      </div>'
        '\n    </div>'
        '\n  </nav>'
    )

def footer_html(n_examples):
    year = datetime.now().year
    return (
        '\n  <footer class="bg-gray-900 text-gray-300 mt-20">'
        '\n    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">'
        '\n      <div class="grid grid-cols-1 md:grid-cols-3 gap-8">'
        '\n        <div>'
        '\n          <h3 class="text-white text-lg font-bold mb-3">🦀 Functional Rust</h3>'
        f'\n          <p class="text-sm">OCaml patterns translated to idiomatic Rust.<br>{n_examples} examples — new ones added daily.</p>'
        '\n        </div>'
        '\n        <div>'
        '\n          <h4 class="text-white font-semibold mb-3">Browse</h4>'
        '\n          <ul class="space-y-2 text-sm">'
        '\n            <li><a href="index.html" class="hover:text-white transition-colors">All Examples</a></li>'
        '\n            <li><a href="by-level.html" class="hover:text-white transition-colors">By Level</a></li>'
        '\n            <li><a href="by-topic.html" class="hover:text-white transition-colors">By Topic</a></li>'
        '\n            <li><a href="by-learning-path.html" class="hover:text-white transition-colors">Learning Paths</a></li>'
        '\n          </ul>'
        '\n        </div>'
        '\n        <div>'
        '\n          <h4 class="text-white font-semibold mb-3">Connect</h4>'
        '\n          <ul class="space-y-2 text-sm">'
        '\n            <li><a href="https://linkedin.com/in/umurozkul" target="_blank" rel="noopener" class="hover:text-white transition-colors">LinkedIn</a></li>'
        '\n            <li><a href="https://github.com/umurozkul" target="_blank" rel="noopener" class="hover:text-white transition-colors">GitHub</a></li>'
        '\n            <li><a href="https://hightechmind.io" class="hover:text-white transition-colors">hightechmind.io</a></li>'
        '\n          </ul>'
        '\n        </div>'
        '\n      </div>'
        '\n      <div class="border-t border-gray-800 mt-8 pt-8 text-center text-sm">'
        f'\n        <p>&copy; {year} High Tech Mind B.V. &middot; Educational resource by '
        '<a href="https://linkedin.com/in/umurozkul" class="hover:text-white transition-colors">Umur Ozkul</a></p>'
        '\n      </div>'
        '\n    </div>'
        '\n  </footer>'
    )

TAILWIND_CONFIG = """<script src="https://cdn.tailwindcss.com"></script>
  <script>
    tailwind.config = {
      darkMode: 'class',
      theme: {
        extend: {
          colors: { 'rust': '#CE422B', 'rust-dark': '#A5341F' },
          fontFamily: {
            'sans': ['Inter', 'system-ui', 'sans-serif'],
            'mono': ['JetBrains Mono', 'Consolas', 'monospace'],
          },
        },
      },
    };
  </script>"""

BASE_STYLE = """<style>
    body { font-family: 'Inter', system-ui, sans-serif; }
    code, pre { font-family: 'JetBrains Mono', Consolas, monospace; }
    pre[class*="language-"] { margin: 0.75rem 0; border-radius: 0.5rem; font-size: 0.85rem; }
    .bg-rust { background-color: #CE422B; }
    .text-rust { color: #CE422B; }
    .border-rust { border-color: #CE422B; }
  </style>"""

FONTS = (
    '<link rel="preconnect" href="https://fonts.googleapis.com">\n'
    '  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>\n'
    '  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700;800'
    '&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">'
)

PRISM_CSS = '<link href="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/themes/prism-tomorrow.min.css" rel="stylesheet" />'
PRISM_JS = (
    '<script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/prism.min.js"></script>\n'
    '  <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-rust.min.js"></script>\n'
    '  <script src="https://cdnjs.cloudflare.com/ajax/libs/prism/1.29.0/components/prism-ocaml.min.js"></script>'
)

def render_page(*, title, description, content, url, extra_head="", back_link=False,
                nav_active="", n_examples=0, keywords=None, og_type="website"):
    back = ("<div class='mb-6'><a href='index.html' class='text-orange-500 hover:underline font-medium'>"
            "← Back to Examples</a></div>" if back_link else "")
    return (
        '<!DOCTYPE html>\n'
        '<html lang="en" class="scroll-smooth">\n'
        '<head>\n'
        '  <meta charset="UTF-8">\n'
        '  <meta name="viewport" content="width=device-width, initial-scale=1.0">\n'
        f'  <title>{escape_html(title)} | Functional Rust</title>\n'
        f'  {meta_tags(title, description, url, keywords, og_type=og_type)}\n'
        f'  {TAILWIND_CONFIG}\n'
        f'  {FONTS}\n'
        f'  {PRISM_CSS}\n'
        f'  {BASE_STYLE}\n'
        f'  {THEME_JS}\n'
        f'  {extra_head}\n'
        '</head>\n'
        '<body class="bg-gray-50 dark:bg-gray-950 text-gray-900 dark:text-gray-100 antialiased transition-colors duration-200">\n'
        f'  {nav_html(nav_active)}\n'
        '  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">\n'
        f'    {back}\n'
        f'    {content}\n'
        '  </main>\n'
        f'  {footer_html(n_examples)}\n'
        f'  {PRISM_JS}\n'
        f'  {THEME_INIT_JS}\n'
        '</body>\n'
        '</html>'
    )


# ---- Page generators ----

def generate_index(examples_data):
    total = len(examples_data)
    by_level = {}
    for ex in examples_data:
        by_level.setdefault(ex["level"], []).append(ex)
    nf = len(by_level.get("fundamental",  []))
    ni = len(by_level.get("intermediate", []))
    na = len(by_level.get("advanced", []) + by_level.get("expert", []))

    stats = (
        '\n        <div class="grid grid-cols-2 md:grid-cols-4 gap-4 max-w-3xl mx-auto">'
        f'\n          <div class="bg-white/10 backdrop-blur-sm rounded-xl p-5 text-center"><div class="text-4xl font-bold">{total}</div><div class="text-sm text-gray-300 mt-1">Examples</div></div>'
        f'\n          <div class="bg-green-500/20 rounded-xl p-5 text-center"><div class="text-4xl font-bold text-green-300">{nf}</div><div class="text-sm text-gray-300 mt-1">Fundamental</div></div>'
        f'\n          <div class="bg-blue-500/20 rounded-xl p-5 text-center"><div class="text-4xl font-bold text-blue-300">{ni}</div><div class="text-sm text-gray-300 mt-1">Intermediate</div></div>'
        f'\n          <div class="bg-purple-500/20 rounded-xl p-5 text-center"><div class="text-4xl font-bold text-purple-300">{na}</div><div class="text-sm text-gray-300 mt-1">Advanced+</div></div>'
        '\n        </div>'
    )

    hero = (
        '\n    <section class="bg-gradient-to-br from-gray-900 via-gray-800 to-red-950 text-white rounded-2xl shadow-2xl p-10 sm:p-14 mb-12">'
        '\n      <div class="max-w-4xl mx-auto text-center">'
        '\n        <div class="text-7xl mb-6">🦀</div>'
        '\n        <h1 class="text-5xl font-extrabold mb-5 tracking-tight">Functional Rust</h1>'
        '\n        <p class="text-xl text-gray-300 mb-10 max-w-2xl mx-auto leading-relaxed">'
        '\n          Learn functional programming by translating OCaml idioms into idiomatic Rust.<br>'
        '\n          Progressive examples with side-by-side comparisons and full explanations.'
        '\n        </p>'
        f'\n        {stats}'
        '\n      </div>'
        '\n    </section>'
    )

    filter_btns = "\n".join(
        '<button class="level-filter px-4 py-2 rounded-full text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-orange-500 hover:text-white transition-colors" data-level="{}">{}</button>'.format(
            lvl, LEVELS[lvl][2])
        for lvl in ["fundamental", "intermediate", "advanced", "expert"]
    )

    search_bar = (
        '\n    <div class="flex flex-col sm:flex-row gap-3 items-start sm:items-center mb-6">'
        '\n      <div class="relative flex-1">'
        '\n        <svg class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">'
        '\n          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0"></path>'
        '\n        </svg>'
        '\n        <input id="search" type="text" placeholder="Search examples by title or concept…"'
        '\n          class="w-full pl-10 pr-4 py-3 rounded-xl border border-gray-200 dark:border-gray-600'
        '\n                 bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-400'
        '\n                 focus:ring-2 focus:ring-orange-400 focus:border-transparent outline-none transition-all text-sm" />'
        '\n      </div>'
        '\n      <div class="flex gap-2 flex-wrap">'
        '\n        <button class="level-filter px-4 py-2 rounded-full text-sm font-medium bg-gray-100 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-orange-500 hover:text-white transition-colors" data-level="all">All</button>'
        f'\n        {filter_btns}'
        '\n      </div>'
        '\n    </div>'
        '\n    <p class="text-sm text-gray-400 dark:text-gray-500 mb-4 h-5" id="result-count"></p>'
    )

    cards = "\n".join(example_card(ex) for ex in examples_data)

    grid = (
        '\n    <section class="mb-16">'
        '\n      <div class="flex items-center justify-between mb-6">'
        '\n        <h2 class="text-3xl font-bold text-gray-900 dark:text-white">All Examples</h2>'
        '\n        <div class="text-sm text-gray-500 dark:text-gray-400 hidden sm:block">'
        '\n          Browse by <a href="by-level.html" class="text-orange-500 hover:underline">level</a> &middot;'
        '\n          <a href="by-topic.html" class="text-orange-500 hover:underline">topic</a> &middot;'
        '\n          <a href="by-learning-path.html" class="text-orange-500 hover:underline">learning path</a>'
        '\n        </div>'
        '\n      </div>'
        f'\n      {search_bar}'
        '\n      <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">'
        f'\n        {cards}'
        '\n      </div>'
        '\n    </section>'
    )

    about = (
        '\n    <section class="bg-white dark:bg-gray-800 rounded-2xl border border-gray-200 dark:border-gray-700 p-8 mb-12">'
        '\n      <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">About This Project</h2>'
        '\n      <div class="text-gray-700 dark:text-gray-300 space-y-4 text-sm leading-relaxed">'
        '\n        <p><strong>Functional Rust</strong> is a comprehensive collection of functional programming'
        '\n        examples, translating OCaml idioms to idiomatic Rust. Each example includes:</p>'
        '\n        <ul class="space-y-1.5 ml-4">'
        '\n          <li>&#x2705; Side-by-side OCaml and Rust code</li>'
        '\n          <li>&#x2705; Full explanations of every translation decision</li>'
        '\n          <li>&#x2705; Key differences in type systems and semantics</li>'
        '\n          <li>&#x2705; Learning outcomes and practical exercises</li>'
        '\n        </ul>'
        '\n        <p>Follow along on <a href="https://linkedin.com/in/umurozkul" target="_blank" class="text-orange-500 hover:underline font-semibold">LinkedIn</a> for daily new examples.</p>'
        '\n        <div class="pt-4 border-t border-gray-200 dark:border-gray-700 text-gray-500 dark:text-gray-400">'
        '\n          Created by <strong class="text-gray-700 dark:text-gray-300">Umur Ozkul</strong>,'
        '\n          CTO at High Tech Mind B.V. — 30+ years of systems programming in OCaml, Rust, and functional languages.'
        '\n        </div>'
        '\n      </div>'
        '\n    </section>'
    )

    content = hero + grid + repo_cards_html() + about
    desc = f"Learn functional programming by translating OCaml idioms into idiomatic Rust. {total} examples covering HOFs, pattern matching, recursion and more."
    extra = SEARCH_JS + "\n" + jsonld_website("Functional Rust", desc, f"{SITE_BASE_URL}/index.html")

    return render_page(
        title="Functional Rust — OCaml Patterns in Rust",
        description=desc,
        content=content,
        url=f"{SITE_BASE_URL}/index.html",
        extra_head=extra,
        nav_active="examples",
        n_examples=total,
    )


def generate_by_level(examples_data):
    descs = {
        "fundamental":  "Core concepts: higher-order functions, closures, basic pattern matching, and list operations.",
        "intermediate": "Building blocks: algebraic types, iterators, error handling, and trait-based design.",
        "advanced":     "Deep dive: advanced type-system patterns, monadic composition, and systems FP.",
        "expert":       "Mastery: macros, unsafe FP idioms, generic programming, and performance-critical patterns.",
    }
    by_level = {}
    for ex in examples_data:
        by_level.setdefault(ex["level"], []).append(ex)

    present_levels = [lvl for lvl in ["fundamental", "intermediate", "advanced", "expert"] if by_level.get(lvl)]

    btn_base = 'border-2 px-5 py-2 rounded-full text-sm font-semibold transition-colors cursor-pointer'
    filter_btns = (
        f'<button {btn_base} data-fv="all" class="{btn_base} border-gray-300 dark:border-gray-600 text-gray-700 dark:text-gray-300 hover:border-orange-400">All <span class="opacity-70">({len(examples_data)})</span></button>\n'
        + "\n".join(
            '<button data-fv="{}" class="{} {} {} hover:opacity-90">{} <span class="opacity-70">({})</span></button>'.format(
                lvl, btn_base, LEVELS[lvl][0], LEVELS[lvl][1], LEVELS[lvl][2], len(by_level[lvl])
            )
            for lvl in present_levels
        )
    )

    sections = ""
    for lvl in present_levels:
        exs = by_level[lvl]
        cfg = LEVELS[lvl]
        cards = "\n".join(example_card(ex) for ex in exs)
        sections += (
            f'\n      <section data-fs="{lvl}" class="mb-12">'
            '\n        <div class="flex items-center gap-4 mb-4">'
            f'\n          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">{cfg[2]}</h2>'
            f'\n          <span class="px-3 py-1 rounded-full text-sm font-semibold {cfg[0]} {cfg[1]}">{len(exs)} examples</span>'
            '\n        </div>'
            f'\n        <p class="text-gray-600 dark:text-gray-400 mb-6 text-sm">{descs.get(lvl, "")}</p>'
            f'\n        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">{cards}</div>'
            '\n      </section>'
        )

    content = (
        '\n    <header class="mb-8">'
        '\n      <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-3">Browse By Level</h1>'
        '\n      <p class="text-xl text-gray-600 dark:text-gray-400 mb-6">Examples organized by difficulty — select a level to filter.</p>'
        '\n    </header>'
        '\n    <div data-filter-wrap>'
        f'\n      <div class="flex flex-wrap gap-2 mb-8">{filter_btns}</div>'
        f'\n      {sections}'
        '\n    </div>'
        f'\n    {FILTER_JS}'
    )

    level_url = f"{SITE_BASE_URL}/by-level.html"
    level_desc = "Functional Rust examples organized by difficulty — fundamental, intermediate, advanced, and expert."
    level_kw = "rust tutorial, functional rust beginner, rust intermediate, rust advanced, rust expert, learning rust"
    return render_page(
        title="Browse By Level",
        description=level_desc,
        content=content,
        url=level_url,
        nav_active="by-level",
        n_examples=len(examples_data),
        keywords=level_kw,
        extra_head=jsonld_collection("Functional Rust — Browse By Level", level_desc, level_url,
                                     [f"{SITE_BASE_URL}/{ex['href']}" for ex in examples_data]),
    )


def generate_by_topic(examples_data):
    # Group by topic_id using the TOPICS taxonomy
    by_topic = {tid: [] for tid, _, _ in TOPICS}
    for ex in examples_data:
        tid = ex.get("topic_id", "other")
        by_topic.setdefault(tid, []).append(ex)

    # Only include topics that have examples, in taxonomy order
    present = [(tid, label) for tid, label, _ in TOPICS if by_topic.get(tid)]

    btn_cls = ('text-left bg-white dark:bg-gray-800 border-2 border-gray-200 dark:border-gray-700 '
               'rounded-lg px-3 py-2 hover:border-orange-400 hover:text-orange-500 '
               'transition-colors text-sm font-medium')
    all_pill = (
        f'<button data-fv="all" class="{btn_cls}">'
        f'All <span class="text-gray-400 dark:text-gray-500 text-xs">({len(examples_data)})</span></button>'
    )
    pills = "\n".join(
        '<button data-fv="{}" class="{}">{} <span class="text-gray-400 dark:text-gray-500 text-xs">({})</span></button>'.format(
            tid, btn_cls, escape_html(label), len(by_topic[tid])
        )
        for tid, label in present
    )

    sections = ""
    for tid, label in present:
        exs   = by_topic[tid]
        cards = "\n".join(example_card(ex) for ex in exs)
        sections += (
            f'\n      <section data-fs="{tid}" class="mb-12">'
            '\n        <div class="flex items-center gap-3 mb-5">'
            f'\n          <h2 class="text-2xl font-bold text-gray-900 dark:text-white">{escape_html(label)}</h2>'
            f'\n          <span class="text-sm text-gray-400 dark:text-gray-500">{len(exs)} example{"s" if len(exs) != 1 else ""}</span>'
            '\n        </div>'
            f'\n        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">{cards}</div>'
            '\n      </section>'
        )

    content = (
        '\n    <header class="mb-10">'
        '\n      <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-3">Browse By Topic</h1>'
        f'\n      <p class="text-xl text-gray-600 dark:text-gray-400 mb-6">{len(present)} topics &middot; {len(examples_data)} examples — select a topic to filter.</p>'
        '\n    </header>'
        '\n    <div data-filter-wrap>'
        f'\n      <div class="flex flex-wrap gap-2 mb-8">{all_pill}\n{pills}</div>'
        f'\n      {sections}'
        '\n    </div>'
        f'\n    {FILTER_JS}'
    )

    topic_url  = f"{SITE_BASE_URL}/by-topic.html"
    topic_desc = f"Functional Rust examples by topic — {len(present)} categories covering HOFs, types, iterators, recursion and more."
    topic_kw   = "rust topics, rust iterators, rust error handling, rust closures, rust traits, rust async, functional programming rust"
    return render_page(
        title="Browse By Topic",
        description=topic_desc,
        content=content,
        url=topic_url,
        nav_active="by-topic",
        n_examples=len(examples_data),
        keywords=topic_kw,
        extra_head=jsonld_collection("Functional Rust — Browse By Topic", topic_desc, topic_url,
                                     [f"{SITE_BASE_URL}/{ex['href']}" for ex in examples_data]),
    )


def generate_by_learning_path(examples_data):
    level_order = {"fundamental": 0, "intermediate": 1, "advanced": 2, "expert": 3}
    path_exa = {p["id"]: [] for p in LEARNING_PATHS}
    for ex in examples_data:
        path_exa.setdefault(ex.get("path_id", "foundations"), []).append(ex)
    for pid in path_exa:
        path_exa[pid].sort(key=lambda e: (level_order.get(e["level"], 0), e["num_int"]))

    present_paths = [p for p in LEARNING_PATHS if path_exa.get(p["id"])]

    # Selector cards — one per path, act as filter buttons
    selector_cards = ""
    all_card = (
        '\n        <button data-fv="all" class="text-left bg-white dark:bg-gray-800 border-2 border-gray-200 '
        'dark:border-gray-700 rounded-xl p-5 hover:border-orange-400 transition-colors cursor-pointer w-full">'
        '\n          <div class="text-3xl mb-2">🗂</div>'
        f'\n          <div class="font-bold text-gray-900 dark:text-white text-sm mb-1">All Paths</div>'
        f'\n          <div class="text-xs text-gray-500 dark:text-gray-400">{len(examples_data)} examples</div>'
        '\n        </button>'
    )
    for path in present_paths:
        exs = path_exa[path["id"]]
        selector_cards += (
            f'\n        <button data-fv="{path["id"]}" class="text-left bg-white dark:bg-gray-800 border-2 border-gray-200 '
            'dark:border-gray-700 rounded-xl p-5 hover:border-orange-400 transition-colors cursor-pointer w-full">'
            f'\n          <div class="text-3xl mb-2">{path["icon"]}</div>'
            f'\n          <div class="font-bold text-gray-900 dark:text-white text-sm mb-1">{escape_html(path["title"])}</div>'
            f'\n          <div class="text-xs text-gray-500 dark:text-gray-400">{len(exs)} example{"s" if len(exs) != 1 else ""}</div>'
            '\n        </button>'
        )

    # Content sections — one per path
    sections = ""
    for path in present_paths:
        exs = path_exa[path["id"]]
        cards = "\n".join(example_card(ex) for ex in exs)
        sections += (
            f'\n      <section data-fs="{path["id"]}" class="mb-14">'
            '\n        <div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-2xl p-8">'
            '\n          <div class="flex items-start gap-5 mb-7">'
            f'\n            <div class="text-5xl mt-1">{path["icon"]}</div>'
            '\n            <div>'
            f'\n              <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-1">{escape_html(path["title"])}</h2>'
            f'\n              <p class="text-gray-600 dark:text-gray-400 mb-1">{escape_html(path["description"])}</p>'
            f'\n              <p class="text-sm text-gray-400 dark:text-gray-500">{len(exs)} example{"s" if len(exs) != 1 else ""}</p>'
            '\n            </div>'
            '\n          </div>'
            f'\n          <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-4">{cards}</div>'
            '\n        </div>'
            '\n      </section>'
        )

    content = (
        '\n    <header class="mb-8">'
        '\n      <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-3">Learning Paths</h1>'
        '\n      <p class="text-xl text-gray-600 dark:text-gray-400 mb-6">'
        '\n        Curated sequences from functional foundations to advanced systems FP — select a path to focus.'
        '\n      </p>'
        '\n    </header>'
        '\n    <div data-filter-wrap>'
        f'\n      <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3 mb-10">{all_card}{selector_cards}</div>'
        f'\n      {sections}'
        '\n    </div>'
        f'\n    {repo_cards_html()}'
        f'\n    {FILTER_JS}'
    )

    path_url  = f"{SITE_BASE_URL}/by-learning-path.html"
    path_desc = "Structured learning paths for Functional Rust — from OCaml fundamentals to advanced Rust patterns."
    path_kw   = "learn rust, rust learning path, rust roadmap, rust for beginners, functional rust course, rust tutorial series"
    return render_page(
        title="Learning Paths",
        description=path_desc,
        content=content,
        url=path_url,
        nav_active="learning-paths",
        n_examples=len(examples_data),
        keywords=path_kw,
        extra_head=jsonld_collection("Functional Rust — Learning Paths", path_desc, path_url,
                                     [f"{SITE_BASE_URL}/{ex['href']}" for ex in examples_data]),
    )


def generate_example_page(example_dir, all_examples, examples_data):
    dirname = example_dir.name
    num     = get_example_number(dirname)
    total   = len(examples_data)

    readme     = read_file(example_dir / "README.md")
    comparison = read_file(example_dir / "COMPARISON.md")
    rs_code    = read_rust_source(example_dir)
    ml_code    = read_file(example_dir / "example.ml")

    title     = extract_title(readme) or dirname.replace("-", " ").title()
    level     = extract_difficulty(readme)
    concepts  = extract_concepts(readme)
    video_url = extract_video_url(readme)

    # Extract all README sections — no truncation
    problem   = extract_section(readme, "Problem Statement")
    ocaml_app = extract_section(readme, "OCaml Approach", "OCaml Idiom")
    rust_app  = extract_section(readme, "Rust Approach", "Rust Idiom")
    learning  = extract_section(readme, "Learning Outcomes", "What You'll Learn")
    key_diff  = extract_section(readme, "Key Differences", "Key Difference")
    exercises = extract_section(readme, "Exercises", "Practice Exercises", "Exercise")

    # Main code from COMPARISON.md; fall back to source files
    ocaml_blocks, rust_blocks = parse_comparison(comparison)
    main_ocaml = ocaml_blocks[0][1] if ocaml_blocks else ml_code
    main_rust  = rust_blocks[0][1]  if rust_blocks  else rs_code

    # Full-source fallback: if example.ml missing, reuse main_ocaml from COMPARISON.md
    full_ocaml = ml_code or main_ocaml
    full_rust  = rs_code

    tests_code = extract_tests(rs_code)

    # Prev / next
    idx     = next((i for i, e in enumerate(all_examples) if e.name == dirname), 0)
    prev_ex = all_examples[idx - 1] if idx > 0 else None
    next_ex = all_examples[idx + 1] if idx < len(all_examples) - 1 else None

    prev_link = ""
    next_link = ""
    if prev_ex:
        prev_link = (
            f'<a href="{prev_ex.name}.html"'
            ' class="px-4 py-2 rounded-lg font-medium transition-colors text-sm'
            ' bg-gray-100 dark:bg-gray-700 hover:bg-gray-200 dark:hover:bg-gray-600 text-gray-700 dark:text-gray-300">'
            f'← {get_example_number(prev_ex.name)}</a>'
        )
    if next_ex:
        next_link = (
            f'<a href="{next_ex.name}.html"'
            ' class="px-4 py-2 rounded-lg font-medium transition-colors text-sm'
            ' bg-orange-500 hover:bg-orange-600 text-white">'
            f'{get_example_number(next_ex.name)} →</a>'
        )

    url         = f"{SITE_BASE_URL}/{dirname}.html"
    description = f"Learn {title} in Rust. Example {num} — tutorial, code, and OCaml comparison."
    keywords    = ", ".join(concepts[:6] + ["rust", "functional programming", "tutorial"])

    # Optional YouTube embed OR local Remotion video
    video_html = ""
    local_video_path = example_dir / "video.mp4"
    local_video_exists = local_video_path.exists()
    a11y_text = ""
    for a11y_name in ("video-description.txt", "video-accessibility.txt", "video-accessibility.md"):
        a11y_file = example_dir / a11y_name
        if a11y_file.exists():
            a11y_text = a11y_file.read_text(encoding="utf-8").strip()
            break

    if local_video_exists:
        # Copy video file to output directory alongside HTML
        video_out = RUST_DIR / f"{dirname}-video.mp4"
        import shutil as _shutil
        _shutil.copy2(str(local_video_path), str(video_out))
        a11y_html = ""
        if a11y_text:
            a11y_escaped = escape_html(a11y_text)
            a11y_html = (
                '\n        <details class="mt-4 bg-gray-50 dark:bg-gray-700 rounded-xl p-4">'
                '\n          <summary class="cursor-pointer text-sm font-semibold text-gray-700 dark:text-gray-300 select-none">'
                '\n            Text description (accessibility)'
                '\n          </summary>'
                f'\n          <p class="mt-3 text-sm text-gray-600 dark:text-gray-400 leading-relaxed whitespace-pre-line">{a11y_escaped}</p>'
                '\n        </details>'
            )
        video_html = (
            '\n      <section class="mb-10">'
            '\n        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">Tutorial Video</h2>'
            f'\n        <video class="w-full rounded-xl shadow-lg bg-black" controls preload="metadata"'
            f'\n               aria-label="Tutorial video: {escape_html(title)}">'
            f'\n          <source src="{dirname}-video.mp4" type="video/mp4">'
            '\n          Your browser does not support video playback.'
            '\n        </video>'
            f'{a11y_html}'
            '\n      </section>'
        )
    elif video_url:
        yt = re.search(r"(?:youtu\.be/|v=|embed/)([A-Za-z0-9_-]{11})", video_url)
        if yt:
            vid = yt.group(1)
            video_html = (
                '\n      <section class="mb-10">'
                '\n        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">Tutorial Video</h2>'
                '\n        <div class="relative w-full rounded-xl overflow-hidden shadow-lg" style="padding-top:56.25%">'
                f'\n          <iframe class="absolute inset-0 w-full h-full"'
                f'\n                  src="https://www.youtube.com/embed/{vid}"'
                f'\n                  title="{escape_html(title)} — Functional Rust Tutorial"'
                '\n                  frameborder="0" allowfullscreen'
                '\n                  allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"'
                f'\n                  aria-label="Tutorial video: {escape_html(title)}">'
                '\n          </iframe>'
                '\n        </div>'
                '\n      </section>'
            )

    def section_box(heading, body_html, extra_cls=""):
        if not body_html:
            return ""
        return (
            f'\n      <section class="mb-8 bg-white dark:bg-gray-800 rounded-2xl border border-gray-200 dark:border-gray-700 p-8 {extra_cls}">'
            f'\n        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-4">{heading}</h2>'
            '\n        <div class="prose dark:prose-invert max-w-none text-gray-700 dark:text-gray-300 text-sm leading-relaxed">'
            f'\n          {body_html}'
            '\n        </div>'
            '\n      </section>'
        )

    def code_tabs(section_id, rust_code, ocaml_code, rust_label="src/lib.rs", ocaml_label="example.ml"):
        """Tabbed code view — Rust first (default), OCaml second."""
        r_filled = escape_html(rust_code.rstrip())  if rust_code  else "(no source)"
        o_filled = escape_html(ocaml_code.rstrip()) if ocaml_code else "(no OCaml source for this example)"
        return (
            f'\n      <div data-tab-wrap id="{section_id}" class="mb-8 bg-gray-900 rounded-2xl overflow-hidden shadow-sm">'
            '\n        <div class="flex border-b border-gray-700">'
            '\n          <button data-tab-btn="rust"'
            '  class="px-5 py-3 text-sm font-semibold bg-gray-800 text-white flex items-center gap-2">'
            '\n            <span class="text-red-400">&#x1F980;</span> Rust'
            f'\n            <span class="text-gray-500 text-xs font-normal">{rust_label}</span>'
            '\n          </button>'
            '\n          <button data-tab-btn="ocaml"'
            '  class="px-5 py-3 text-sm font-semibold text-gray-400 hover:text-white transition-colors flex items-center gap-2">'
            '\n            <span class="text-orange-400">&#x1F42B;</span> OCaml'
            f'\n            <span class="text-gray-500 text-xs font-normal">{ocaml_label}</span>'
            '\n          </button>'
            '\n        </div>'
            f'\n        <div data-tab-panel="rust"  class="overflow-x-auto"><pre class="!m-0 !rounded-none"><code class="language-rust">{r_filled}</code></pre></div>'
            f'\n        <div data-tab-panel="ocaml" class="overflow-x-auto" style="display:none"><pre class="!m-0 !rounded-none"><code class="language-ocaml">{o_filled}</code></pre></div>'
            '\n      </div>'
        )

    tests_section = ""
    if tests_code:
        tests_section = (
            '\n      <section class="mb-8 bg-gray-900 rounded-2xl overflow-hidden">'
            '\n        <div class="flex items-center gap-2 px-6 py-4 bg-gray-800 border-b border-gray-700">'
            '\n          <span class="text-green-400 font-semibold text-sm">&#x2713; Tests</span>'
            '\n          <span class="text-gray-400 text-xs">Rust test suite</span>'
            '\n        </div>'
            '\n        <div class="p-2">'
            f'\n          <pre class="!m-0"><code class="language-rust">{escape_html(tests_code)}</code></pre>'
            '\n        </div>'
            '\n      </section>'
        )

    # Tutorial section: Rust-first content for people learning Rust (no OCaml assumed)
    tutorial_parts = []
    if problem:
        tutorial_parts.append(
            '\n        <div class="mb-6">'
            '\n          <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-3">The Problem</h3>'
            '\n          <div class="prose dark:prose-invert max-w-none text-gray-700 dark:text-gray-300 text-sm leading-relaxed">'
            f'\n            {md_to_html(problem)}'
            '\n          </div>'
            '\n        </div>'
        )
    if learning:
        tutorial_parts.append(
            '\n        <div class="mb-6 bg-green-50 dark:bg-green-900/10 rounded-xl p-5 border border-green-200 dark:border-green-800">'
            '\n          <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-3">&#x1F3AF; Learning Outcomes</h3>'
            '\n          <div class="prose dark:prose-invert max-w-none text-gray-700 dark:text-gray-300 text-sm leading-relaxed">'
            f'\n            {md_to_html(learning)}'
            '\n          </div>'
            '\n        </div>'
        )
    if rust_app:
        tutorial_parts.append(
            '\n        <div class="mb-2">'
            '\n          <h3 class="text-lg font-semibold text-gray-900 dark:text-white mb-3">&#x1F980; The Rust Way</h3>'
            '\n          <div class="prose dark:prose-invert max-w-none text-gray-700 dark:text-gray-300 text-sm leading-relaxed">'
            f'\n            {md_to_html(rust_app)}'
            '\n          </div>'
            '\n        </div>'
        )

    tutorial_html = ""
    if tutorial_parts:
        tutorial_html = (
            '\n      <section class="mb-8 bg-white dark:bg-gray-800 rounded-2xl border border-gray-200 dark:border-gray-700 p-8">'
            '\n        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-6">Tutorial</h2>'
            + "".join(tutorial_parts) +
            '\n      </section>'
        )

    content = (
        '\n    <article>'
        '\n      <header class="mb-8 bg-white dark:bg-gray-800 rounded-2xl border border-gray-200 dark:border-gray-700 p-8 shadow-sm">'
        '\n        <div class="flex items-center justify-between mb-4">'
        '\n          <div class="flex items-center gap-3">'
        f'\n            <span class="font-mono text-xl text-gray-400 dark:text-gray-500">{num}</span>'
        f'\n            {level_badge(level)}'
        '\n          </div>'
        f'\n          <div class="flex gap-2">{prev_link}{next_link}</div>'
        '\n        </div>'
        f'\n        <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-4 leading-tight">{escape_html(title)}</h1>'
        f'\n        <div class="flex flex-wrap gap-2">{concepts_html(concepts)}</div>'
        '\n      </header>'

        # Video (if present)
        f'\n      {video_html}'

        # Tutorial: Rust-focused explanation for learners
        f'\n      {tutorial_html}'

        # Code — Rust first tab, OCaml second
        '\n      <section class="mb-2">'
        '\n        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-3">Code Example</h2>'
        '\n      </section>'
        + code_tabs("code-comparison", main_rust, main_ocaml, "idiomatic", "functional") +

        # Key differences
        f'\n      {section_box("Key Differences", md_to_html(key_diff), "border-l-4 border-orange-400 dark:border-orange-500") if key_diff else ""}'

        # OCaml context — for those who know OCaml
        f'\n      {section_box("OCaml Approach", md_to_html(ocaml_app), "border-l-4 border-orange-200 dark:border-orange-900") if ocaml_app else ""}'

        # Full source — tabbed, Rust first
        '\n      <section class="mb-2">'
        '\n        <h2 class="text-2xl font-bold text-gray-900 dark:text-white mb-3">Full Source</h2>'
        '\n      </section>'
        + code_tabs("full-source", full_rust, full_ocaml, "src/lib.rs", "example.ml") +

        # Tests
        f'\n      {tests_section}'

        # Deep comparison (COMPARISON.md)
        f'\n      {section_box("Deep Comparison", md_to_html(comparison)) if comparison else ""}'

        # Exercises
        f'\n      {section_box("Exercises", md_to_html(exercises), "border-amber-200 dark:border-amber-800 !bg-amber-50 dark:!bg-amber-900/10") if exercises else ""}'

        # Repo card
        f'\n      {repo_cards_html(dirname=dirname)}'

        # Bottom nav
        '\n      <nav class="flex justify-between items-center pt-6 border-t border-gray-200 dark:border-gray-700 mt-8">'
        f'\n        <div>{prev_link}</div>'
        '\n        <a href="index.html" class="text-orange-500 hover:underline font-medium text-sm">&#x2191; All Examples</a>'
        f'\n        <div>{next_link}</div>'
        '\n      </nav>'
        '\n    </article>'
        f'\n    {TAB_JS}'
    )

    breadcrumb = jsonld_breadcrumb([
        ("Functional Rust",  f"{SITE_BASE_URL}/index.html"),
        ("Examples",         f"{SITE_BASE_URL}/index.html"),
        (f"Example {num}: {title}", url),
    ])
    article = jsonld_article(title, description, url,
                             keywords=keywords, num=int(num))
    return render_page(
        title=f"Example {num}: {title}",
        description=description,
        content=content,
        url=url,
        extra_head=article + "\n" + breadcrumb,
        back_link=False,
        nav_active="examples",
        n_examples=total,
        keywords=keywords,
        og_type="article",
    )


# ---- Backup ----

def backup_existing_site():
    if not RUST_DIR.exists():
        return
    files = list(RUST_DIR.glob("*.html"))
    if not files:
        return
    timestamp = datetime.now().strftime("%Y%m%d-%H%M%S")
    backup_dir = OUTPUT_DIR.parent / f"rust-site-backup-{timestamp}"
    try:
        shutil.copytree(RUST_DIR, backup_dir)
        print(f"Backed up {len(files)} existing files → {backup_dir}")
    except Exception as e:
        print(f"Warning: backup failed: {e}", file=sys.stderr)


# ---- Main ----

backup_existing_site()

if not EXAMPLES_DIR.exists():
    print(f"ERROR: examples directory not found: {EXAMPLES_DIR}", file=sys.stderr)
    sys.exit(1)

all_examples = sorted(EXAMPLES_DIR.iterdir(), key=lambda p: p.name)
all_examples = [e for e in all_examples if e.is_dir() and (e / "Cargo.toml").exists()]

print(f"Found {len(all_examples)} examples in {EXAMPLES_DIR}")

# Build shared metadata list (avoids re-reading README for every nav page)
examples_data = []
_seen_titles = set()
for ex in all_examples:
    readme   = read_file(ex / "README.md")
    # Skip placeholder/dummy examples: no README means no real content
    if not readme.strip():
        print(f"  [skip] {ex.name} — no README.md")
        continue
    num      = get_example_number(ex.name)
    title    = extract_title(readme) or ex.name.replace("-", " ").title()
    # Skip duplicate examples (same title seen before)
    title_key = title.lower().strip()
    if title_key in _seen_titles:
        print(f"  [skip] {ex.name} — duplicate title: {title}")
        continue
    _seen_titles.add(title_key)
    level    = extract_difficulty(readme)
    concepts = extract_concepts(readme)
    path_id  = classify_learning_path(concepts, title, ex.name)
    topic_id = classify_topic(ex.name)
    examples_data.append({
        "dirname":  ex.name,
        "num":      num,
        "num_int":  int(num),
        "title":    title,
        "level":    level,
        "concepts": concepts,
        "path_id":  path_id,
        "topic_id": topic_id,
        "href":     f"{ex.name}.html",
    })

# Only generate pages for examples that made it into examples_data
_included_dirnames = {e["dirname"] for e in examples_data}

# Generate individual example pages
generated = 0
for ex in all_examples:
    if ex.name not in _included_dirnames:
        continue
    generated += 1
    html = generate_example_page(ex, all_examples, examples_data)
    out  = RUST_DIR / f"{ex.name}.html"
    out.write_text(html, encoding="utf-8")
    print(f"  [{generated}] {ex.name}.html")

# Generate the 4 navigation / index pages
pages = [
    ("index.html",            generate_index(examples_data)),
    ("by-level.html",         generate_by_level(examples_data)),
    ("by-topic.html",         generate_by_topic(examples_data)),
    ("by-learning-path.html", generate_by_learning_path(examples_data)),
]
for filename, html in pages:
    (RUST_DIR / filename).write_text(html, encoding="utf-8")
    print(f"  {filename} ({len(html):,} bytes)")

# Generate sitemap.xml
today = datetime.now().strftime("%Y-%m-%d")
static_urls = [
    (f"{SITE_BASE_URL}/index.html",            "1.0", "weekly"),
    (f"{SITE_BASE_URL}/by-level.html",          "0.8", "weekly"),
    (f"{SITE_BASE_URL}/by-topic.html",          "0.8", "weekly"),
    (f"{SITE_BASE_URL}/by-learning-path.html",  "0.8", "weekly"),
]
example_urls = [
    (f"{SITE_BASE_URL}/{ex['href']}", "0.7", "monthly")
    for ex in examples_data
]
sitemap_entries = "\n".join(
    f"  <url>\n    <loc>{loc}</loc>\n    <lastmod>{today}</lastmod>\n    <changefreq>{freq}</changefreq>\n    <priority>{pri}</priority>\n  </url>"
    for loc, pri, freq in static_urls + example_urls
)
sitemap_xml = (
    '<?xml version="1.0" encoding="UTF-8"?>\n'
    '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n'
    f'{sitemap_entries}\n'
    '</urlset>'
)
(RUST_DIR / "sitemap.xml").write_text(sitemap_xml, encoding="utf-8")
print(f"  sitemap.xml ({len(examples_data)+4} URLs)")

# Generate robots.txt
robots_txt = (
    "User-agent: *\n"
    "Allow: /\n"
    f"Sitemap: {SITE_BASE_URL}/sitemap.xml\n"
)
(RUST_DIR / "robots.txt").write_text(robots_txt, encoding="utf-8")
print(f"  robots.txt")

total_out = generated + len(pages) + 2  # +2 for sitemap + robots
print(f"\nDone! {total_out} files in {RUST_DIR}/")
print(f"\nLocal preview:")
print(f"  python3 -m http.server 8080 --directory {RUST_DIR}")
print(f"  open http://localhost:8080/index.html")
