(* List — Zip and Unzip *)
(* Combine and split parallel lists *)

let rec zip l1 l2 = match (l1, l2) with
  | ([], _) | (_, []) -> []
  | (x :: xs, y :: ys) -> (x, y) :: zip xs ys

let unzip lst =
  List.fold_right (fun (a, b) (la, lb) -> (a :: la, b :: lb)) lst ([], [])

let names = ["Alice"; "Bob"; "Carol"]
let scores = [95; 87; 92]
let paired = zip names scores
let () = List.iter (fun (n, s) -> Printf.printf "%s: %d\n" n s) paired

let (ns, ss) = unzip paired
let () = Printf.printf "Names: %s\n" (String.concat ", " ns)
