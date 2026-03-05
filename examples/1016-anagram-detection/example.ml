(* Anagram Detection *)
(* Sorting characters to detect anagrams *)

let to_sorted_list s =
  let chars = List.init (String.length s) (String.get s) in
  List.sort Char.compare chars

let anagrams target candidates =
  let target_lc = String.lowercase_ascii target in
  let target_sorted = to_sorted_list target_lc in
  List.filter (fun c ->
    let lc = String.lowercase_ascii c in
    to_sorted_list lc = target_sorted && lc <> target_lc
  ) candidates
