(* Example 064: Traverse with Option *)
(* Apply a function returning Option to each element, collecting results *)

(* Approach 1: Manual traverse for lists *)
let rec traverse_option f = function
  | [] -> Some []
  | x :: xs ->
    match f x with
    | None -> None
    | Some y ->
      match traverse_option f xs with
      | None -> None
      | Some ys -> Some (y :: ys)

(* Approach 2: Using fold *)
let traverse_option_fold f xs =
  List.fold_right (fun x acc ->
    match f x, acc with
    | Some y, Some ys -> Some (y :: ys)
    | _ -> None
  ) xs (Some [])

(* Approach 3: Sequence — traverse with identity *)
let sequence_option xs = traverse_option Fun.id xs

(* Test functions *)
let safe_div10 x = if x = 0 then None else Some (10 / x)
let parse_int s = int_of_string_opt s

let () =
  (* All succeed *)
  assert (traverse_option safe_div10 [2; 5; 1] = Some [5; 2; 10]);

  (* One fails — whole thing fails *)
  assert (traverse_option safe_div10 [2; 0; 1] = None);

  (* Empty list *)
  assert (traverse_option safe_div10 [] = Some []);

  (* Fold version *)
  assert (traverse_option_fold safe_div10 [2; 5; 1] = Some [5; 2; 10]);
  assert (traverse_option_fold safe_div10 [2; 0; 1] = None);

  (* Parse strings *)
  assert (traverse_option parse_int ["1"; "2"; "3"] = Some [1; 2; 3]);
  assert (traverse_option parse_int ["1"; "bad"; "3"] = None);

  (* Sequence *)
  assert (sequence_option [Some 1; Some 2; Some 3] = Some [1; 2; 3]);
  assert (sequence_option [Some 1; None; Some 3] = None);

  Printf.printf "✓ All tests passed\n"
