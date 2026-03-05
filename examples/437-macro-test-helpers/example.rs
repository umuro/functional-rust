// Test helper macros in Rust

// Parameterized test case generator
macro_rules! test_cases {
    (
        $test_name:ident :
        fn $fn_name:ident($input:ident: $input_ty:ty) -> $output_ty:ty
        = $function:expr ;
        $( $case_name:ident: $input_val:expr => $expected:expr ),* $(,)?
    ) => {
        $(
            #[test]
            fn $case_name() {
                let $input: $input_ty = $input_val;
                let expected: $output_ty = $expected;
                let actual = $function($input);
                assert_eq!(actual, expected,
                    concat!("Test case '", stringify!($case_name),
                            "' failed: input = {:?}"),
                    $input_val);
            }
        )*
    };
}

// Custom assertion macros with better messages
macro_rules! assert_between {
    ($val:expr, $lo:expr, $hi:expr) => {
        assert!($val >= $lo && $val <= $hi,
            "{} = {} is not in range [{}, {}]",
            stringify!($val), $val, $lo, $hi);
    };
}

macro_rules! assert_sorted {
    ($v:expr) => {
        {
            let v = &$v;
            for i in 1..v.len() {
                assert!(v[i-1] <= v[i],
                    "Not sorted at index {}: {:?} > {:?}", i-1, v[i-1], v[i]);
            }
        }
    };
}

macro_rules! assert_all {
    ($v:expr, $pred:expr, $msg:expr) => {
        for (i, item) in $v.iter().enumerate() {
            assert!($pred(item), "{} failed for item[{}] = {:?}", $msg, i, item);
        }
    };
}

// Fixture macro
macro_rules! with_temp_vec {
    (let $name:ident = $init:expr; $($body:tt)*) => {
        {
            let mut $name = $init;
            $($body)*
        }
    };
}

// Test builder
macro_rules! build_test_data {
    ($($field:ident : $val:expr),* $(,)?) => {
        TestData {
            $($field: $val,)*
            ..TestData::default()
        }
    };
}

#[derive(Debug, Default)]
struct TestData {
    id: u32,
    name: String,
    value: f64,
    active: bool,
}

fn double(x: i32) -> i32 { x * 2 }
fn square(x: i32) -> i32 { x * x }

// Generate test cases using macro
test_cases! {
    doubling:
    fn _double(x: i32) -> i32 = double;
    test_double_0: 0 => 0,
    test_double_1: 1 => 2,
    test_double_5: 5 => 10,
    test_double_neg: -3 => -6,
}

test_cases! {
    squaring:
    fn _square(x: i32) -> i32 = square;
    test_square_0: 0 => 0,
    test_square_2: 2 => 4,
    test_square_5: 5 => 25,
}

fn main() {
    println!("Test helpers work both in tests and in main");

    let v = vec![1, 2, 3, 4, 5];
    assert_sorted!(v);
    assert_between!(v[2], 1, 5);
    assert_all!(v, |x| *x > 0, "all positive");

    let td = build_test_data!(
        id: 42,
        name: "test".to_string(),
        active: true,
    );
    println!("TestData: {:?}", td);
}

#[cfg(test)]
mod extra_tests {
    #[test]
    fn test_assert_between() {
        assert_between!(5, 1, 10);
    }

    #[test]
    fn test_assert_sorted() {
        let v = vec![1, 2, 3, 4, 5];
        assert_sorted!(v);
    }

    #[test]
    #[should_panic]
    fn test_assert_sorted_fails() {
        let v = vec![3, 1, 2];
        assert_sorted!(v);
    }
}
