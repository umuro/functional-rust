(* 005: Reverse a List *)

(* Approach 1: Built-in *)
let reverse_builtin lst = List.rev lst

(* Approach 2: Naive recursive — O(n^2) *)
let rec reverse_naive = function
  | [] -> []
  | x :: xs -> reverse_naive xs @ [x]

(* Approach 3: Tail-recursive with accumulator — O(n) *)
let reverse_acc lst =
  let rec aux acc = function
    | [] -> acc
    | x :: xs -> aux (x :: acc) xs
  in
  aux [] lst

(* Tests *)
let () =
  assert (reverse_builtin [1; 2; 3; 4; 5] = [5; 4; 3; 2; 1]);
  assert (reverse_builtin [] = []);
  assert (reverse_naive [1; 2; 3] = [3; 2; 1]);
  assert (reverse_acc [1; 2; 3; 4; 5] = [5; 4; 3; 2; 1]);
  assert (reverse_acc [] = []);
  assert (reverse_acc [42] = [42]);
  Printf.printf "✓ All tests passed\n"
