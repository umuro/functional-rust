(* Example 066: Sequence Monadic *)
(* Turn a list of monadic values into a monadic list *)

(* Approach 1: sequence for Option *)
let sequence_option xs =
  List.fold_right (fun x acc ->
    match x, acc with
    | Some y, Some ys -> Some (y :: ys)
    | _ -> None
  ) xs (Some [])

(* Approach 2: sequence for Result *)
let sequence_result xs =
  List.fold_right (fun x acc ->
    match x, acc with
    | Ok y, Ok ys -> Ok (y :: ys)
    | Error e, _ | _, Error e -> Error e
  ) xs (Ok [])

(* Approach 3: Generic sequence via bind *)
let sequence_generic ~bind ~return_ xs =
  List.fold_right (fun mx acc ->
    bind mx (fun x ->
    bind acc (fun xs ->
    return_ (x :: xs)))
  ) xs (return_ [])

let opt_bind m f = match m with None -> None | Some x -> f x
let opt_return x = Some x

let res_bind m f = match m with Error e -> Error e | Ok x -> f x
let res_return x = Ok x

let () =
  (* Option sequence *)
  assert (sequence_option [Some 1; Some 2; Some 3] = Some [1; 2; 3]);
  assert (sequence_option [Some 1; None; Some 3] = None);
  assert (sequence_option [] = Some []);

  (* Result sequence *)
  assert (sequence_result [Ok 1; Ok 2; Ok 3] = Ok [1; 2; 3]);
  assert (sequence_result [Ok 1; Error "e"; Ok 3] = Error "e");

  (* Generic sequence *)
  let opt_seq = sequence_generic ~bind:opt_bind ~return_:opt_return in
  assert (opt_seq [Some 1; Some 2] = Some [1; 2]);
  assert (opt_seq [Some 1; None] = None);

  let res_seq = sequence_generic ~bind:res_bind ~return_:res_return in
  assert (res_seq [Ok 1; Ok 2] = Ok [1; 2]);

  Printf.printf "✓ All tests passed\n"
