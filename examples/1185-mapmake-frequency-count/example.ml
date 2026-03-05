(* Map.Make — Frequency Count *)
(* Count occurrences using Map *)

module CharMap = Map.Make(Char)

let char_freq s =
  String.fold_left (fun m c ->
    let n = match CharMap.find_opt c m with Some n -> n | None -> 0 in
    CharMap.add c (n + 1) m
  ) CharMap.empty s

let freq = char_freq "mississippi"
let sorted = CharMap.bindings freq
  |> List.sort (fun (_,a) (_,b) -> compare b a)

let () = List.iter (fun (c, n) ->
  Printf.printf "'%c': %d\n" c n
) sorted
