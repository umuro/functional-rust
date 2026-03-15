(* 091: Zip and Unzip *)

(* Approach 1: Zip two lists *)
let rec zip l1 l2 =
  match l1, l2 with
  | [], _ | _, [] -> []
  | x :: xs, y :: ys -> (x, y) :: zip xs ys

(* Approach 2: Unzip pairs *)
let unzip pairs =
  List.fold_right (fun (a, b) (la, lb) -> (a :: la, b :: lb)) pairs ([], [])

(* Approach 3: Zip with function *)
let rec zip_with f l1 l2 =
  match l1, l2 with
  | [], _ | _, [] -> []
  | x :: xs, y :: ys -> f x y :: zip_with f xs ys

(* Tests *)
let () =
  assert (zip [1; 2; 3] ["a"; "b"; "c"] = [(1, "a"); (2, "b"); (3, "c")]);
  assert (zip [1; 2] [10; 20; 30] = [(1, 10); (2, 20)]);
  assert (unzip [(1, "a"); (2, "b")] = ([1; 2], ["a"; "b"]));
  assert (zip_with ( + ) [1; 2; 3] [10; 20; 30] = [11; 22; 33]);
  Printf.printf "✓ All tests passed\n"
