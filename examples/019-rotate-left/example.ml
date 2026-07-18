(* Rotate Left *)
(* OCaml 99 Problems #19 *)

let split_at n lst =
  let rec aux acc n = function
    | [] -> (List.rev acc, [])
    | rest when n = 0 -> (List.rev acc, rest)
    | x :: t -> aux (x :: acc) (n - 1) t
  in
  aux [] n lst

let rotate_left lst n =
  match lst with
  | [] -> []
  | _ ->
    let len = List.length lst in
    let n = ((n mod len) + len) mod len in
    let left, right = split_at n lst in
    right @ left

(* Tests *)
let () =
  assert (rotate_left [1; 2; 3; 4; 5] 2 = [3; 4; 5; 1; 2]);
  assert (rotate_left [1; 2; 3] 0 = [1; 2; 3]);
  assert (rotate_left [1; 2; 3] 3 = [1; 2; 3]);
  assert (rotate_left [1; 2; 3] 7 = [2; 3; 1]);
  assert (rotate_left [1; 2; 3; 4; 5] (-2) = [4; 5; 1; 2; 3]);
  assert (rotate_left [] 3 = []);
  print_endline "✓ OCaml tests passed"
