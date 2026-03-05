(* List — Interleave Two Lists *)
(* Alternate elements from two lists *)

let rec interleave l1 l2 = match (l1, l2) with
  | ([], l) | (l, []) -> l
  | (x :: xs, y :: ys) -> x :: y :: interleave xs ys

let transpose matrix =
  match matrix with
  | [] -> []
  | first :: _ ->
    List.mapi (fun i _ ->
      List.map (fun row -> List.nth row i) matrix
    ) first

let r = interleave [1;3;5] [2;4;6]
let () = List.iter (fun x -> Printf.printf "%d " x) r;
  print_newline ()

let t = transpose [[1;2;3]; [4;5;6]; [7;8;9]]
let () = List.iter (fun row ->
  List.iter (fun x -> Printf.printf "%d " x) row;
  print_newline ()
) t
