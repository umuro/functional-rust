(* 1009: Collecting Results *)
(* Turning a list of results into a result of list *)

let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "bad: %s" s)

(* Approach 1: Manual fold — short-circuits on first error *)
let collect_results results =
  List.fold_left (fun acc r ->
    match acc, r with
    | Error e, _ -> Error e        (* already failed *)
    | _, Error e -> Error e        (* new failure *)
    | Ok xs, Ok x -> Ok (xs @ [x]) (* both ok, accumulate *)
  ) (Ok []) results

(* Approach 2: Tail-recursive with early exit *)
let rec sequence_results acc = function
  | [] -> Ok (List.rev acc)
  | Ok x :: rest -> sequence_results (x :: acc) rest
  | Error e :: _ -> Error e

let sequence rs = sequence_results [] rs

(* Approach 3: map then sequence *)
let traverse f xs =
  List.map f xs |> sequence

let test_fold () =
  let inputs = ["1"; "2"; "3"] in
  let results = List.map parse_int inputs in
  assert (collect_results results = Ok [1; 2; 3]);
  let bad_inputs = ["1"; "abc"; "3"] in
  let bad_results = List.map parse_int bad_inputs in
  (match collect_results bad_results with
   | Error e -> assert (e = "bad: abc")
   | Ok _ -> assert false);
  Printf.printf "  Approach 1 (fold): passed\n"

let test_sequence () =
  assert (sequence [Ok 1; Ok 2; Ok 3] = Ok [1; 2; 3]);
  (match sequence [Ok 1; Error "fail"; Ok 3] with
   | Error "fail" -> ()
   | _ -> assert false);
  Printf.printf "  Approach 2 (sequence): passed\n"

let test_traverse () =
  assert (traverse parse_int ["10"; "20"; "30"] = Ok [10; 20; 30]);
  assert (traverse parse_int [] = Ok []);
  (match traverse parse_int ["1"; "x"] with
   | Error _ -> ()
   | Ok _ -> assert false);
  Printf.printf "  Approach 3 (traverse): passed\n"

let () =
  Printf.printf "Testing collecting results:\n";
  test_fold ();
  test_sequence ();
  test_traverse ();
  Printf.printf "✓ All tests passed\n"
