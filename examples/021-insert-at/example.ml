(* Insert At *)
(* OCaml 99 Problems #21 *)

let insert_at x k lst =
  let rec aux acc n = function
    | [] -> List.rev (x :: acc)
    | t when n <= 0 -> List.rev_append acc (x :: t)
    | h :: t -> aux (h :: acc) (n - 1) t
  in
  aux [] (k - 1) lst

(* Tests *)
let () =
  assert (insert_at 99 3 [1; 2; 3; 4; 5] = [1; 2; 99; 3; 4; 5]);
  assert (insert_at 0 1 [1; 2; 3] = [0; 1; 2; 3]);
  assert (insert_at 4 4 [1; 2; 3] = [1; 2; 3; 4]);
  assert (insert_at 4 10 [1; 2; 3] = [1; 2; 3; 4]);
  assert (insert_at 1 1 [] = [1]);
  print_endline "✓ OCaml tests passed"
