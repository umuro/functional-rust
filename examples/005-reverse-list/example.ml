(* Reverse a list *)

(* Tail-recursive with accumulator *)
let rev list =
  let rec aux acc = function
    | [] -> acc
    | h :: t -> aux (h :: acc) t
  in
  aux [] list

(* Tests *)
let () =
  assert (rev [] = []);
  assert (rev [1] = [1]);
  assert (rev [1; 2; 3; 4] = [4; 3; 2; 1]);
  assert (rev ["a"; "b"; "c"] = ["c"; "b"; "a"]);
  print_endline "✓ OCaml tests passed"
