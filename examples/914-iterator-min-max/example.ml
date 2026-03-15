(* 275. Finding extremes: min() and max() - OCaml *)

let list_min = function [] -> None | lst -> Some (List.fold_left min max_int lst)
let list_max = function [] -> None | lst -> Some (List.fold_left max min_int lst)

let () =
  let nums = [3; 1; 4; 1; 5; 9; 2; 6; 5; 3; 5] in
  Printf.printf "Min: %s\n" (match list_min nums with Some n -> string_of_int n | None -> "None");
  Printf.printf "Max: %s\n" (match list_max nums with Some n -> string_of_int n | None -> "None");
  Printf.printf "Min of []: %s\n" (match list_min [] with None -> "None" | Some n -> string_of_int n);

  let words = ["banana"; "apple"; "fig"; "kiwi"; "cherry"] in
  let shortest = List.fold_left (fun acc w ->
    if String.length w < String.length acc then w else acc
  ) (List.hd words) (List.tl words) in
  Printf.printf "Shortest: %s\n" shortest;

  let longest = List.fold_left (fun acc w ->
    if String.length w > String.length acc then w else acc
  ) (List.hd words) (List.tl words) in
  Printf.printf "Longest: %s\n" longest
