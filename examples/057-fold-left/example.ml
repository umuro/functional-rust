(* fold_left — Tail-Recursive Accumulator *)

(* Implementation 1: Direct tail-recursive fold_left *)
let rec fold_left f acc = function
  | []     -> acc
  | h :: t -> fold_left f (f acc h) t

(* Implementation 2: Using List.fold_left from stdlib *)
let fold_left_stdlib f acc lst = List.fold_left f acc lst

(* Classic uses *)
let sum     lst = fold_left ( + ) 0 lst
let product lst = fold_left ( * ) 1 lst
let maximum lst = fold_left max (List.hd lst) (List.tl lst)
let reverse lst = fold_left (fun acc x -> x :: acc) [] lst

(* Tests *)
let () =
  let nums = [3; 1; 4; 1; 5; 9; 2; 6] in
  assert (sum nums = 31);
  assert (product nums = 6480);
  assert (maximum nums = 9);
  assert (reverse [1; 2; 3] = [3; 2; 1]);
  assert (sum [] = 0);
  assert (reverse [] = []);
  Printf.printf "All fold_left tests passed!\n"
