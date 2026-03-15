(* 1010: Partition Results
   Separate Ok and Error values from a list of Results.
   We use List.partition_map (OCaml 4.12+) or a fold. *)

let parse_int s =
  match int_of_string_opt (String.trim s) with
  | Some n -> Ok n
  | None   -> Error (Printf.sprintf "bad: %s" s)

(* Approach 1: List.partition_map — clean and direct *)
let partition_results inputs =
  List.partition_map (fun s ->
    match parse_int s with
    | Ok n    -> Left n
    | Error e -> Right e
  ) inputs

(* Approach 2: fold into two accumulators *)
let partition_fold inputs =
  let (oks, errs) =
    List.fold_left (fun (oks, errs) s ->
      match parse_int s with
      | Ok n    -> (n :: oks, errs)
      | Error e -> (oks, e :: errs)
    ) ([], []) inputs
  in
  (List.rev oks, List.rev errs)

(* Approach 3: filter_map for just one side *)
let only_successes inputs =
  List.filter_map (fun s ->
    match parse_int s with Ok n -> Some n | Error _ -> None
  ) inputs

let only_errors inputs =
  List.filter_map (fun s ->
    match parse_int s with Ok _ -> None | Error e -> Some e
  ) inputs

let () =
  let (oks, errs) = partition_results ["1"; "abc"; "3"; "def"; "5"] in
  assert (oks = [1; 3; 5]);
  assert (errs = ["bad: abc"; "bad: def"]);

  let (oks2, errs2) = partition_results ["1"; "2"; "3"] in
  assert (oks2 = [1; 2; 3]);
  assert (errs2 = []);

  let (oks3, errs3) = partition_results ["a"; "b"; "c"] in
  assert (oks3 = []);
  assert (List.length errs3 = 3);

  (* fold matches partition_map *)
  let (f_oks, f_errs) = partition_fold ["1"; "abc"; "3"] in
  let (p_oks, p_errs) = partition_results ["1"; "abc"; "3"] in
  assert (f_oks = p_oks && f_errs = p_errs);

  assert (only_successes ["1"; "x"; "3"] = [1; 3]);
  assert (only_errors ["1"; "x"; "3"] = ["bad: x"]);

  let (e_oks, e_errs) = partition_results [] in
  assert (e_oks = [] && e_errs = []);

  Printf.printf "oks: [%s]  errs: [%s]\n"
    (String.concat "; " (List.map string_of_int oks))
    (String.concat "; " errs)
