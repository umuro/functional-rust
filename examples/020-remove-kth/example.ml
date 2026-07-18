(* Remove Kth *)
(* OCaml 99 Problems #20 *)

let remove_kth lst k =
  let rec aux acc n = function
    | [] -> None
    | x :: t ->
      if n = k then Some (x, List.rev_append acc t)
      else aux (x :: acc) (n + 1) t
  in
  aux [] 1 lst

(* Tests *)
let () =
  assert (remove_kth [1; 2; 3; 4; 5] 3 = Some (3, [1; 2; 4; 5]));
  assert (remove_kth [1; 2; 3] 1 = Some (1, [2; 3]));
  assert (remove_kth [1; 2; 3] 3 = Some (3, [1; 2]));
  assert (remove_kth [1; 2; 3] 0 = None);
  assert (remove_kth [1; 2; 3] 10 = None);
  assert (remove_kth [] 1 = None);
  print_endline "✓ OCaml tests passed"
