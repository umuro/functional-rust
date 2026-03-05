module CMap = Map.Make(Char)

(* Count letter frequencies in a single string *)
let letter_freq s =
  String.fold_left (fun m c ->
    let c = Char.lowercase_ascii c in
    if c >= 'a' && c <= 'z' then
      CMap.update c (function None -> Some 1 | Some n -> Some (n+1)) m
    else m
  ) CMap.empty s

(* Merge two frequency maps by summing counts *)
let merge_maps =
  CMap.union (fun _ a b -> Some (a + b))

(* Map-reduce: map each text to freq, then fold-merge *)
let parallel_frequency texts =
  texts
  |> List.map letter_freq
  |> List.fold_left merge_maps CMap.empty

let () =
  let texts = ["Hello World"; "Functional Programming"; "OCaml is Great"] in
  let freq = parallel_frequency texts in
  CMap.iter (Printf.printf "%c:%d ") freq;
  print_newline ();
  (* Basic assertions *)
  assert (CMap.find 'o' freq = 5);
  assert (CMap.mem 'h' freq);
  print_endline "ok"
