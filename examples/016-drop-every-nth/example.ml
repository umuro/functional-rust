(* Drop Every Nth *)
(* OCaml 99 Problems #16 *)

let drop_every_nth lst n =
  let rec aux acc count = function
    | [] -> List.rev acc
    | x :: t -> if count = n then aux acc 1 t else aux (x :: acc) (count + 1) t
  in
  aux [] 1 lst

(* Tests *)
let () =
  assert (drop_every_nth [1; 2; 3; 4; 5; 6; 7; 8; 9] 3 = [1; 2; 4; 5; 7; 8]);
  assert (drop_every_nth [1; 2; 3; 4; 5; 6] 2 = [1; 3; 5]);
  assert (drop_every_nth [1; 2; 3] 1 = []);
  assert (drop_every_nth [1; 2; 3] 10 = [1; 2; 3]);
  assert (drop_every_nth [] 3 = []);
  print_endline "✓ OCaml tests passed"
