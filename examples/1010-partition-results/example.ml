(* 1010: Partition Results *)
(* Separate Ok and Err values from a list of results *)

let parse_int s =
  match int_of_string_opt s with
  | Some n -> Ok n
  | None -> Error (Printf.sprintf "bad: %s" s)

(* Approach 1: List.partition with is_ok helper *)
let is_ok = function Ok _ -> true | Error _ -> false

let partition_results results =
  let oks, errs = List.partition is_ok results in
  let unwrap_ok = function Ok v -> v | Error _ -> assert false in
  let unwrap_err = function Error e -> e | Ok _ -> assert false in
  (List.map unwrap_ok oks, List.map unwrap_err errs)

(* Approach 2: Single fold separating into two accumulators *)
let partition_fold results =
  let oks, errs = List.fold_left (fun (oks, errs) r ->
    match r with
    | Ok v -> (v :: oks, errs)
    | Error e -> (oks, e :: errs)
  ) ([], []) results in
  (List.rev oks, List.rev errs)

let test_partition () =
  let inputs = ["1"; "abc"; "3"; "def"; "5"] in
  let results = List.map parse_int inputs in
  let oks, errs = partition_results results in
  assert (oks = [1; 3; 5]);
  assert (errs = ["bad: abc"; "bad: def"]);
  Printf.printf "  Approach 1 (partition + unwrap): passed\n"

let test_fold () =
  let inputs = ["1"; "abc"; "3"; "def"; "5"] in
  let results = List.map parse_int inputs in
  let oks, errs = partition_fold results in
  assert (oks = [1; 3; 5]);
  assert (errs = ["bad: abc"; "bad: def"]);
  (* All ok *)
  let all_ok = List.map parse_int ["1"; "2"; "3"] in
  let oks, errs = partition_fold all_ok in
  assert (oks = [1; 2; 3]);
  assert (errs = []);
  Printf.printf "  Approach 2 (fold): passed\n"

let () =
  Printf.printf "Testing partition results:\n";
  test_partition ();
  test_fold ();
  Printf.printf "✓ All tests passed\n"
