(* Find the k-th element (1-indexed) *)

let rec at k = function
  | [] -> None
  | h :: t -> if k = 1 then Some h else at (k - 1) t

(* Tests *)
let () =
  assert (at 3 [1; 2; 3; 4; 5] = Some 3);
  assert (at 1 [1; 2; 3] = Some 1);
  assert (at 10 [1; 2; 3] = None);
  print_endline "✓ OCaml tests passed"
