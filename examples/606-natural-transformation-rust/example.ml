(* Natural transformations in OCaml *)

(* η : Option -> List (nat transform) *)
let opt_to_list = function None -> [] | Some x -> [x]

(* η : List -> Option (head) *)
let list_to_opt = function [] -> None | x :: _ -> Some x

(* Naturality condition:
   list_map f ∘ opt_to_list = opt_to_list ∘ Option.map f *)
let naturality_opt_to_list f opt =
  List.map f (opt_to_list opt) = opt_to_list (Option.map f opt)

let () =
  let f x = x * 2 in
  Printf.printf "opt->list naturality (Some 5): %b\n" (naturality_opt_to_list f (Some 5));
  Printf.printf "opt->list naturality (None): %b\n"   (naturality_opt_to_list f None);
  Printf.printf "opt_to_list Some 42 = %s\n"
    (match opt_to_list (Some 42) with [x]->"["^string_of_int x^"]" | _->"[]");
  Printf.printf "list_to_opt [1;2;3] = %s\n"
    (match list_to_opt [1;2;3] with Some x->string_of_int x | None->"none")
