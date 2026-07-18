(* Combinations *)
(* OCaml 99 Problems #26 *)

let rec combinations k lst =
  match k, lst with
  | 0, _ -> [[]]
  | _, [] -> []
  | k, x :: xs -> List.map (fun c -> x :: c) (combinations (k - 1) xs) @ combinations k xs

(* Tests *)
let () =
  assert (combinations 2 [1; 2; 3] = [[1; 2]; [1; 3]; [2; 3]]);
  assert (combinations 0 [1; 2; 3] = [[]]);
  assert (combinations 5 [1; 2; 3] = []);
  assert (combinations 3 [1; 2; 3] = [[1; 2; 3]]);
  assert (List.length (combinations 2 [1; 2; 3; 4; 5]) = 10);
  print_endline "✓ OCaml tests passed"
