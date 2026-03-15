(* 002: List Operations *)
(* Core list operations: head, tail, length, append, reverse *)

(* Approach 1: Standard library *)
let head_of lst = List.hd lst
let tail_of lst = List.tl lst
let length_of lst = List.length lst
let append_lists a b = a @ b
let reverse_list lst = List.rev lst

(* Approach 2: Pattern matching — safer *)
let safe_head = function
  | [] -> None
  | x :: _ -> Some x

let safe_tail = function
  | [] -> None
  | _ :: xs -> Some xs

let rec my_length = function
  | [] -> 0
  | _ :: xs -> 1 + my_length xs

let rec my_append a b =
  match a with
  | [] -> b
  | x :: xs -> x :: my_append xs b

let rec my_reverse = function
  | [] -> []
  | x :: xs -> my_reverse xs @ [x]

(* Approach 3: Tail-recursive reverse *)
let rev_tr lst =
  let rec aux acc = function
    | [] -> acc
    | x :: xs -> aux (x :: acc) xs
  in
  aux [] lst

(* Tests *)
let () =
  assert (head_of [1; 2; 3] = 1);
  assert (tail_of [1; 2; 3] = [2; 3]);
  assert (length_of [1; 2; 3] = 3);
  assert (append_lists [1; 2] [3; 4] = [1; 2; 3; 4]);
  assert (reverse_list [1; 2; 3] = [3; 2; 1]);
  assert (safe_head [] = None);
  assert (safe_head [42] = Some 42);
  assert (safe_tail [] = None);
  assert (my_length [1; 2; 3; 4] = 4);
  assert (my_append [1] [2; 3] = [1; 2; 3]);
  assert (my_reverse [1; 2; 3] = [3; 2; 1]);
  assert (rev_tr [1; 2; 3] = [3; 2; 1]);
  Printf.printf "✓ All tests passed\n"
