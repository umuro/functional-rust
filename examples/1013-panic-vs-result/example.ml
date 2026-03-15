(* 1013: Panic vs Result
   OCaml uses:
   - failwith / invalid_arg / assert for programming errors (like Rust panic!)
   - Result for expected, recoverable failures
   The distinction: "this cannot happen" vs "the caller should handle this" *)

(* Approach 1: failwith / assert — for programming errors / invariants *)
let divide_or_panic a b =
  if b = 0 then failwith "division by zero: programming error"
  else a / b

let first_element = function
  | []    -> invalid_arg "slice must not be empty"
  | x :: _ -> x

(* Approach 2: Result — for expected recoverable failures *)
let divide a b =
  if b = 0 then Error "division by zero"
  else Ok (a / b)

let parse_positive s =
  match int_of_string_opt (String.trim s) with
  | None   -> Error (Printf.sprintf "not a number: %s" s)
  | Some n ->
    if n <= 0 then Error (Printf.sprintf "not positive: %d" n)
    else Ok n

(* assert — always checked, fails loudly *)
let process_data data =
  assert (List.length data <= 1000);  (* always checked *)
  List.fold_left (+) 0 data

let () =
  assert (divide_or_panic 10 2 = 5);

  (try ignore (divide_or_panic 10 0)
   with Failure msg ->
     assert (String.length msg > 0));

  assert (first_element [1; 2; 3] = 1);

  (try ignore (first_element [])
   with Invalid_argument msg ->
     assert (String.length msg > 0));

  assert (divide 10 2 = Ok 5);
  assert (divide 10 0 = Error "division by zero");

  assert (parse_positive "42" = Ok 42);
  (match parse_positive "-5" with Error msg -> assert (String.length msg > 0) | _ -> assert false);
  (match parse_positive "abc" with Error msg -> assert (String.length msg > 0) | _ -> assert false);

  assert (process_data [1; 2; 3] = 6);

  (* option wrapping — like unwrap/expect *)
  let v = Some 42 in
  assert (Option.get v = 42);  (* raises Invalid_argument if None — like unwrap *)

  Printf.printf "divide 10 2 = %s\n"
    (match divide 10 2 with Ok n -> string_of_int n | Error e -> e)
