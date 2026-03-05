(* Test helper macros in OCaml via Alcotest/OUnit2 patterns *)

(* Parameterized test helper *)
let test_cases name cases =
  List.iter (fun (input, expected, desc) ->
    if input = expected then
      Printf.printf "PASS: %s - %s\n" name desc
    else
      Printf.printf "FAIL: %s - %s (got %d, expected %d)\n"
        name desc input expected
  ) cases

let () =
  test_cases "arithmetic" [
    (1 + 1, 2, "addition");
    (3 * 3, 9, "multiplication");
    (10 - 3, 7, "subtraction");
  ];

  (* Custom assertion *)
  let assert_in_range n lo hi =
    if n >= lo && n <= hi then ()
    else failwith (Printf.sprintf "%d not in [%d, %d]" n lo hi)
  in
  assert_in_range 5 1 10;
  Printf.printf "All tests passed\n"
