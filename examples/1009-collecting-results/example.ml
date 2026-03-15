(* 1009: Collecting Results
   Turn a list of Results into a single Result<list> — stopping at first error.
   This mirrors Rust's Iterator<Item=Result<T,E>> -> Result<Vec<T>,E> via collect().
   In OCaml we use List.fold_left or a recursive approach. *)

let parse_int s =
  match int_of_string_opt (String.trim s) with
  | Some n -> Ok n
  | None   -> Error (Printf.sprintf "bad: %s" s)

(* Approach 1: fold that short-circuits at first error *)
let parse_all inputs =
  List.fold_left (fun acc s ->
    match acc with
    | Error _ as e -> e   (* already failed, pass error through *)
    | Ok ns ->
      (match parse_int s with
       | Ok n    -> Ok (ns @ [n])
       | Error e -> Error e)
  ) (Ok []) inputs

(* Approach 2: Recursive — tail-call variant *)
let parse_all_rec inputs =
  let rec loop acc = function
    | [] -> Ok (List.rev acc)
    | s :: rest ->
      (match parse_int s with
       | Error e -> Error e
       | Ok n    -> loop (n :: acc) rest)
  in
  loop [] inputs

(* Approach 3: Using the standard library helper *)
(* In OCaml 4.14+ there is no direct List.result_all,
   but we can write a clean version using Result.bind *)
let parse_all_bind inputs =
  List.fold_right (fun s acc ->
    Result.bind (parse_int s) (fun n ->
      Result.map (fun ns -> n :: ns) acc)
  ) inputs (Ok [])

let () =
  assert (parse_all ["1"; "2"; "3"] = Ok [1; 2; 3]);
  assert (parse_all ["1"; "abc"; "3"] = Error "bad: abc");
  assert (parse_all [] = Ok []);
  assert (parse_all ["42"] = Ok [42]);
  assert (Result.is_error (parse_all ["xyz"]));

  (* All approaches should agree *)
  let inputs = ["10"; "20"; "30"] in
  assert (parse_all inputs = parse_all_rec inputs);
  assert (parse_all inputs = parse_all_bind inputs);

  let bad = ["10"; "x"] in
  assert (Result.is_error (parse_all_rec bad));

  Printf.printf "parse_all [1;2;3]: %s\n"
    (match parse_all ["1"; "2"; "3"] with
     | Ok ns -> "[" ^ String.concat "; " (List.map string_of_int ns) ^ "]"
     | Error e -> e)
