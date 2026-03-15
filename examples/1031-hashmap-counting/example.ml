(* 1031: Count Frequencies
   The classic frequency-counting pattern.
   OCaml: Hashtbl with or-insert-0 pattern, or a pure fold using Map. *)

(* Count character frequencies using Hashtbl *)
let char_frequency () =
  let text = "abracadabra" in
  let counts : (char, int) Hashtbl.t = Hashtbl.create 16 in
  String.iter (fun ch ->
    let n = match Hashtbl.find_opt counts ch with Some n -> n | None -> 0 in
    Hashtbl.replace counts ch (n + 1)
  ) text;
  assert (Hashtbl.find counts 'a' = 5);
  assert (Hashtbl.find counts 'b' = 2);
  assert (Hashtbl.find counts 'r' = 2);
  assert (Hashtbl.find counts 'c' = 1);
  assert (Hashtbl.find counts 'd' = 1)

(* Word frequency *)
let word_frequency () =
  let words = ["the"; "cat"; "sat"; "on"; "the"; "mat"; "the"; "cat"] in
  let counts : (string, int) Hashtbl.t = Hashtbl.create 8 in
  List.iter (fun word ->
    let n = match Hashtbl.find_opt counts word with Some n -> n | None -> 0 in
    Hashtbl.replace counts word (n + 1)
  ) words;
  assert (Hashtbl.find counts "the" = 3);
  assert (Hashtbl.find counts "cat" = 2);
  assert (Hashtbl.find counts "sat" = 1)

(* Find the most frequent element — pure functional with fold *)
let most_frequent lst =
  (* Build frequency map via fold *)
  let counts =
    List.fold_left (fun acc x ->
      let n = match List.assoc_opt x acc with Some n -> n | None -> 0 in
      (x, n + 1) :: List.filter (fun (k, _) -> k <> x) acc
    ) [] lst
  in
  match counts with
  | [] -> None
  | _  ->
    Some (List.fold_left (fun (bk, bv) (k, v) ->
      if v > bv then (k, v) else (bk, bv)
    ) (List.hd counts) (List.tl counts))

(* Functional counting using Map — purely immutable *)
module IntMap = Map.Make(Int)

let functional_counting data =
  List.fold_left (fun acc x ->
    let n = match IntMap.find_opt x acc with Some n -> n | None -> 0 in
    IntMap.add x (n + 1) acc
  ) IntMap.empty data

let () =
  char_frequency ();
  word_frequency ();

  (match most_frequent [1; 2; 3; 2; 1; 2; 3; 2; 2] with
   | Some (most, count) ->
     assert (most = 2);
     assert (count = 5)
   | None -> assert false);

  let counts = functional_counting [1; 1; 2; 3; 3; 3] in
  assert (IntMap.find 1 counts = 2);
  assert (IntMap.find 3 counts = 3);

  (* Empty input *)
  let empty_counts = functional_counting [] in
  assert (IntMap.is_empty empty_counts);

  Printf.printf "Frequency counting tests passed\n"
