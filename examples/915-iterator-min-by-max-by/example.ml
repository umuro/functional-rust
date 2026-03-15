(* 915: Custom comparison min_by / max_by

   OCaml's List module provides List.sort, but for min/max with custom
   comparators we fold with an explicit compare function. *)

(* Generic min_by: find element minimising the comparison *)
let min_by cmp lst =
  match lst with
  | [] -> None
  | hd :: tl ->
    Some (List.fold_left (fun acc x -> if cmp x acc < 0 then x else acc) hd tl)

let max_by cmp lst =
  match lst with
  | [] -> None
  | hd :: tl ->
    Some (List.fold_left (fun acc x -> if cmp x acc > 0 then x else acc) hd tl)

(* min_by_key: derive a comparison key from each element *)
let min_by_key f lst =
  match lst with
  | [] -> None
  | hd :: tl ->
    Some (List.fold_left
      (fun acc x -> if compare (f x) (f acc) < 0 then x else acc)
      hd tl)

let max_by_key f lst =
  match lst with
  | [] -> None
  | hd :: tl ->
    Some (List.fold_left
      (fun acc x -> if compare (f x) (f acc) > 0 then x else acc)
      hd tl)

let () =
  (* min/max of floats using Float.compare *)
  let floats = [3.0; 1.0; 2.0] in
  assert (min_by Float.compare floats = Some 1.0);
  assert (max_by Float.compare floats = Some 3.0);

  (* max_by with reversed comparator *)
  let nums = [1; 5; 3; 2; 4] in
  (* "min with reversed cmp" = max of original *)
  assert (min_by (fun a b -> compare b a) nums = Some 5);

  (* min_by with multi-key: length first, then lexicographic *)
  let words = ["bb"; "aa"; "c"] in
  let multi_key s = (String.length s, s) in
  assert (min_by_key multi_key words = Some "c");

  (* max_by_key: longest word *)
  let ws = ["cat"; "elephant"; "ox"] in
  assert (max_by_key String.length ws = Some "elephant");

  (* min/max_by on records via key extraction *)
  let people = [("Alice", 30); ("Bob", 25); ("Carol", 35)] in
  let youngest = min_by_key snd people in
  let oldest   = max_by_key snd people in
  assert (youngest = Some ("Bob", 25));
  assert (oldest   = Some ("Carol", 35));

  print_endline "915-iterator-min-by-max-by: all tests passed"
