(* 744: Unit Test Patterns — OCaml with Alcotest-style structure *)

(* The code under test *)
let clamp lo hi x = max lo (min hi x)

let divide_checked a b =
  if b = 0 then None
  else Some (a / b)

let is_palindrome s =
  let n = String.length s in
  let rec check i =
    if i >= n / 2 then true
    else if s.[i] <> s.[n - 1 - i] then false
    else check (i + 1)
  in
  check 0

(* Helper assertion functions *)
let assert_some result =
  match result with
  | Some x -> x
  | None   -> failwith "Expected Some, got None"

let assert_none result =
  match result with
  | None   -> ()
  | Some _ -> failwith "Expected None, got Some"

(* AAA pattern tests *)
let test_clamp_below_lo () =
  (* Arrange *)
  let lo, hi, x = 0, 10, -5 in
  (* Act *)
  let result = clamp lo hi x in
  (* Assert *)
  assert (result = 0)

let test_clamp_within_range () =
  let lo, hi, x = 0, 10, 5 in
  let result = clamp lo hi x in
  assert (result = 5)

let test_clamp_above_hi () =
  let lo, hi, x = 0, 10, 15 in
  let result = clamp lo hi x in
  assert (result = 10)

let test_divide_checked_non_zero () =
  let result = divide_checked 10 3 in
  let v = assert_some result in
  assert (v = 3)

let test_divide_checked_by_zero () =
  assert_none (divide_checked 10 0)

let test_palindrome () =
  assert (is_palindrome "racecar");
  assert (is_palindrome "");
  assert (is_palindrome "a");
  assert (not (is_palindrome "hello"))

let () =
  test_clamp_below_lo ();
  test_clamp_within_range ();
  test_clamp_above_hi ();
  test_divide_checked_non_zero ();
  test_divide_checked_by_zero ();
  test_palindrome ();
  Printf.printf "All OCaml tests passed!\n"
