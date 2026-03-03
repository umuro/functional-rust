(* fold_right — Structural Recursion *)

(* Implementation 1: Direct recursive fold_right *)
let rec fold_right f lst acc =
  match lst with
  | []     -> acc
  | h :: t -> f h (fold_right f t acc)

(* Implementation 2: Using List.fold_right from stdlib *)
let fold_right_stdlib f lst acc = List.fold_right f lst acc

(* Classic uses *)
let sum  lst = fold_right ( + ) lst 0
let prod lst = fold_right ( * ) lst 1
let cat  lst = fold_right ( ^ ) lst ""
let copy lst = fold_right (fun h t -> h :: t) lst []

(* Tests *)
let () =
  assert (sum [1; 2; 3; 4; 5] = 15);
  assert (prod [1; 2; 3; 4; 5] = 120);
  assert (cat ["a"; "b"; "c"] = "abc");
  assert (copy [1; 2; 3] = [1; 2; 3]);
  assert (sum [] = 0);
  assert (prod [] = 1);
  assert (cat [] = "");
  Printf.printf "All fold_right tests passed!\n"
