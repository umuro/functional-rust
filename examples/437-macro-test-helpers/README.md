# 437: Test Helper Macros

**Difficulty:** 2  **Level:** Intermediate

Write macros that generate test cases, custom assertions, and fixtures — turning repetitive test boilerplate into readable, maintainable test suites.

## The Problem This Solves

Test suites grow fast. A function with five edge cases needs five near-identical `#[test]` functions that differ only in input and expected output. Copy-pasting them works until someone changes the function signature and has to update every copy. Missing one is a silent test gap.

Custom assertion macros solve the other half: `assert!(v.len() == 3)` gives you `assertion failed: v.len() == 3` when it fails. `assert_has_len!(v, 3)` gives you `Expected len 3 but got 7` — pointing directly at the problem with domain vocabulary. The raw `assert!` version requires you to write the message yourself every time, and most developers don't bother.

Test macros also enable fixture and builder patterns that would otherwise require verbose setup in every test: `build_test_data!(id: 42, active: true)` fills a struct with defaults and overrides only the fields you care about for that test.

## The Intuition

`test_cases!` is a code generator: it takes a list of `name: input => expected` pairs and emits one `#[test]` function per pair. Each generated function has a distinct name so failures identify themselves, calls the function under test, and compares the result. You write the table; the macro writes the test functions.

Custom assertion macros are thin wrappers around `assert!` that format a richer message. They're worth writing for any assertion pattern you repeat more than twice — the investment in the macro pays back immediately in better failure messages.

## How It Works in Rust

```rust
// ── Parameterised test generator ─────────────────────────────────────────────
macro_rules! test_cases {
    (
        $test_group:ident:
        fn $fn_name:ident($input:ident: $it:ty) -> $ot:ty = $function:expr;
        $($case_name:ident: $input_val:expr => $expected:expr),* $(,)?
    ) => {
        $(
            #[test]  // each case becomes an independent #[test] function
            fn $case_name() {
                let $input: $it = $input_val;
                let actual = $function($input);
                assert_eq!(actual, $expected,
                    "Test '{}': input={:?}", stringify!($case_name), $input_val);
            }
        )*
    };
}

fn double(x: i32) -> i32 { x * 2 }

// Generates 4 separate #[test] fns: test_double_0, test_double_1, etc.
test_cases! {
    doubling:
    fn _double(x: i32) -> i32 = double;
    test_double_0: 0 => 0,
    test_double_1: 1 => 2,
    test_double_5: 5 => 10,
    test_double_neg: -3 => -6,
}

// ── Custom assertions with domain vocabulary ──────────────────────────────────
macro_rules! assert_between {
    ($val:expr, $lo:expr, $hi:expr) => {
        assert!($val >= $lo && $val <= $hi,
            "{} = {} is not in [{}, {}]", stringify!($val), $val, $lo, $hi);
    };
}

macro_rules! assert_sorted {
    ($v:expr) => {{
        let v = &$v;
        for i in 1..v.len() {
            assert!(v[i-1] <= v[i],
                "Not sorted at index {}: {:?} > {:?}", i-1, v[i-1], v[i]);
        }
    }};
}

macro_rules! assert_all {
    ($v:expr, $pred:expr, $msg:expr) => {
        for (i, item) in $v.iter().enumerate() {
            assert!($pred(item), "{} failed for item[{}] = {:?}", $msg, i, item);
        }
    };
}

// ── Test data builder (default + overrides) ───────────────────────────────────
#[derive(Debug, Default)]
struct TestData { id: u32, name: String, value: f64, active: bool }

macro_rules! build_test_data {
    ($($field:ident : $val:expr),* $(,)?) => {
        TestData { $($field: $val,)* ..TestData::default() }
    };
}

// Only specify what matters for this test — defaults fill the rest
let td = build_test_data!(id: 42, active: true);
```

## What This Unlocks

- **Table-driven tests** — define 10 cases as a table; the macro generates 10 independent, named test functions that appear separately in test output.
- **Readable failure messages** — `assert_sorted!`, `assert_between!`, `assert_all!` produce contextual messages instead of bare "assertion failed".
- **Focused fixtures** — `build_test_data!` lets each test declare only its relevant fields, making test intent obvious and reducing coupling to struct field order.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Parameterised tests | OUnit/Alcotest test lists with `List.map`; no macro needed | `test_cases!` macro generates individual `#[test]` functions |
| Custom assertions | `assert_equal` with custom printer in OUnit | `macro_rules!` assertion with custom message format |
| Test fixtures | Record construction with let-bindings | `build_test_data!` macro with `..Default::default()` |
| Failure output | Alcotest shows diff; OUnit shows message | `assert_eq!` shows both values; custom macros add context |
