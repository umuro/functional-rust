(* Split List *)
(* OCaml 99 Problems #17 *)

let split_list lst n =
  let rec aux acc n = function
    | [] -> (List.rev acc, [])
    | rest when n = 0 -> (List.rev acc, rest)
    | x :: t -> aux (x :: acc) (n - 1) t
  in
  aux [] n lst

(* Tests *)
let () =
  assert (split_list [1; 2; 3; 4; 5] 2 = ([1; 2], [3; 4; 5]));
  assert (split_list [1; 2; 3] 0 = ([], [1; 2; 3]));
  assert (split_list [1; 2; 3] 3 = ([1; 2; 3], []));
  assert (split_list [1; 2; 3] 10 = ([1; 2; 3], []));
  assert (split_list [] 2 = ([], []));
  print_endline "✓ OCaml tests passed"
