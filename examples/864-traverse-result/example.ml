(* Example 065: Traverse with Result *)
(* Turn a list of Results into a Result of list *)

(* Approach 1: Recursive traverse *)
let rec traverse_result f = function
  | [] -> Ok []
  | x :: xs ->
    match f x with
    | Error e -> Error e
    | Ok y ->
      match traverse_result f xs with
      | Error e -> Error e
      | Ok ys -> Ok (y :: ys)

(* Approach 2: fold_right based *)
let traverse_result_fold f xs =
  List.fold_right (fun x acc ->
    match f x, acc with
    | Ok y, Ok ys -> Ok (y :: ys)
    | Error e, _ | _, Error e -> Error e
  ) xs (Ok [])

(* Approach 3: sequence *)
let sequence_result xs = traverse_result Fun.id xs

(* Test functions *)
let parse_positive s =
  match int_of_string_opt s with
  | None -> Error (Printf.sprintf "Not a number: %s" s)
  | Some n when n <= 0 -> Error (Printf.sprintf "Not positive: %d" n)
  | Some n -> Ok n

let validate_username s =
  if String.length s < 3 then Error "Too short"
  else if String.length s > 20 then Error "Too long"
  else Ok s

let () =
  assert (traverse_result parse_positive ["1"; "2"; "3"] = Ok [1; 2; 3]);
  assert (traverse_result parse_positive ["1"; "bad"; "3"] = Error "Not a number: bad");
  assert (traverse_result parse_positive ["1"; "-2"; "3"] = Error "Not positive: -2");
  assert (traverse_result parse_positive [] = Ok []);

  assert (traverse_result_fold parse_positive ["1"; "2"; "3"] = Ok [1; 2; 3]);

  assert (sequence_result [Ok 1; Ok 2; Ok 3] = Ok [1; 2; 3]);
  assert (sequence_result [Ok 1; Error "e"; Ok 3] = Error "e");

  assert (traverse_result validate_username ["alice"; "bob"] = Ok ["alice"; "bob"]);
  assert (traverse_result validate_username ["alice"; "ab"] = Error "Too short");

  Printf.printf "✓ All tests passed\n"
