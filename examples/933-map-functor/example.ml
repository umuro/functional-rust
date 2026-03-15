module StringMap = Map.Make(String)

let word_lengths words =
  List.fold_left
    (fun acc w -> StringMap.add w (String.length w) acc)
    StringMap.empty
    words

let () =
  let words = ["ocaml"; "rust"; "haskell"; "erlang"; "go"] in
  let m = word_lengths words in
  assert (StringMap.find_opt "rust" m = Some 4);
  assert (StringMap.find_opt "missing" m = None);
  let long = StringMap.filter (fun _ v -> v > 4) m in
  assert (StringMap.cardinal long = 3);
  let doubled = StringMap.map (fun v -> v * 2) m in
  assert (StringMap.find "rust" doubled = 8);
  print_endline "All assertions passed."
