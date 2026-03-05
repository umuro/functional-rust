(* 439. assert_matches! style – OCaml *)

let assert_matches ~msg pred v =
  if not (pred v) then failwith msg

let parse_positive s =
  match int_of_string_opt s with
  | Some n when n > 0 -> Ok n
  | Some _ -> Error "not positive"
  | None   -> Error "not a number"

let () =
  (* Pattern-match assertions *)
  (match parse_positive "42" with
   | Ok n when n > 0 -> Printf.printf "Ok positive: %d\n" n
   | _ -> failwith "expected Ok positive");

  assert_matches ~msg:"should be Error"
    (function Error _ -> true | _ -> false)
    (parse_positive "-5");

  (* matches! equivalent: check shape without unwrapping *)
  let is_ok = function Ok _ -> true | _ -> false in
  assert (is_ok (parse_positive "10"));
  Printf.printf "All assertions passed!\n"
