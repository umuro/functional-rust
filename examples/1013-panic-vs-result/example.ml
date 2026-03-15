(* 1013: Panic vs Result *)
(* When to use exceptions vs Result in OCaml *)

(* Approach 1: Exceptions — for "should never happen" bugs *)
let divide_exn a b =
  if b = 0 then failwith "division by zero: programming error"
  else a / b

let head_exn = function
  | [] -> invalid_arg "head of empty list"
  | x :: _ -> x

(* Approach 2: Result — for expected failures *)
let divide a b =
  if b = 0 then Error "division by zero"
  else Ok (a / b)

let parse_positive s =
  match int_of_string_opt s with
  | None -> Error (Printf.sprintf "not a number: %s" s)
  | Some n when n <= 0 -> Error (Printf.sprintf "not positive: %d" n)
  | Some n -> Ok n

(* Approach 3: assert for invariants *)
let process_list lst =
  assert (List.length lst > 0);  (* crashes if invariant violated *)
  List.hd lst

let test_exceptions () =
  assert (divide_exn 10 2 = 5);
  (try let _ = divide_exn 10 0 in assert false
   with Failure _ -> ());
  assert (head_exn [1; 2; 3] = 1);
  (try let _ = head_exn [] in assert false
   with Invalid_argument _ -> ());
  Printf.printf "  Approach 1 (exceptions for bugs): passed\n"

let test_result () =
  assert (divide 10 2 = Ok 5);
  assert (divide 10 0 = Error "division by zero");
  assert (parse_positive "42" = Ok 42);
  assert (parse_positive "-5" = Error "not positive: -5");
  assert (parse_positive "abc" = Error "not a number: abc");
  Printf.printf "  Approach 2 (result for expected failures): passed\n"

let test_assert () =
  assert (process_list [1; 2; 3] = 1);
  (try let _ = process_list [] in assert false
   with Assert_failure _ -> ());
  Printf.printf "  Approach 3 (assert for invariants): passed\n"

let () =
  Printf.printf "Testing panic vs result:\n";
  test_exceptions ();
  test_result ();
  test_assert ();
  Printf.printf "✓ All tests passed\n"
