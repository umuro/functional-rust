(* 744: Unit Test Patterns — modules, helpers, AAA pattern *)

(* ── Code under test ─────────────────────────────────────────────────────── *)

let clamp lo hi x = max lo (min hi x)

let divide_checked a b =
  if b = 0 then None else Some (a / b)

let is_palindrome s =
  let n = String.length s in
  let rec check i =
    if i >= n / 2 then true
    else if s.[i] <> s.[n - 1 - i] then false
    else check (i + 1)
  in
  check 0

let fizzbuzz n =
  match (n mod 3, n mod 5) with
  | (0, 0) -> "FizzBuzz"
  | (0, _) -> "Fizz"
  | (_, 0) -> "Buzz"
  | _      -> string_of_int n

(* ── Test helpers ────────────────────────────────────────────────────────── *)

(* Assert two floats are equal within epsilon *)
let assert_approx_eq ?(eps=1e-10) a b =
  if abs_float (a -. b) >= eps then
    failwith (Printf.sprintf "assert_approx_eq: |%f - %f| = %f >= %f"
                a b (abs_float (a -. b)) eps)

(* Assert a list is sorted ascending *)
let assert_sorted compare lst =
  let rec check = function
    | [] | [_] -> ()
    | a :: (b :: _ as rest) ->
      if compare a b > 0 then failwith "not sorted"
      else check rest
  in
  check lst

(* Lightweight test runner — run a list of (name, thunk) pairs *)
let run_tests tests =
  let passed = ref 0 and failed = ref 0 in
  List.iter (fun (name, f) ->
    try
      f ();
      incr passed;
      Printf.printf "  PASS: %s\n" name
    with
    | Failure msg ->
      incr failed;
      Printf.printf "  FAIL: %s — %s\n" name msg
    | exn ->
      incr failed;
      Printf.printf "  FAIL: %s — exception: %s\n" name (Printexc.to_string exn)
  ) tests;
  Printf.printf "\n%d passed, %d failed\n" !passed !failed;
  if !failed > 0 then exit 1

(* ── Tests ───────────────────────────────────────────────────────────────── *)

let () = run_tests [
  (* clamp *)
  "clamp: below lo returns lo", (fun () ->
    (* Arrange *)
    let (lo, hi, x) = (0, 10, -5) in
    (* Act *)
    let result = clamp lo hi x in
    (* Assert *)
    assert (result = 0));

  "clamp: within range returns x", (fun () ->
    assert (clamp 0 10 5 = 5));

  "clamp: above hi returns hi", (fun () ->
    assert (clamp 0 10 15 = 10));

  "clamp: at boundaries", (fun () ->
    assert (clamp 0 10 0 = 0);
    assert (clamp 0 10 10 = 10));

  (* divide_checked *)
  "divide_checked: non-zero returns Some", (fun () ->
    assert (divide_checked 10 3 = Some 3));

  "divide_checked: by zero returns None", (fun () ->
    assert (divide_checked 42 0 = None));

  "divide_checked: negative dividend", (fun () ->
    assert (divide_checked (-10) 2 = Some (-5)));

  (* is_palindrome *)
  "palindrome: empty is palindrome", (fun () ->
    assert (is_palindrome ""));

  "palindrome: single char is palindrome", (fun () ->
    assert (is_palindrome "a"));

  "palindrome: racecar is palindrome", (fun () ->
    assert (is_palindrome "racecar"));

  "palindrome: hello is not palindrome", (fun () ->
    assert (not (is_palindrome "hello")));

  (* fizzbuzz *)
  "fizzbuzz: divisible by both returns FizzBuzz", (fun () ->
    assert (fizzbuzz 15 = "FizzBuzz"));

  "fizzbuzz: divisible by 3 returns Fizz", (fun () ->
    assert (fizzbuzz 9 = "Fizz"));

  "fizzbuzz: divisible by 5 returns Buzz", (fun () ->
    assert (fizzbuzz 10 = "Buzz"));

  "fizzbuzz: other returns number", (fun () ->
    assert (fizzbuzz 7 = "7"));

  (* helper tests *)
  "assert_approx_eq: passes for close floats", (fun () ->
    assert_approx_eq (0.1 +. 0.2) 0.3);

  "assert_sorted: passes for sorted list", (fun () ->
    assert_sorted compare [1; 2; 3; 4; 5];
    assert_sorted compare [1];
    assert_sorted compare ([] : int list));

  (* OCaml has no #[should_panic] but we can test for exceptions *)
  "division by zero raises exception", (fun () ->
    try ignore (1 / 0); failwith "expected exception"
    with Division_by_zero -> ());
]
