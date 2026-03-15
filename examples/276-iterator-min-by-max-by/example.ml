(* 276: Custom comparison min_by and max_by.
   OCaml: provide a comparison function directly to find extremes.
   Useful for floats (partial ordering) and multi-key comparisons. *)

(* min_by: find minimum using a custom comparator *)
let min_by cmp = function
  | []      -> None
  | x :: xs ->
    Some (List.fold_left (fun acc el ->
      if cmp el acc < 0 then el else acc) x xs)

(* max_by: find maximum using a custom comparator *)
let max_by cmp = function
  | []      -> None
  | x :: xs ->
    Some (List.fold_left (fun acc el ->
      if cmp el acc > 0 then el else acc) x xs)

let () =
  (* min/max of floats — Float.compare handles NaN gracefully *)
  let floats = [3.0; 1.0; 2.0] in
  Printf.printf "min float = %s\n"
    (Option.fold ~none:"None"
      ~some:(Printf.sprintf "%.1f")
      (min_by Float.compare floats));

  (* max using reversed comparator (min_by reversed = max) *)
  let nums = [1;5;3;2;4] in
  let max_via_rev = min_by (fun a b -> compare b a) nums in
  Printf.printf "max via reversed cmp = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int max_via_rev);

  (* Multi-key: first by length, then lexicographically *)
  let words = ["bb"; "aa"; "c"] in
  let min_word = min_by (fun a b ->
    let by_len = compare (String.length a) (String.length b) in
    if by_len <> 0 then by_len else String.compare a b
  ) words in
  Printf.printf "min by (len, lex) = %s\n"
    (Option.value ~default:"?" min_word);

  (* Max by absolute value *)
  let signed = [-10; 3; -7; 1] in
  let max_abs = max_by (fun a b -> compare (abs a) (abs b)) signed in
  Printf.printf "max by |x| = %s\n"
    (Option.fold ~none:"None" ~some:string_of_int max_abs);

  (* Compare with standard List.sort approach *)
  let sorted_by_len = List.sort (fun a b ->
    compare (String.length a) (String.length b)
  ) words in
  Printf.printf "sorted by length: [%s]\n" (String.concat ";" sorted_by_len)
