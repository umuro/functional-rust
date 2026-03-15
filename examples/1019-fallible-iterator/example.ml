(* 1019: Fallible Iterator
   An iterator that can fail: each item is Result<T, E>.
   In OCaml we represent this as a list of Results (or a Seq of Results),
   and provide take_while_ok (stops at first error) and process_all (keeps both). *)

let parse_int s =
  match int_of_string_opt (String.trim s) with
  | Some n -> Ok n
  | None   -> Error (Printf.sprintf "bad line: %s" s)

(* Create a "fallible iterator" as a list of Results *)
let line_parser lines =
  List.map parse_int lines

(* Approach 1: Stop at first error — like take_while_ok *)
let take_while_ok items =
  let rec loop acc = function
    | [] -> Ok (List.rev acc)
    | Ok v :: rest -> loop (v :: acc) rest
    | Error e :: _ -> Error e
  in
  loop [] items

(* Approach 2: Process all, partition into (successes, failures) *)
let process_all items =
  List.fold_left (fun (oks, errs) item ->
    match item with
    | Ok v    -> (v :: oks, errs)
    | Error e -> (oks, e :: errs)
  ) ([], []) items
  |> (fun (oks, errs) -> (List.rev oks, List.rev errs))

(* Approach 3: collect (same as take_while_ok but using fold) *)
let collect_results items =
  List.fold_right (fun item acc ->
    match acc with
    | Error _ -> acc  (* pass error through *)
    | Ok ns ->
      (match item with
       | Ok v    -> Ok (v :: ns)
       | Error e -> Error e)
  ) items (Ok [])

let () =
  (* All valid *)
  let parser = line_parser ["1"; "2"; "3"] in
  assert (take_while_ok parser = Ok [1; 2; 3]);

  (* Stop at first error *)
  let parser2 = line_parser ["1"; "abc"; "3"] in
  (match take_while_ok parser2 with
   | Error msg -> assert (String.length msg > 0)
   | _ -> assert false);

  (* Empty iterator *)
  assert (take_while_ok [] = Ok []);

  (* process_all: collect both sides *)
  let parser3 = line_parser ["1"; "abc"; "3"; "def"] in
  let (oks, errs) = process_all parser3 in
  assert (oks = [1; 3]);
  assert (List.length errs = 2);

  let (oks2, errs2) = process_all (line_parser ["10"; "20"]) in
  assert (oks2 = [10; 20]);
  assert (errs2 = []);

  (* collect_results mirrors take_while_ok *)
  assert (collect_results (line_parser ["1"; "2"; "3"]) = Ok [1; 2; 3]);

  (* Lazy: only look at first item — error not yet encountered *)
  let first = List.hd (line_parser ["1"; "bad"; "3"]) in
  assert (first = Ok 1);

  Printf.printf "Fallible iterator tests passed\n"
