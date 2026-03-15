(* 1031: Count Frequencies *)
(* Count occurrences of each element *)

module CharMap = Map.Make(Char)
module StringMap = Map.Make(String)

(* Approach 1: Character frequency counting *)
let char_frequency () =
  let text = "abracadabra" in
  let counts = String.fold_left (fun acc ch ->
    let n = match CharMap.find_opt ch acc with
      | Some n -> n
      | None -> 0
    in
    CharMap.add ch (n + 1) acc
  ) CharMap.empty text in
  assert (CharMap.find 'a' counts = 5);
  assert (CharMap.find 'b' counts = 2);
  assert (CharMap.find 'r' counts = 2);
  assert (CharMap.find 'c' counts = 1);
  assert (CharMap.find 'd' counts = 1)

(* Approach 2: Word frequency *)
let word_frequency () =
  let words = ["the"; "cat"; "sat"; "on"; "the"; "mat"; "the"; "cat"] in
  let counts = List.fold_left (fun acc w ->
    let n = match StringMap.find_opt w acc with
      | Some n -> n
      | None -> 0
    in
    StringMap.add w (n + 1) acc
  ) StringMap.empty words in
  assert (StringMap.find "the" counts = 3);
  assert (StringMap.find "cat" counts = 2);
  assert (StringMap.find "sat" counts = 1)

(* Approach 3: Most frequent element *)
let most_frequent () =
  let items = [1; 2; 3; 2; 1; 2; 3; 2; 2] in
  module IntMap = Map.Make(Int) in
  let counts = List.fold_left (fun acc x ->
    let n = match IntMap.find_opt x acc with
      | Some n -> n
      | None -> 0
    in
    IntMap.add x (n + 1) acc
  ) IntMap.empty items in
  let (most, count) = IntMap.fold (fun k v (mk, mv) ->
    if v > mv then (k, v) else (mk, mv)
  ) counts (0, 0) in
  assert (most = 2);
  assert (count = 5)

let () =
  char_frequency ();
  word_frequency ();
  most_frequent ();
  Printf.printf "✓ All tests passed\n"
