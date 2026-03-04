let to_sorted_chars s =
  s |> String.lowercase_ascii
    |> String.to_seq |> List.of_seq
    |> List.sort Char.compare

let is_anagram s1 s2 =
  let s1' = String.lowercase_ascii s1 in
  let s2' = String.lowercase_ascii s2 in
  s1' <> s2' && to_sorted_chars s1 = to_sorted_chars s2

let find_anagrams word candidates =
  List.filter (is_anagram word) candidates

let () =
  assert (is_anagram "listen" "silent" = true);
  assert (is_anagram "hello" "world" = false);
  assert (is_anagram "listen" "listen" = false);
  let results = find_anagrams "listen" ["enlists";"google";"inlets";"silent"] in
  assert (results = ["inlets";"silent"]);
  print_endline "ok"
